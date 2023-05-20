/*
 * SPDX-FileCopyrightText: 2021~2021 CSSlayer <wengxt@gmail.com>
 *
 * SPDX-License-Identifier: BSD-3-Clause
 *
 */

#pragma once

#include "src/include/rust.h"
#include <cstdint>
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
    std::unique_ptr<fcitx::EventDispatcher> dispatcher;

    void commitCandidateByIndex(const int idx);
    void commitCandidateByFixedKey();
    void commitPreedit(std::string preedit);
    bool canPageUp();
    void nextPage();
    void prevPage();
    void nextCandidate();
    void prevCanddiate();
    void setPage(int idx);
    void setPreedit(std::string preedit);
    void setLoading();
    void setCandidates(std::vector<std::string> candidates);
    void appendCandidates(std::vector<std::string> candidates);
    void clearCandidates();
    void uiUpdate();

private:
    FcpOpaque* fcpOpaque;
    fcitx::Instance* instance_;
    fcitx::InputContext* ic_;

    void select(const int idx);
    std::unique_ptr<fcitx::CommonCandidateList> makeCandidateList();
    void reset();
};

class QuweiEngineFactory : public fcitx::AddonFactory {
    fcitx::AddonInstance* create(fcitx::AddonManager* manager) override
    {
        FCITX_UNUSED(manager);
        return new QuweiEngine(manager->instance());
    }
};