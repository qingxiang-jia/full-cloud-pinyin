/*
 * SPDX-FileCopyrightText: 2021~2021 CSSlayer <wengxt@gmail.com>
 *
 * SPDX-License-Identifier: BSD-3-Clause
 *
 */

#pragma once

#include "rustpinyin.h"
#include <fcitx-utils/eventdispatcher.h>
#include <fcitx-utils/inputbuffer.h>
#include <fcitx/addonfactory.h>
#include <fcitx/addonmanager.h>
#include <fcitx/candidatelist.h>
#include <fcitx/inputcontext.h>
#include <fcitx/inputcontextproperty.h>
#include <fcitx/inputmethodengine.h>
#include <fcitx/inputpanel.h>
#include <fcitx/instance.h>
#include <memory>
#include <vector>

class QuweiEngine;

class QuweiEngine : public fcitx::InputMethodEngineV2 {
public:
    QuweiEngine(fcitx::Instance* instance);

    void activate(const fcitx::InputMethodEntry& entry, fcitx::InputContextEvent& event) override;
    void keyEvent(const fcitx::InputMethodEntry& entry, fcitx::KeyEvent& keyEvent) override;
    void reset(const fcitx::InputMethodEntry&, fcitx::InputContextEvent& event) override;

    std::unique_ptr<RustPinyin> rustPinyin_;
    std::unique_ptr<fcitx::EventDispatcher> dispatcher;

private:
    fcitx::Instance* instance_;
    fcitx::InputContext* ic_;
    fcitx::InputBuffer buffer_ { { fcitx::InputBufferOption::AsciiOnly, fcitx::InputBufferOption::FixedCursor } };
    std::vector<unsigned long> lens;

    void select(const int idx);
    void setPreedit(std::string preedit);
    void setCandidates(::rust::Vec< ::fcp::CandidateWord> candidates, bool append);
    void getCandidatesAndUpdateAsync(bool append = false);
    void getUpdateCandidatesRefreshUI(bool append);
    void preeditRemoveFirstN(int lenToRemove);
    std::unique_ptr<fcitx::CommonCandidateList> makeCandidateList();
    void setDummyCandidates();
    void reset();
};

class QuweiEngineFactory : public fcitx::AddonFactory {
    fcitx::AddonInstance* create(fcitx::AddonManager* manager) override
    {
        FCITX_UNUSED(manager);
        return new QuweiEngine(manager->instance());
    }
};