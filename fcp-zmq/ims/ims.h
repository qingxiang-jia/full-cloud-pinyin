#pragma once

#include <cstdint>
#include <fcitx-utils/eventdispatcher.h>
#include <fcitx-utils/inputbuffer.h>
#include <fcitx-utils/macros.h>
#include <fcitx/addonfactory.h>
#include <fcitx/addoninstance.h>
#include <fcitx/addonmanager.h>
#include <fcitx/candidatelist.h>
#include <fcitx/inputcontext.h>
#include <fcitx/inputcontextproperty.h>
#include <fcitx/inputmethodengine.h>
#include <fcitx/inputpanel.h>
#include <fcitx/instance.h>
#include <memory>
#include <vector>

class ImsEngine;

class ImsEngine : public fcitx::InputMethodEngineV2 {
public:
    ImsEngine(fcitx::Instance* instance);

    void activate(const fcitx::InputMethodEntry& entry, fcitx::InputContextEvent& event) override;
    void keyEvent(const fcitx::InputMethodEntry& entry, fcitx::KeyEvent& keyEvent) override;
    void reset(const fcitx::InputMethodEntry&, fcitx::InputContextEvent& event) override;
};

class ImsEngineFactory : public fcitx::AddonFactory {
    fcitx::AddonInstance* create(fcitx::AddonManager* manager) override
    {
        FCITX_UNUSED(manager);
        return new ImsEngine(manager->instance());
    }
};