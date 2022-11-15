/*
 * SPDX-FileCopyrightText: 2021~2021 CSSlayer <wengxt@gmail.com>
 *
 * SPDX-License-Identifier: BSD-3-Clause
 *
 */
#ifndef _FCITX5_QUWEI_QUWEI_H_
#define _FCITX5_QUWEI_QUWEI_H_

#include "rustpinyin.h"
#include <fcitx-utils/eventdispatcher.h>
#include <fcitx-utils/inputbuffer.h>
#include <fcitx/addonfactory.h>
#include <fcitx/addonmanager.h>
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

    void updateUI();
    void getCandidatesAndUpdateAsync(bool append = false);
    std::string getPreedit();
    void preeditRemoveFront(int lenToRemove);
    void reset();

    void reset(const fcitx::InputMethodEntry&, fcitx::InputContextEvent& event) override;

    auto instance() const { return instance_; }

    std::unique_ptr<RustPinyin> rustPinyin_;
    std::unique_ptr<fcitx::EventDispatcher> dispatcher;

private:
    fcitx::Instance* instance_;
    fcitx::InputContext* ic_;
    fcitx::InputBuffer buffer_ { { fcitx::InputBufferOption::AsciiOnly, fcitx::InputBufferOption::FixedCursor } };
    void getUpdateCandidatesRefreshUI(bool append);
};

class QuweiEngineFactory : public fcitx::AddonFactory {
    fcitx::AddonInstance* create(fcitx::AddonManager* manager) override
    {
        FCITX_UNUSED(manager);
        return new QuweiEngine(manager->instance());
    }
};

#endif // _FCITX5_QUWEI_QUWEI_H_
