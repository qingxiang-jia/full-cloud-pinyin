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
#include <thread>
#include <vector>
#include <zmq.hpp>

class ImsEngine;

class ImsEngine : public fcitx::InputMethodEngineV2 {
public:
    ImsEngine(fcitx::Instance* instance);
    ~ImsEngine();

    void activate(const fcitx::InputMethodEntry& entry, fcitx::InputContextEvent& event) override;
    void keyEvent(const fcitx::InputMethodEntry& entry, fcitx::KeyEvent& keyEvent) override;
    void reset(const fcitx::InputMethodEntry&, fcitx::InputContextEvent& event) override;

private:
    fcitx::Instance* instance_;
    zmq::context_t* ctx;
    zmq::socket_t* pub;
};

class ImsServer {
public:
    ImsServer(fcitx::Instance* instance);
    void Serve();
    ~ImsServer();
private:
    zmq::context_t* ctx;
    zmq::socket_t* sock;
    fcitx::Instance* ins;
};

void initImsServer(fcitx::Instance* ins) {
    auto imsServer = new ImsServer(ins);
    imsServer->Serve();
}

class ImsEngineFactory : public fcitx::AddonFactory {
    fcitx::AddonInstance* create(fcitx::AddonManager* manager) override
    {
        std::thread t(initImsServer, manager->instance());
        t.detach();
        return new ImsEngine(manager->instance());
    }
};
