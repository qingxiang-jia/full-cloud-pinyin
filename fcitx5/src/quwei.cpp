/*
 * SPDX-FileCopyrightText: 2021~2021 CSSlayer <wengxt@gmail.com>
 *
 * SPDX-License-Identifier: BSD-3-Clause
 *
 */
#include "quwei.h"
#include "src/include/rust.h"
#include <chrono>
#include <cstddef>
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
#include <vector>

QuweiEngine* engine;

/* BEGIN UI */
extern "C" void set_loading() { engine->setLoading(); }

extern "C" void set_candidates(int16_t** candidates, size_t cnt)
{
    std::vector<std::string> candidatesToSet;
    for (size_t i = 0; i < cnt; i++) {
        std::string candidate((char*)candidates[i]);
        candidatesToSet.push_back(std::move(candidate));
    }
    engine->setCandidates(candidatesToSet);
}

extern "C" void append_candidates(int16_t** candidates, size_t cnt)
{
    std::vector<std::string> candidatesToSet;
    for (size_t i = 0; i < cnt; i++) {
        std::string candidate((char*)candidates[i]);
        candidatesToSet.push_back(std::move(candidate));
    }
    engine->appendCandidates(candidatesToSet);
}

extern "C" void clear_candidates()
{
    engine->clearCandidates();
}

extern "C" void set_preedit(char* preedit)
{
    std::string preeditStr(preedit);
    engine->setPreedit(std::move(preeditStr));
}
/* END UI */

/* BEGIN TABLE */
extern "C" bool can_page_up() { return engine->canPageUp(); }

extern "C" void page_up() { engine->nextPage(); }

extern "C" void page_down() { engine->prevPage(); }

extern "C" void prev() { engine->prevCanddiate(); }

extern "C" void next() { engine->nextCandidate(); }
/* END TABLE */

/* BEGIN ENGINE */
extern "C" void commit(uint16_t idx) { engine->commitCandidateByIndex(idx); }

extern "C" void commit_candidate_by_fixed_key() { engine->commitCandidateByFixedKey(); }

extern "C" void commit_preedit(char* preedit)
{
    std::string preeditStr(preedit);
    engine->commitPreedit(preeditStr);
}
/* END ENGINE */

namespace {

template <class F, typename... Args> void call_async(F&& lambda)
{
    // Modified from https://stackoverflow.com/a/56834117/1509779
    auto futptr = std::make_shared<std::future<void>>();
    *futptr = std::async(std::launch::async, [futptr, lambda]() { lambda(); });
}

class QuweiCandidate : public fcitx::CandidateWord {
public:
    QuweiCandidate(fcitx::Text text) { setText(text); }
    void select(fcitx::InputContext*) const {};
};

} // namespace

QuweiEngine::QuweiEngine(fcitx::Instance* instance)
    : instance_(instance)
{
    engine = this;
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

void QuweiEngine::commitCandidateByIndex(const int idx)
{
    auto candidate = ic_->inputPanel().candidateList()->candidate(idx).text();
    ic_->commitString(candidate.toStringForCommit());
    ic_->updateUserInterface(fcitx::UserInterfaceComponent::InputPanel);
}

void QuweiEngine::commitCandidateByFixedKey()
{
    auto idx = ic_->inputPanel().candidateList()->cursorIndex();
    commitCandidateByIndex(idx);
}

void QuweiEngine::commitPreedit(std::string preedit)
{
    ic_->commitString(preedit);
    reset();
}

bool QuweiEngine::canPageUp()
{
    if (auto* pageable = ic_->inputPanel().candidateList()->toPageable(); pageable) {
        return pageable->hasNext();
    }
    return false;
}

void QuweiEngine::nextPage()
{
    if (auto* pageable = ic_->inputPanel().candidateList()->toPageable(); pageable) {
        if (pageable->hasNext()) {
            pageable->next();
            ic_->updateUserInterface(fcitx::UserInterfaceComponent::InputPanel);
        }
    }
}

void QuweiEngine::prevPage()
{
    if (auto* pageable = ic_->inputPanel().candidateList()->toPageable(); pageable && pageable->hasPrev()) {
        pageable->prev();
        ic_->updateUserInterface(fcitx::UserInterfaceComponent::InputPanel);
    }
}

void QuweiEngine::nextCandidate()
{
    if (auto* cursorMovable = ic_->inputPanel().candidateList()->toCursorMovable()) {
        cursorMovable->nextCandidate();
        ic_->updateUserInterface(fcitx::UserInterfaceComponent::InputPanel);
    }
}

void QuweiEngine::prevCanddiate()
{
    if (auto* cursorMovable = ic_->inputPanel().candidateList()->toCursorMovable()) {
        cursorMovable->prevCandidate();
        ic_->updateUserInterface(fcitx::UserInterfaceComponent::InputPanel);
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

    // Call Rust side to handle
    bool shouldAccept = on_key_press(key);
    if (shouldAccept) {
        keyEvent.filterAndAccept();
    }
}

std::unique_ptr<fcitx::CommonCandidateList> QuweiEngine::makeCandidateList()
{
    auto candidateList = std::make_unique<fcitx::CommonCandidateList>();
    candidateList->setLabels(std::vector<std::string> { "1. ", "2. ", "3. ", "4. ", "5. ", "6. ", "7. ", "8. ", "9. ", "10. " });
    candidateList->setCursorPositionAfterPaging(fcitx::CursorPositionAfterPaging::ResetToFirst);
    candidateList->setPageSize(instance_->globalConfig().defaultPageSize());
    return candidateList;
}

void QuweiEngine::setLoading()
{
    auto candidateList = std::dynamic_pointer_cast<fcitx::CommonCandidateList>(ic_->inputPanel().candidateList());

    candidateList->clear();
    for (int i = 0; i < 5; i++) {
        std::unique_ptr<fcitx::CandidateWord> candidateWord = std::make_unique<QuweiCandidate>(fcitx::Text("☁"));
        candidateList->append(std::move(candidateWord));
        lens.push_back(0);
    }
    candidateList->setGlobalCursorIndex(0);

    ic_->updateUserInterface(fcitx::UserInterfaceComponent::InputPanel);
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
}

void QuweiEngine::setCandidates(std::vector<std::string> candidates, bool append)
{
    if (candidates.empty()) {
        return;
    }

    auto candidateList = std::dynamic_pointer_cast<fcitx::CommonCandidateList>(ic_->inputPanel().candidateList());
    
    if (!append) {
        candidateList->clear();
    }

    for (auto candidate : candidates) {
        std::unique_ptr<fcitx::CandidateWord> candidateWord = std::make_unique<QuweiCandidate>(fcitx::Text(candidate));
        candidateList->append(std::move(candidateWord));
    }

    ic_->updateUserInterface(fcitx::UserInterfaceComponent::InputPanel);
}

void QuweiEngine::appendCandidates(std::vector<std::string> candidates) { setCandidates(candidates, true); }

void QuweiEngine::clearCandidates()
{
    auto candidateList = std::dynamic_pointer_cast<fcitx::CommonCandidateList>(ic_->inputPanel().candidateList());

    candidateList->clear();

    ic_->updateUserInterface(fcitx::UserInterfaceComponent::InputPanel);
}

void QuweiEngine::updateUI() { ic_->updateUserInterface(fcitx::UserInterfaceComponent::InputPanel); }

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
    updateUI();
}

void QuweiEngine::reset(const fcitx::InputMethodEntry&, fcitx::InputContextEvent& event)
{
    FCITX_UNUSED(event);
    reset();
}

FCITX_ADDON_FACTORY(QuweiEngineFactory);
