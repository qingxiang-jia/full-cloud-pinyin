/*
 * SPDX-FileCopyrightText: 2021~2021 CSSlayer <wengxt@gmail.com>
 *
 * SPDX-License-Identifier: BSD-3-Clause
 *
 */
#include "quwei.h"
#include "../../fcpinyin/ffi.rs.h"
#include <fcitx-utils/i18n.h>
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

class QuweiCandidate : public fcitx::CandidateWord {
public:
    QuweiCandidate(QuweiEngine *engine, std::string text)
        : engine_(engine) {
        setText(fcitx::Text(std::move(text)));
    }

    void select(fcitx::InputContext *inputContext) const override {
        inputContext->commitString(text().toString());
        auto state = inputContext->propertyFor(engine_->factory());
        state->reset();
    }

private:
    QuweiEngine *engine_;
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

        // Go to the previous page by keying in the default previous page key
        if (event.key().checkKeyList(
                engine_->instance()->globalConfig().defaultPrevPage())) {
            if (auto *pageable = candidateList->toPageable();
                pageable && pageable->hasPrev()) {
                event.accept();
                pageable->prev();
                ic_->updateUserInterface(
                    fcitx::UserInterfaceComponent::InputPanel);
            }
            return event.filterAndAccept();
        }

        // Go to the next page by keying in the default next page key
        if (event.key().checkKeyList(
                engine_->instance()->globalConfig().defaultNextPage())) {
            if (auto *pageable = candidateList->toPageable();
                pageable && pageable->hasNext()) {
                pageable->next();
                ic_->updateUserInterface(
                    fcitx::UserInterfaceComponent::InputPanel);
            }
            return event.filterAndAccept();
        }

        // Remove one character from buffer
        if (event.key().check(FcitxKey_BackSpace)) {
            buffer_.backspace();

            std::string preedit = buffer_.userInput();
            candidates = engine_->dummyPinyin_->getCandidates(preedit);

            updateUI();
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

        // Use preedit to query the dummy
        candidates = engine_->dummyPinyin_->getCandidates(preedit);

        updateUI();
        return event.filterAndAccept();
    }

    return;
}

void QuweiState::setCandidateList() {
    if (candidates.empty()) {
        return;
    }

    // Store candidates in candidate list
    auto candidateList = std::make_unique<fcitx::CommonCandidateList>();
    candidateList->setSelectionKey(candListSelectKey);
    candidateList->setCursorPositionAfterPaging(                fcitx::CursorPositionAfterPaging::ResetToFirst);
    candidateList->setPageSize(engine_->instance()->globalConfig().defaultPageSize());

    for (unsigned long i = 0; i < candidates.size(); i++) {
        std::unique_ptr<fcitx::CandidateWord> candidate = std::make_unique<QuweiCandidate>(engine_, candidates[i]);
        candidateList->append(std::move(candidate));
    }

    candidates.clear();

    candidateList->setGlobalCursorIndex(0);
    ic_->inputPanel().setCandidateList(std::move(candidateList));
}

void QuweiState::updateUI() {
    auto &inputPanel = ic_->inputPanel();
    inputPanel.reset();
    setCandidateList();
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

QuweiEngine::QuweiEngine(fcitx::Instance *instance)
    : dummyPinyin_(new DummyPinyin()), instance_(instance), factory_([this](fcitx::InputContext &ic) {
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

DummyPinyin::DummyPinyin() {}

std::string gen_random_str(const int len) {
    // https://stackoverflow.com/questions/440133/how-do-i-create-a-random-alpha-numeric-string-in-c
    
    static const char alphanum[] =
        "0123456789"
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        "abcdefghijklmnopqrstuvwxyz";
    std::string str;
    str.reserve(len);

    for (int i = 0; i < len; ++i) {
        str += alphanum[rand() % (sizeof(alphanum) - 1)];
    }
    
    return str;
}

std::vector<std::string> DummyPinyin::getCandidates(std::string preedit) {
    int candidadteCount = 25;
    
    std::vector<std::string> candidates = {};
    
    for (int i = 0; i < candidadteCount; i++) {
        candidates.push_back(gen_random_str(preedit.length()));
    }
    return candidates;
}

FCITX_ADDON_FACTORY(QuweiEngineFactory);
