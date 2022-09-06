/*
 * SPDX-FileCopyrightText: 2021~2021 CSSlayer <wengxt@gmail.com>
 *
 * SPDX-License-Identifier: BSD-3-Clause
 *
 */
#include "quwei.h"
#include <fcitx-utils/i18n.h>
#include <fcitx-utils/utf8.h>
#include <fcitx/candidatelist.h>
#include <fcitx/inputpanel.h>
#include <fcitx/instance.h>
#include <fcitx/userinterfacemanager.h>
#include <punctuation_public.h>
#include <quickphrase_public.h>
#include <utility>

namespace {

// Template to help resolve iconv parameter issue on BSD.
template <class T>
struct function_traits;

// partial specialization for function pointer
template <class R, class... Args>
struct function_traits<R (*)(Args...)> {
    using result_type = R;
    using argument_types = std::tuple<Args...>;
};

template <class T>
using second_argument_type = typename std::tuple_element<
    1, typename function_traits<T>::argument_types>::type;

static const std::array<fcitx::Key, 10> selectionKeys = {
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
        if (idx >= 0 && idx < candidateList->size()) {
            event.accept();
            candidateList->candidate(idx).select(ic_);
            return;
        }
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
    }

    if (buffer_.empty()) {
        if (!event.key().isDigit()) {
            // if it gonna commit something
            auto c = fcitx::Key::keySymToUnicode(event.key().sym());
            if (!c) {
                return;
            }
            std::string punc, puncAfter;
            // skip key pad
            if (c && !event.key().isKeyPad()) {
                std::tie(punc, puncAfter) =
                    engine_->punctuation()
                        ->call<fcitx::IPunctuation::pushPunctuationV2>("zh_CN",
                                                                       ic_, c);
            }
            if (event.key().check(FcitxKey_semicolon) &&
                engine_->quickphrase()) {
                auto keyString = fcitx::utf8::UCS4ToUTF8(c);
                // s is punc or key
                auto output = !punc.empty() ? (punc + puncAfter) : keyString;
                // alt is key or empty
                auto altOutput = !punc.empty() ? keyString : "";
                // if no punc: key -> key (s = key, alt = empty)
                // if there's punc: key -> punc, return -> key (s = punc, alt =
                // key)
                std::string text;
                engine_->quickphrase()->call<fcitx::IQuickPhrase::trigger>(
                    ic_, text, "", output, altOutput,
                    fcitx::Key(FcitxKey_semicolon));
                event.filterAndAccept();
                return;
            }
            if (!punc.empty()) {
                event.filterAndAccept();
                ic_->commitString(punc + puncAfter);
                if (size_t length = fcitx::utf8::lengthValidated(puncAfter);
                    length != 0 && length != fcitx::utf8::INVALID_LENGTH) {
                    for (size_t i = 0; i < length; i++) {
                        ic_->forwardKey(fcitx::Key(FcitxKey_Left));
                    }
                }
            }
            return;
        }
    } else {
        if (event.key().check(FcitxKey_BackSpace)) {
            buffer_.backspace();
            updateUI();
            return event.filterAndAccept();
        }
        if (event.key().check(FcitxKey_Return)) {
            ic_->commitString(buffer_.userInput());
            reset();
            return event.filterAndAccept();
        }
        if (event.key().check(FcitxKey_Escape)) {
            reset();
            return event.filterAndAccept();
        }
        if (!event.key().isDigit()) {
            return event.filterAndAccept();
        }
    }

    buffer_.type(event.key().sym());
    updateUI();
    return event.filterAndAccept();
}

void QuweiState::setCode(int code) {
    if (code < 0 || code > 999) {
        return;
    }
    buffer_.clear();
    auto codeStr = std::to_string(code);
    while (codeStr.size() < 3) {
        codeStr = "0" + codeStr;
    }
    buffer_.type(std::to_string(code));
    updateUI();
}

void QuweiState::updateUI() {
    auto &inputPanel = ic_->inputPanel();
    inputPanel.reset();
    if (buffer_.size() == 3) {
        inputPanel.setCandidateList(std::make_unique<fcitx::CommonCandidateList>());
    }
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
    : instance_(instance), factory_([this](fcitx::InputContext &ic) {
          return new QuweiState(this, &ic);
      }) {
    conv_ = iconv_open("UTF-8", "GB18030");
    if (conv_ == reinterpret_cast<iconv_t>(-1)) {
        throw std::runtime_error("Failed to create converter");
    }
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
    std::string tmp_s;
    tmp_s.reserve(len);

    for (int i = 0; i < len; ++i) {
        tmp_s += alphanum[rand() % (sizeof(alphanum) - 1)];
    }
    
    return tmp_s;
}

std::vector<std::string> getCandidates(std::string preedit) {
    int candidadteCount = 5;
    
    std::vector<std::string> candidates = {};
    
    for (int i = 0; i < candidadteCount; i++) {
        candidates.push_back(gen_random_str(preedit.length()));
    }
    return candidates;
}

FCITX_ADDON_FACTORY(QuweiEngineFactory);
