/*
 * SPDX-FileCopyrightText: 2021~2021 CSSlayer <wengxt@gmail.com>
 *
 * SPDX-License-Identifier: BSD-3-Clause
 *
 */
#include "quwei.h"
#include <chrono>
#include <fcitx-utils/eventdispatcher.h>
#include <fcitx-utils/i18n.h>
#include <fcitx-utils/key.h>
#include <fcitx-utils/keysymgen.h>
#include <fcitx-utils/log.h>
#include <fcitx-utils/utf8.h>
#include <fcitx/candidatelist.h>
#include <fcitx/inputpanel.h>
#include <fcitx/instance.h>
#include <fcitx/text.h>
#include <fcitx/userinterfacemanager.h>
#include <functional>
#include <future>
#include <iostream>
#include <memory>
#include <new>
#include <punctuation_public.h>
#include <quickphrase_public.h>
#include <thread>
#include <utility>

namespace {

template <class F, typename... Args> void call_async(F&& lambda)
{
    // Modified from https://stackoverflow.com/a/56834117/1509779
    auto futptr = std::make_shared<std::future<void>>();
    *futptr = std::async(std::launch::async, [futptr, lambda]() { lambda(); });
}

class QuweiCandidate : public fcitx::CandidateWord {
public:
    QuweiCandidate(::rust::String text) { setText(fcitx::Text(std::move(text.c_str()))); }
    QuweiCandidate(fcitx::Text text) { setText(text); }
    void select(fcitx::InputContext*) const {};
};

} // namespace

QuweiEngine::QuweiEngine(fcitx::Instance* instance)
    : rustPinyin_(new RustPinyin())
    , instance_(instance)
{
    dispatcher = std::make_unique<fcitx::EventDispatcher>();
    dispatcher->attach(&instance->eventLoop());
}

void QuweiEngine::activate(const fcitx::InputMethodEntry& entry, fcitx::InputContextEvent& event)
{
    FCITX_UNUSED(entry);
    auto* inputContext = event.inputContext();
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
        preeditRemoveFirstN(matchedLen);
        // Query and update candidates for updated preedit and update UI
        getCandidatesAndUpdateAsync();
    } else {
        FCITX_INFO() << "Matched length > preedit length, which doesn't make sense.";
    }
}

void QuweiEngine::keyEvent(const fcitx::InputMethodEntry& entry, fcitx::KeyEvent& keyEvent)
{
    FCITX_UNUSED(entry);
    if (keyEvent.isRelease() || keyEvent.key().states()) {
        return;
    }
    if (ic_->inputPanel().candidateList() == nullptr) {
        ic_->inputPanel().setCandidateList(makeCandidateList());
    } // Surprisingly, if you set it in activate(), it is still null when keyuEvent is called.

    fcitx::KeySym key = keyEvent.key().sym();
    auto candidateList = ic_->inputPanel().candidateList();

    if (candidateList->size() > 0) {
        if (FcitxKey_0 <= key && key <= FcitxKey_9) {
            auto idx = key - FcitxKey_1;
            // Select a candidate by keying in 0-9
            if (idx >= 0 && idx < candidateList->size()) {
                select(idx);
                return keyEvent.filterAndAccept();
            }
        }

        // Select a candidate by space key
        if (key == FcitxKey_space) {
            auto idx = candidateList->cursorIndex();
            select(idx);
            return keyEvent.filterAndAccept();
        }

        // Go to the next page by keying in the next page keys
        if (key == FcitxKey_equal) {
            if (auto* pageable = candidateList->toPageable(); pageable) {
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
            if (auto* pageable = candidateList->toPageable(); pageable && pageable->hasPrev()) {
                pageable->prev();
                ic_->updateUserInterface(fcitx::UserInterfaceComponent::InputPanel);
            }
            return keyEvent.filterAndAccept();
        }

        // Go to the next candidate by ->
        if (auto* cursorMovable = candidateList->toCursorMovable()) {
            if (key == FcitxKey_Right) {
                cursorMovable->nextCandidate();
                ic_->updateUserInterface(fcitx::UserInterfaceComponent::InputPanel);
                return keyEvent.filterAndAccept();
            }
            if (key == FcitxKey_Left) {
                cursorMovable->prevCandidate();
                ic_->updateUserInterface(fcitx::UserInterfaceComponent::InputPanel);
                return keyEvent.filterAndAccept();
            }
        }

        // Remove one character from buffer
        if (key == FcitxKey_BackSpace) {
            buffer_.backspace();
            if (buffer_.size() == 0) {
                reset();
            } else {
                setPreedit(buffer_.userInput());
                setDummyCandidates();
                getCandidatesAndUpdateAsync();
            }
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
    if ((FcitxKey_A <= key && key <= FcitxKey_Z) || (FcitxKey_a <= key && key <= FcitxKey_z)) {
        // Append this key into the buffer
        buffer_.type(key);
        setPreedit(buffer_.userInput());
        setDummyCandidates();

        // Use preedit to query pinyin candidates, update candidates, and update UI
        getCandidatesAndUpdateAsync();
        return keyEvent.filterAndAccept();
    }

    return;
}

std::unique_ptr<fcitx::CommonCandidateList> QuweiEngine::makeCandidateList()
{
    auto candidateList = std::make_unique<fcitx::CommonCandidateList>();
    candidateList->setLabels(std::vector<std::string> { "1. ", "2. ", "3. ", "4. ", "5. ", "6. ", "7. ", "8. ", "9. ", "10. " });
    candidateList->setCursorPositionAfterPaging(fcitx::CursorPositionAfterPaging::ResetToFirst);
    candidateList->setPageSize(instance_->globalConfig().defaultPageSize());
    return candidateList;
}

void QuweiEngine::setDummyCandidates()
{
    auto candidateList = std::dynamic_pointer_cast<fcitx::CommonCandidateList>(ic_->inputPanel().candidateList());

    lens.clear();
    candidateList->clear();
    for (int i = 0; i < 5; i++) {
        std::unique_ptr<fcitx::CandidateWord> candidateWord = std::make_unique<QuweiCandidate>(fcitx::Text("☁"));
        candidateList->append(std::move(candidateWord));
        lens.push_back(0);
    }
    candidateList->setGlobalCursorIndex(0);

    ic_->updateUserInterface(fcitx::UserInterfaceComponent::InputPanel);
}

void QuweiEngine::setPreedit(std::string preedit)
{
    if (ic_->capabilityFlags().test(fcitx::CapabilityFlag::Preedit)) {
        fcitx::Text text(preedit, fcitx::TextFormatFlag::HighLight);
        ic_->inputPanel().setClientPreedit(text);
    } else {
        fcitx::Text text(preedit);
        ic_->inputPanel().setPreedit(text);
    }
    ic_->updatePreedit();
    ic_->updateUserInterface(fcitx::UserInterfaceComponent::InputPanel);
}

void QuweiEngine::setCandidates(::rust::Vec<::fcp::CandidateWord> candidates, bool append)
{
    if (candidates.empty()) {
        return;
    }

    auto candidateList = std::dynamic_pointer_cast<fcitx::CommonCandidateList>(ic_->inputPanel().candidateList());

    if (!append) {
        lens.clear();
        candidateList->clear();
    }
    for (auto& candidate : candidates) {
        std::unique_ptr<fcitx::CandidateWord> candidateWord = std::make_unique<QuweiCandidate>(candidate.word);
        candidateList->append(std::move(candidateWord));
        lens.push_back(candidate.len);
    }
    candidates.clear();

    if (!append) {
        candidateList->setGlobalCursorIndex(0);
    }

    ic_->updateUserInterface(fcitx::UserInterfaceComponent::InputPanel);
}

void QuweiEngine::getUpdateCandidatesRefreshUI(bool append)
{
    std::string preedit = buffer_.userInput();

    auto candidates = rustPinyin_->fcp->query_candidates(preedit);

    setCandidates(candidates, append);
}

void QuweiEngine::getCandidatesAndUpdateAsync(bool append)
{
    call_async([this, append]() { dispatcher->schedule([this, append]() { getUpdateCandidatesRefreshUI(append); }); });
}

void QuweiEngine::preeditRemoveFirstN(int lenToRemove)
{
    auto oldPreedit = buffer_.userInput();
    auto newPreedit = oldPreedit.substr(lenToRemove, oldPreedit.length() - lenToRemove);
    buffer_.clear();
    buffer_.type(newPreedit);
}

void QuweiEngine::reset()
{
    buffer_.clear();
    ic_->inputPanel().reset();
    setPreedit(buffer_.userInput());
}

void QuweiEngine::reset(const fcitx::InputMethodEntry&, fcitx::InputContextEvent& event)
{
    FCITX_UNUSED(event);
    reset();
}

RustPinyin::RustPinyin()
{
    auto boxedFcp = fcp::init();
    this->fcp = boxedFcp.into_raw();
}

FCITX_ADDON_FACTORY(QuweiEngineFactory);
