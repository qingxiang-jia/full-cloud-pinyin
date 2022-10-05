/*
 * SPDX-FileCopyrightText: 2021~2021 CSSlayer <wengxt@gmail.com>
 *
 * SPDX-License-Identifier: BSD-3-Clause
 *
 */
#include "quwei.h"
#include <fcitx-utils/i18n.h>
#include <fcitx-utils/key.h>
#include <fcitx-utils/keysymgen.h>
#include <fcitx-utils/log.h>
#include <fcitx-utils/utf8.h>
#include <fcitx/candidatelist.h>
#include <fcitx/inputpanel.h>
#include <fcitx/instance.h>
#include <fcitx/userinterfacemanager.h>
#include <memory>
#include <punctuation_public.h>
#include <quickphrase_public.h>
#include <utility>

namespace {

static const std::array<fcitx::Key, 10> selectionKeys = {
    fcitx::Key{FcitxKey_1}, fcitx::Key{FcitxKey_2}, fcitx::Key{FcitxKey_3},
    fcitx::Key{FcitxKey_4}, fcitx::Key{FcitxKey_5}, fcitx::Key{FcitxKey_6},
    fcitx::Key{FcitxKey_7}, fcitx::Key{FcitxKey_8}, fcitx::Key{FcitxKey_9},
    fcitx::Key{FcitxKey_0},
};

static const std::vector<fcitx::Key> candListSelectKey = {
    fcitx::Key{FcitxKey_1}, fcitx::Key{FcitxKey_2}, fcitx::Key{FcitxKey_3},
    fcitx::Key{FcitxKey_4}, fcitx::Key{FcitxKey_5}, fcitx::Key{FcitxKey_6},
    fcitx::Key{FcitxKey_7}, fcitx::Key{FcitxKey_8}, fcitx::Key{FcitxKey_9},
    fcitx::Key{FcitxKey_0},
};

static const std::array<fcitx::Key, 1> prevPageKeys = {
    fcitx::Key{FcitxKey_minus}
};

static const std::array<fcitx::Key, 1> nextPageKeys = {
    fcitx::Key{FcitxKey_equal}
};

class QuweiCandidate : public fcitx::CandidateWord {
public:
    QuweiCandidate(QuweiEngine *engine, ::rust::String text, int matched_len)
        : engine_(engine), matched_len(matched_len) {
        setText(fcitx::Text(std::move(text.c_str())));
    }

    void select(fcitx::InputContext *inputContext) const override {
        auto state = inputContext->propertyFor(engine_->factory());
        auto preedit = state->getPreedit();

        if (preedit.length() == matched_len) {
            inputContext->commitString(text().toString());
            state->reset();
        } else if (preedit.length() > matched_len) {
            // Partial match
            inputContext->commitString(text().toString());
            // Update preedit
            state->preeditRemoveFront(matched_len);
            // Query and update candidates for updated preedit and update UI
            state->getUpdateCandidatesRefreshUI(false);
        } else {
            FCITX_INFO() << "Matched length > preedit length, which doesn't make sense.";
        }
    }

private:
    QuweiEngine *engine_;
    unsigned long matched_len;
};

} // namespace

void QuweiState::keyEvent(fcitx::KeyEvent &event) {
    if (auto candidateList = ic_->inputPanel().candidateList()) {
        int idx = event.key().keyListIndex(selectionKeys);
        
        // Select a candidate by keying in 0-9
        if (idx >= 0 && idx < candidateList->size()) {
            event.accept();
            candidateList->candidate(idx).select(ic_);
            return;
        }

        // Select a candidate by space key
        if (event.key().check(FcitxKey_space)) {
            event.accept();
            auto idx = candidateList->cursorIndex();
            candidateList->candidate(idx).select(ic_);
            return;
        }

        // Go to the next page by keying in the next page keys
        if (event.key().checkKeyList(nextPageKeys)) {
            if (auto *pageable = candidateList->toPageable();
                pageable && pageable->hasNext()) {
                pageable->next();
                ic_->updateUserInterface(
                    fcitx::UserInterfaceComponent::InputPanel);
            }
            return event.filterAndAccept();
        }

        // Go to the previous page by previous page keys
        if (event.key().checkKeyList(prevPageKeys)) {
            if (auto *pageable = candidateList->toPageable();
                pageable && pageable->hasPrev()) {
                event.accept();
                pageable->prev();
                ic_->updateUserInterface(
                    fcitx::UserInterfaceComponent::InputPanel);
            }
            return event.filterAndAccept();
        }

        // Go to the next candidate by ->
        if (auto *cursorMovable = candidateList->toCursorMovable()) {
            if (event.key().check(FcitxKey_Right)) {
                cursorMovable->nextCandidate();
                ic_->updateUserInterface(
                    fcitx::UserInterfaceComponent::InputPanel);
                return event.filterAndAccept();
            }
            if (event.key().check(FcitxKey_Left)) {
                cursorMovable->prevCandidate();
                ic_->updateUserInterface(
                    fcitx::UserInterfaceComponent::InputPanel);
                return event.filterAndAccept();
            }
        }

        // Remove one character from buffer
        if (event.key().check(FcitxKey_BackSpace)) {
            buffer_.backspace();
            getUpdateCandidatesRefreshUI(false);
            return event.filterAndAccept();
        }

        // Commit buffer as is (i.e., not Chinese)
        if (event.key().check(FcitxKey_Return)) {
            ic_->commitString(buffer_.userInput());
            reset();
            return event.filterAndAccept();
        }

        // Terminate this input session
        if (event.key().check(FcitxKey_Escape)) {
            reset();
            return event.filterAndAccept();
        }
    }

    // If buffer is empty and has keyed in a letter, show lookup table
    if (event.key().isLAZ() || event.key().isUAZ()) {
        // Append this key into the buffer
        buffer_.type(event.key().sym());

        std::string preedit = buffer_.userInput();

        // Use preedit to query pinyin candidates, update candidates, and update UI
        getUpdateCandidatesRefreshUI(false);
        return event.filterAndAccept();
    }

    return;
}

void QuweiState::setCandidateList(bool append) {
    if (candidates.empty()) {
        return;
    }

    if (append) {

    } else {
        // Store candidates in candidate list
        auto candidateList = std::make_unique<fcitx::CommonCandidateList>();
        candidateList->setSelectionKey(candListSelectKey);
        candidateList->setCursorPositionAfterPaging(                fcitx::CursorPositionAfterPaging::ResetToFirst);
        candidateList->setPageSize(engine_->instance()->globalConfig().defaultPageSize());

        for (unsigned long i = 0; i < candidates.size(); i++) {
            std::unique_ptr<fcitx::CandidateWord> candidate = std::make_unique<QuweiCandidate>(engine_, candidates[i].word, candidates[i].len);
            candidateList->append(std::move(candidate));
        }

        candidates.clear();

        candidateList->setGlobalCursorIndex(0);
        ic_->inputPanel().setCandidateList(std::move(candidateList));
    }
}

void QuweiState::updateUI(bool append) {
    auto &inputPanel = ic_->inputPanel();
    inputPanel.reset();
    setCandidateList(append);
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

void QuweiState::getUpdateCandidatesRefreshUI(bool append) {
    std::string preedit = buffer_.userInput();
    candidates = engine_->rustPinyin_->queryCandidates(preedit);
    updateUI(append);
}

std::string QuweiState::getPreedit() {
    auto preedit = buffer_.userInput();
    return preedit;
}

void QuweiState::preeditRemoveFront(int lenToRemove) {
    auto oldPreedit = buffer_.userInput();
    auto newPreedit = oldPreedit.substr(lenToRemove, oldPreedit.length() - lenToRemove);
    buffer_.clear();
    buffer_.type(newPreedit);
}

QuweiEngine::QuweiEngine(fcitx::Instance *instance)
    : rustPinyin_(new RustPinyin()), instance_(instance), factory_([this](fcitx::InputContext &ic) {
          return new QuweiState(this, &ic);
      }) {
    instance->inputContextManager().registerProperty("quweiState", &factory_);
}

void QuweiEngine::activate(const fcitx::InputMethodEntry &entry,
                           fcitx::InputContextEvent &event) {
    FCITX_UNUSED(entry);
    auto *inputContext = event.inputContext();
    // Request full width.
    fullwidth();
    chttrans();
    for (const auto *actionName : {"chttrans", "punctuation", "fullwidth"}) {
        if (auto *action =
                instance_->userInterfaceManager().lookupAction(actionName)) {
            inputContext->statusArea().addAction(
                fcitx::StatusGroup::InputMethod, action);
        }
    }
}

void QuweiEngine::keyEvent(const fcitx::InputMethodEntry &entry,
                           fcitx::KeyEvent &keyEvent) {
    FCITX_UNUSED(entry);
    if (keyEvent.isRelease() || keyEvent.key().states()) {
        return;
    }
    // FCITX_INFO() << keyEvent.key() << " isRelease=" << keyEvent.isRelease();
    auto ic = keyEvent.inputContext();
    auto *state = ic->propertyFor(&factory_);
    state->keyEvent(keyEvent);
}

void QuweiEngine::reset(const fcitx::InputMethodEntry &,
                        fcitx::InputContextEvent &event) {
    auto *state = event.inputContext()->propertyFor(&factory_);
    state->reset();
}

RustPinyin::RustPinyin() {
    auto boxedFcp = fcp::init();
    this->fcp = boxedFcp.into_raw();
}

::rust::Vec<::fcp::CandidateWord> RustPinyin::queryCandidates(std::string preedit) {
    auto rustCand = this->fcp->query_candidates(preedit);
    return rustCand;
}

FCITX_ADDON_FACTORY(QuweiEngineFactory);
