#pragma once

#include "ims_recv.pb.h"
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
class ImsServer;

class ImsEngine : public fcitx::InputMethodEngineV2 {
public:
    ImsEngine(fcitx::Instance* instance);
    ~ImsEngine();

    void activate(const fcitx::InputMethodEntry& entry, fcitx::InputContextEvent& event) override;
    void keyEvent(const fcitx::InputMethodEntry& entry, fcitx::KeyEvent& keyEvent) override;
    void reset(const fcitx::InputMethodEntry&, fcitx::InputContextEvent& event) override;

    fcitx::InputContext* getInputContext();
    fcitx::Instance* getInstance();

private:
    fcitx::Instance* instance_;
    fcitx::InputContext* ic;
    zmq::context_t* ctx;
    zmq::socket_t* pub;
    ImsServer* imsServer;
    fcitx::EventDispatcher* dispatcher;
};

class ImsServer {
public:
    ImsServer();
    void setEngine(ImsEngine* engine);
    void setDispatcher(fcitx::EventDispatcher* dispatcher);
    void serve();
    ~ImsServer();
private:
    zmq::context_t* ctx;
    zmq::socket_t* rep;
    ImsEngine* engine;
    fcitx::EventDispatcher* dispatcher;
    void dispatch(CommandToFcitx*);
};

class ImsEngineFactory : public fcitx::AddonFactory {
    fcitx::AddonInstance* create(fcitx::AddonManager* manager) override
    {
        return new ImsEngine(manager->instance());
    }
};
