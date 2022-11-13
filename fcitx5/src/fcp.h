#pragma once
#include <fcitx-utils/eventdispatcher.h>
#include <fcitx-utils/inputbuffer.h>
#include <fcitx/addonfactory.h>
#include <fcitx/addonmanager.h>
#include <fcitx/event.h>
#include <fcitx/inputcontext.h>
#include <fcitx/inputcontextproperty.h>
#include <fcitx/inputmethodengine.h>
#include <fcitx/inputmethodentry.h>
#include <fcitx/inputpanel.h>
#include <fcitx/instance.h>
#include <memory>
#include <vector>

class FcpEngine : public fcitx::InputMethodEngineV2 {
public:
    // Must have, not exposed to the Rust side
    FcpEngine(fcitx::Instance* instance);
    void activate(const fcitx::InputMethodEntry& entry, fcitx::InputContextEvent& event) override;
    void keyEvent(const fcitx::InputMethodEntry& entry, fcitx::KeyEvent& keyEvent) override;
    // Exposed to the Rust side via EventDispatcher and C FFI
    void commit(unsigned int idx);
    void pageUp();
    void pageDown();
    void setState(std::string preedit, std::vector<std::string> candidates, std::vector<unsigned int> lens);
};