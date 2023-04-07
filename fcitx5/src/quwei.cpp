/*
 * SPDX-FileCopyrightText: 2021~2021 CSSlayer <wengxt@gmail.com>
 *
 * SPDX-License-Identifier: BSD-3-Clause
 *
 */
#include "quwei.h"
#include <chrono>
#include <cstddef>
#include <fcitx-utils/eventdispatcher.h>
#include <fcitx-utils/i18n.h>
#include <fcitx-utils/key.h>
#include <fcitx-utils/keysymgen.h>
#include <fcitx-utils/log.h>
#include <fcitx-utils/macros.h>
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
extern "C" void set_loading()
{
    std::cout << "Begin set_loading\n";
    
    engine->setLoading();
    
    std::cout << "end set_loading\n";
}

extern "C" void set_candidates(int16_t** candidates, size_t cnt)
{
    std::cout << "Begin set_candidates\n";

    std::vector<std::string> candidatesToSet;
    for (size_t i = 0; i < cnt; i++) {
        std::string candidate((char*)candidates[i]);
        candidatesToSet.push_back(std::move(candidate));
    }
    engine->setCandidates(candidatesToSet);
    
    std::cout << "end set_candidates\n";
}

extern "C" void clear_candidates()
{
    std::cout << "Begin clear_candidates\n";
    
    engine->clearCandidates();
    
    std::cout << "end clear_candidates\n";
}

extern "C" void set_preedit(char* preedit)
{
    std::cout << "Begin set_preedit\n";
    
    std::string preeditStr(preedit);
    engine->setPreedit(std::move(preeditStr));
    
    std::cout << "end set_preedit\n";
}
/* END UI */

/* BEGIN TABLE */
extern "C" bool can_page_up()
{
    return engine->canPageUp();
}

extern "C" void page_up()
{
    engine->nextPage();
}

extern "C" void page_down()
{
    engine->prevPage();
}

extern "C" void prev()
{
    engine->prevCanddiate();
}

extern "C" void next()
{
    engine->nextCandidate();
}

extern "C" void set_page(int idx)
{
    engine->setPage(idx);
}
/* END TABLE */

/* BEGIN ENGINE */
extern "C" void commit(uint16_t idx)
{
    engine->commitCandidateByIndex(idx);
}

extern "C" void commit_candidate_by_fixed_key()
{
    engine->commitCandidateByFixedKey();
}

extern "C" void commit_preedit(char* preedit)
{
    std::cout << "Begin commit_preedit\n";
    
    std::string preeditStr(preedit);
    engine->commitPreedit(preeditStr);
    
    std::cout << "end commit_preedit\n";
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
    std::cout << "Begin QuweiEngine\n";
    engine = this;

    // Initialize Rust side
    FcpOpaque* fcpOpaque = new_fcp();
    this->fcpOpaque = fcpOpaque;

    // Prepare callbacks
    FnVoid fn_set_loading = &set_loading;
    FnSetCandidates fn_set_candidates = &set_candidates;
    FnVoid fn_clear_candidates = &clear_candidates;
    FnSetPreedit fn_set_preedit = &set_preedit;
    FnCanPageUp fn_can_page_up = &can_page_up;
    FnVoid fn_page_up = &page_up;
    FnVoid fn_page_down = &page_down;
    FnVoid fn_prev = &prev;
    FnVoid fn_next = &next;
    FnSetPage fn_set_page = &set_page;
    FnCommit fn_commit = &commit;
    FnSetPreedit fn_commit_preedit = &commit_preedit;
    FnVoid fn_commit_candidate_by_fixed_key = &commit_candidate_by_fixed_key;

    // Register callbacks
    register_callbacks(fcpOpaque, fn_set_loading, fn_set_candidates, fn_clear_candidates, fn_set_preedit, fn_can_page_up, fn_page_up, fn_page_down, fn_prev,
        fn_next, fn_set_page, fn_commit, fn_commit_preedit, fn_commit_candidate_by_fixed_key);

    // Initialize dispatcher
    dispatcher = std::make_unique<fcitx::EventDispatcher>();
    dispatcher->attach(&instance->eventLoop());
    
    std::cout << "end QuweiEngine\n";
}

void QuweiEngine::activate(const fcitx::InputMethodEntry& entry, fcitx::InputContextEvent& event)
{
    std::cout << "Begin activate\n";
    
    FCITX_UNUSED(entry);
    auto* inputContext = event.inputContext();
    ic_ = inputContext;
    
    std::cout << "end activate\n";
}

void QuweiEngine::select(const int idx)
{
    FCITX_UNUSED(idx);
}

void QuweiEngine::commitCandidateByIndex(const int idx)
{
    std::cout << "Begin commitCandidateByIndex\n";
    
    auto candidate = ic_->inputPanel().candidateList()->candidate(idx).text();
    ic_->commitString(candidate.toStringForCommit());
    clearCandidates();
    setPreedit("");
    uiUpdate();
    
    std::cout << "end commitCandidateByIndex\n";
}

void QuweiEngine::commitCandidateByFixedKey()
{
    std::cout << "Begin commitCandidateByFixedKey\n";
    
    auto idx = ic_->inputPanel().candidateList()->cursorIndex();
    if (idx == -1) {
        // When you type and didn't interact with the candidates, it's -1, but we know you mean 0
        idx = 0;
    }
    commitCandidateByIndex(idx);
    
    std::cout << "end commitCandidateByFixedKey\n";
}

void QuweiEngine::commitPreedit(std::string preedit)
{
    std::cout << "Begin commitPreedit\n";
    
    ic_->commitString(preedit);
    reset();
    
    std::cout << "end commitPreedit\n";
}

bool QuweiEngine::canPageUp()
{
    std::cout << "Begin canPageUp\n";
    
    if (auto* pageable = ic_->inputPanel().candidateList()->toPageable(); pageable) {
        return pageable->hasNext();
    }
    
    std::cout << "end canPageUp\n";
    return false;
}

void QuweiEngine::nextPage()
{
    std::cout << "Begin nextPage\n";
    
    if (auto* pageable = ic_->inputPanel().candidateList()->toPageable(); pageable) {
        if (pageable->hasNext()) {
            pageable->next();
            uiUpdate();
        }
    }
    
    std::cout << "end nextPage\n";
}

void QuweiEngine::prevPage()
{
    std::cout << "Begin prevPage\n";
    
    if (auto* pageable = ic_->inputPanel().candidateList()->toPageable(); pageable && pageable->hasPrev()) {
        pageable->prev();
        uiUpdate();
    }
    
    std::cout << "end prevPage\n";
}

void QuweiEngine::nextCandidate()
{
    std::cout << "Begin nextCandidate\n";
    
    if (auto* cursorMovable = ic_->inputPanel().candidateList()->toCursorMovable()) {
        cursorMovable->nextCandidate();
        uiUpdate();
    }
    
    std::cout << "end nextCandidate\n";
}

void QuweiEngine::prevCanddiate()
{
    std::cout << "Begin prevCanddiate\n";
    
    if (auto* cursorMovable = ic_->inputPanel().candidateList()->toCursorMovable()) {
        cursorMovable->prevCandidate();
        uiUpdate();
    }
    
    std::cout << "end prevCanddiate\n";
}

void QuweiEngine::setPage(int idx)
{
    std::cout << "Begin setPage\n";
    
    if (auto* pageable = ic_->inputPanel().candidateList()->toPageable(); pageable) {
        pageable->setPage(idx);
        uiUpdate();
    }
    
    std::cout << "end setPage\n";
}

void QuweiEngine::keyEvent(const fcitx::InputMethodEntry& entry, fcitx::KeyEvent& keyEvent)
{
    std::cout << "Begin keyEvent\n";
    
    FCITX_UNUSED(entry);
    if (keyEvent.isRelease() || keyEvent.key().states()) {
        
        std::cout << "end keyEvent\n";
        
        return;
    }
    if (ic_->inputPanel().candidateList() == nullptr) {
        ic_->inputPanel().setCandidateList(makeCandidateList());
    } // Surprisingly, if you set it in activate(), it is still null when keyuEvent is called.

    fcitx::KeySym key = keyEvent.key().sym();

    // Call Rust side to handle
    bool shouldAccept = on_key_press(fcpOpaque, key);
    if (shouldAccept) {
        keyEvent.filterAndAccept();
    }
    
    std::cout << "end keyEvent\n";
}

std::unique_ptr<fcitx::CommonCandidateList> QuweiEngine::makeCandidateList()
{
    std::cout << "Begin makeCandidateList\n";
    
    auto candidateList = std::make_unique<fcitx::CommonCandidateList>();
    candidateList->setLabels(std::vector<std::string> { "1. ", "2. ", "3. ", "4. ", "5. ", "6. ", "7. ", "8. ", "9. ", "10. " });
    candidateList->setCursorPositionAfterPaging(fcitx::CursorPositionAfterPaging::ResetToFirst);
    candidateList->setPageSize(instance_->globalConfig().defaultPageSize());
    
    std::cout << "end makeCandidateList\n";
    
    return candidateList;
}

void QuweiEngine::setLoading()
{
    std::cout << "Begin setLoading\n";
    
    auto candidateList = std::dynamic_pointer_cast<fcitx::CommonCandidateList>(ic_->inputPanel().candidateList());

    candidateList->clear();
    for (int i = 0; i < 5; i++) {
        std::unique_ptr<fcitx::CandidateWord> candidateWord = std::make_unique<QuweiCandidate>(fcitx::Text("☁"));
        candidateList->append(std::move(candidateWord));
    }
    candidateList->setGlobalCursorIndex(0);

    uiUpdate();
    
    std::cout << "end setLoading\n";
}

void QuweiEngine::setPreedit(std::string preedit)
{
    std::cout << "Begin setPreedit\n";
    
    if (ic_->capabilityFlags().test(fcitx::CapabilityFlag::Preedit)) {
        fcitx::Text text(preedit, fcitx::TextFormatFlag::HighLight);
        ic_->inputPanel().setClientPreedit(text);
    } else {
        fcitx::Text text(preedit);
        ic_->inputPanel().setPreedit(text);
    }
    ic_->updatePreedit();
    
    std::cout << "end setPreedit\n";
}

void QuweiEngine::setCandidates(std::vector<std::string> candidates)
{
    std::cout << "Begin setCandidates\n";
    if (candidates.empty()) {
        
        std::cout << "end setCandidates\n";
        
        return;
    }

    auto candidateList = std::dynamic_pointer_cast<fcitx::CommonCandidateList>(ic_->inputPanel().candidateList()); //
    
    if (candidateList == nullptr) {
        std::cout << "end setCandidates 1\n";

        return;
    }
    candidateList->clear();

    for (auto candidate : candidates) {
        std::unique_ptr<fcitx::CandidateWord> candidateWord = std::make_unique<QuweiCandidate>(fcitx::Text(candidate));
        candidateList->append(std::move(candidateWord));
    }

    uiUpdate();

    std::cout << "end setCandidates\n";
}

void QuweiEngine::clearCandidates()
{
    std::cout << "Begin clearCandidates\n";
    
    if (ic_ == nullptr || ic_->inputPanel().candidateList() == nullptr) {
        std::cout << "end clearCandidates 1\n";
        return; // It seems this could happen too
    }
    auto candidateList = std::dynamic_pointer_cast<fcitx::CommonCandidateList>(ic_->inputPanel().candidateList());

    if (candidateList == nullptr) {
        std::cout << "Begin clearCandidates 2\n";
        return; // Other operations may have already cause it to be null
    }
    candidateList->clear();

    uiUpdate();
 
    std::cout << "end clearCandidates\n";
}

void QuweiEngine::uiUpdate()
{
    std::cout << "Begin uiUpdate\n";
    
    if (ic_ != nullptr) {
        ic_->updateUserInterface(fcitx::UserInterfaceComponent::InputPanel);
    }
    
    std::cout << "end uiUpdate\n";
}

void QuweiEngine::reset()
{
    std::cout << "Begin QuweiEngine::reset\n";
    
    ic_->inputPanel().reset();
    setPreedit("");
    uiUpdate();
    
    std::cout << "end QuweiEngine::reset\n";
}

void QuweiEngine::reset(const fcitx::InputMethodEntry&, fcitx::InputContextEvent& event)
{
    std::cout << "Begin QuweiEngine::reset 2 args\n";
    
    FCITX_UNUSED(event);
    reset();
    
    std::cout << "end QuweiEngine::reset 2 args\n";
}

FCITX_ADDON_FACTORY(QuweiEngineFactory);
