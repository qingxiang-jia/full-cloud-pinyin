#include "ims.h"
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
#include <thread>
#include <utility>
#include <vector>
#include "msgpack.hpp"

ImsEngine* engine;

ImsEngine::ImsEngine(fcitx::Instance* instance)
    : instance_(instance)
{
    engine = this;
    ctx = new zmq::context_t();
    pub = new zmq::socket_t(*ctx, ZMQ_PUB);
}

void ImsEngine::activate(const fcitx::InputMethodEntry& entry, fcitx::InputContextEvent& event)
{
    FCITX_UNUSED(entry);
    FCITX_UNUSED(event);
}

void ImsEngine::keyEvent(const fcitx::InputMethodEntry& entry, fcitx::KeyEvent& keyEvent)
{
    FCITX_UNUSED(entry);

    if (keyEvent.isRelease() || keyEvent.key().states()) {
        return;
    }

    auto packer = msgpack::Packer{};
    packer.clear();
    packer.process(12);
    auto data = packer.vector().data();

    fcitx::KeySym key = keyEvent.key().sym();

    keyEvent.filterAndAccept();
}

void ImsEngine::reset(const fcitx::InputMethodEntry&, fcitx::InputContextEvent& event)
{
    FCITX_UNUSED(event);
}

ImsServer::ImsServer(fcitx::Instance* instance) {

}

FCITX_ADDON_FACTORY(ImsEngineFactory);
