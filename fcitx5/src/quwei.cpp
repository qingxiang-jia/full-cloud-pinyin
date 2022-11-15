/*
 * SPDX-FileCopyrightText: 2021~2021 CSSlayer <wengxt@gmail.com>
 *
 * SPDX-License-Identifier: BSD-3-Clause
 *
 */
#include "quwei.h"
#include <fcitx-utils/eventdispatcher.h>
#include <fcitx-utils/i18n.h>
#include <fcitx-utils/key.h>
#include <fcitx-utils/keysymgen.h>
#include <fcitx-utils/log.h>
#include <fcitx-utils/utf8.h>
#include <fcitx/candidatelist.h>
#include <fcitx/inputpanel.h>
#include <fcitx/instance.h>
#include <fcitx/userinterfacemanager.h>
#include <chrono>
#include <future>
#include <iostream>
#include <new>
#include <thread>
#include <functional>
#include <memory>
#include <punctuation_public.h>
#include <quickphrase_public.h>
#include <utility>

namespace {

template <class F, typename... Args>
void call_async(F&& lambda) {
    // Modified from https://stackoverflow.com/a/56834117/1509779
    auto futptr = std::make_shared<std::future<void>>();
    *futptr = std::async(std::launch::async, [futptr, lambda]() {
        lambda();
    });
}

class QuweiCandidate : public fcitx::CandidateWord {
public:
    QuweiCandidate(::rust::String text) { setText(fcitx::Text(std::move(text.c_str()))); }
    void select(fcitx::InputContext*) const {};
};

} // namespace

QuweiEngine::QuweiEngine(fcitx::Instance *instance)
    : rustPinyin_(new RustPinyin()), instance_(instance) {
        dispatcher = std::make_unique<fcitx::EventDispatcher>();
        dispatcher->attach(&instance->eventLoop());
    }

void QuweiEngine::activate(const fcitx::InputMethodEntry &entry,
                           fcitx::InputContextEvent &event) {
    FCITX_UNUSED(entry);
    auto *inputContext = event.inputContext();
    ic_ = inputContext;
}

void QuweiEngine::select(const int idx)
{
    auto preedit = buffer_.userInput();
    auto matchedLen = lens[idx];
    auto candidate = ic_->inputPanel().candidateList()->candidate(idx).text();

    if (preedit.length() == matchedLen) {
        ic_->commitString(candidate.toStringForCommit());
        reset();
    } else if (preedit.length() > matchedLen) {
        // Partial match
        ic_->commitString(candidate.toStringForCommit());
        // Update preedit
        preeditRemoveFront(matchedLen);
        // Query and update candidates for updated preedit and update UI
        getCandidatesAndUpdateAsync();
    } else {
        FCITX_INFO() << "Matched length > preedit length, which doesn't make sense.";
    }
}

void QuweiEngine::keyEvent(const fcitx::InputMethodEntry &entry,
                           fcitx::KeyEvent &keyEvent) {
    FCITX_UNUSED(entry);
    if (keyEvent.isRelease() || keyEvent.key().states()) {
        return;
    }
    // FCITX_INFO() << keyEvent.key() << " isRelease=" << keyEvent.isRelease();
    ic_ = keyEvent.inputContext();

    if (auto candidateList = ic_->inputPanel().candidateList()) {
        fcitx::KeySym key = keyEvent.key().sym();

        if (FcitxKey_0 <= key && key <= FcitxKey_9) {
            auto idx = key - FcitxKey_1;
            // Select a candidate by keying in 0-9
            if (idx >= 0 && idx < candidateList->size()) {
                keyEvent.accept();
                select(idx);
                return;
            }
        }
        
        // Select a candidate by space key
        if (key == FcitxKey_space) {
            keyEvent.accept();
            auto idx = candidateList->cursorIndex();
            select(idx);
            return;
        }

        // Go to the next page by keying in the next page keys
        if (key == FcitxKey_equal) {
            if (auto *pageable = candidateList->toPageable();
                pageable) {
                if (pageable->hasNext()) {
                    pageable->next();
                    ic_->updateUserInterface(fcitx::UserInterfaceComponent::InputPanel);
                } else {
                    // Request more candidates
                    getCandidatesAndUpdateAsync(true);
                }
            }
            return keyEvent.filterAndAccept();
        }

        // Go to the previous page by previous page keys
        if (key == FcitxKey_minus) {
            if (auto *pageable = candidateList->toPageable();
                pageable && pageable->hasPrev()) {
                keyEvent.accept();
                pageable->prev();
                ic_->updateUserInterface(
                    fcitx::UserInterfaceComponent::InputPanel);
            }
            return keyEvent.filterAndAccept();
        }

        // Go to the next candidate by ->
        if (auto *cursorMovable = candidateList->toCursorMovable()) {
            if (key == FcitxKey_Right) {
                cursorMovable->nextCandidate();
                ic_->updateUserInterface(
                    fcitx::UserInterfaceComponent::InputPanel);
                return keyEvent.filterAndAccept();
            }
            if (key == FcitxKey_Left) {
                cursorMovable->prevCandidate();
                ic_->updateUserInterface(
                    fcitx::UserInterfaceComponent::InputPanel);
                return keyEvent.filterAndAccept();
            }
        }

        // Remove one character from buffer
        if (key == FcitxKey_BackSpace) {
            buffer_.backspace();
            getCandidatesAndUpdateAsync();
            return keyEvent.filterAndAccept();
        }

        // Commit buffer as is (i.e., not Chinese)
        if (key == FcitxKey_Return) {
            ic_->commitString(buffer_.userInput());
            reset();
            return keyEvent.filterAndAccept();
        }

        // Terminate this input session
        if (key == FcitxKey_Escape) {
            reset();
            return keyEvent.filterAndAccept();
        }
    }

    // If buffer is empty and has keyed in a letter, show lookup table
    if (keyEvent.key().isLAZ() || keyEvent.key().isUAZ()) {
        // Append this key into the buffer
        buffer_.type(keyEvent.key().sym());

        // Use preedit to query pinyin candidates, update candidates, and update UI
        getCandidatesAndUpdateAsync();
        return keyEvent.filterAndAccept();
    }

    return;
}

void QuweiEngine::updateUI() {
    auto &inputPanel = ic_->inputPanel();
    inputPanel.reset();
    if (ic_->capabilityFlags().test(fcitx::CapabilityFlag::Preedit)) {
        fcitx::Text preedit(buffer_.userInput(),
                            fcitx::TextFormatFlag::HighLight);
        inputPanel.setClientPreedit(preedit);
    } else {
        fcitx::Text preedit(buffer_.userInput());
        inputPanel.setPreedit(preedit);
    }
    ic_->updateUserInterface(fcitx::UserInterfaceComponent::InputPanel);
    ic_->updatePreedit();
}

void QuweiEngine::getUpdateCandidatesRefreshUI(bool append) {
    auto &inputPanel = ic_->inputPanel();
    std::string preedit = buffer_.userInput();
    
    auto candidates = rustPinyin_->fcp->query_candidates(preedit);

    if (!candidates.empty()) {
        // Store candidates in candidate list
        auto candidateList = std::make_unique<fcitx::CommonCandidateList>();
        candidateList->setLabels(std::vector<std::string>{"1. ", "2. ", "3. ", "4. ", "5. ", "6. ", "7. ", "8. ", "9. ", "10. "});
        candidateList->setCursorPositionAfterPaging(fcitx::CursorPositionAfterPaging::ResetToFirst);
        candidateList->setPageSize(instance()->globalConfig().defaultPageSize());

        lens.clear();
        for (auto& candidate : candidates) {
            std::unique_ptr<fcitx::CandidateWord> candidateWord = std::make_unique<QuweiCandidate>(candidate.word);
            candidateList->append(std::move(candidateWord));
            lens.push_back(candidate.len);
        }

        candidates.clear();

        if (!append) {
            candidateList->setGlobalCursorIndex(0);
        } else {
            // Get current page number
            auto pageable = ic_->inputPanel().candidateList()->toPageable();
            auto currPage = pageable->currentPage();
            candidateList->setPage(currPage);
        }
        ic_->inputPanel().setCandidateList(std::move(candidateList));
    }

    if (ic_->capabilityFlags().test(fcitx::CapabilityFlag::Preedit)) {
        fcitx::Text preedit(buffer_.userInput(),
                            fcitx::TextFormatFlag::HighLight);
        inputPanel.setClientPreedit(preedit);
    } else {
        fcitx::Text preedit(buffer_.userInput());
        inputPanel.setPreedit(preedit);
    }
    ic_->updateUserInterface(fcitx::UserInterfaceComponent::InputPanel, true);
    ic_->updatePreedit();
}

void QuweiEngine::getCandidatesAndUpdateAsync(bool append) {
    call_async([this, append](){ dispatcher->schedule([this, append](){ getUpdateCandidatesRefreshUI(append); }); });
}

std::string QuweiEngine::getPreedit() {
    auto preedit = buffer_.userInput();
    return preedit;
}

void QuweiEngine::preeditRemoveFront(int lenToRemove) {
    auto oldPreedit = buffer_.userInput();
    auto newPreedit = oldPreedit.substr(lenToRemove, oldPreedit.length() - lenToRemove);
    buffer_.clear();
    buffer_.type(newPreedit);
}

void QuweiEngine::reset() {
    buffer_.clear();
    updateUI();
}

void QuweiEngine::reset(const fcitx::InputMethodEntry &,
                        fcitx::InputContextEvent &event) {
    FCITX_UNUSED(event);
    buffer_.clear();
    updateUI();
}

RustPinyin::RustPinyin() {
    auto boxedFcp = fcp::init();
    this->fcp = boxedFcp.into_raw();
}

FCITX_ADDON_FACTORY(QuweiEngineFactory);
