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

ImsEngine* engine;

ImsEngine::ImsEngine(fcitx::Instance* instance)
{
    FCITX_UNUSED(instance);

    engine = this;
}

void ImsEngine::activate(const fcitx::InputMethodEntry& entry, fcitx::InputContextEvent& event)
{
    FCITX_UNUSED(entry);
    FCITX_UNUSED(event);
}

void ImsEngine::keyEvent(const fcitx::InputMethodEntry& entry, fcitx::KeyEvent& keyEvent)
{
    FCITX_UNUSED(entry);
    FCITX_INFO() << "key event";
    keyEvent.filterAndAccept();
}

void ImsEngine::reset(const fcitx::InputMethodEntry&, fcitx::InputContextEvent& event)
{
    FCITX_UNUSED(event);
}
