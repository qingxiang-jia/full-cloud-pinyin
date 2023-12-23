#include "ims.h"
#include "msgs.pb.h"
#include <chrono>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <fcitx-utils/eventdispatcher.h>
#include <fcitx-utils/i18n.h>
#include <fcitx-utils/key.h>
#include <fcitx-utils/keysymgen.h>
#include <fcitx-utils/log.h>
#include <fcitx-utils/macros.h>
#include <fcitx-utils/utf8.h>
#include <fcitx/candidatelist.h>
#include <fcitx/inputcontext.h>
#include <fcitx/inputpanel.h>
#include <fcitx/instance.h>
#include <fcitx/text.h>
#include <fcitx/userinterfacemanager.h>
#include <functional>
#include <future>
#include <google/protobuf/descriptor.h>
#include <google/protobuf/io/coded_stream.h>
#include <google/protobuf/io/zero_copy_stream.h>
#include <google/protobuf/message.h>
#include <iostream>
#include <memory>
#include <new>
#include <thread>
#include <utility>
#include <vector>
#include <zmq.h>
#include <zmq.hpp>

class ImsCandidate : public fcitx::CandidateWord {
public:
  ImsCandidate(fcitx::Text text) { setText(text); }
  void select(fcitx::InputContext *) const {};
};

ImsEngine *engine;

ImsEngine::ImsEngine(fcitx::Instance *instance) : instance_(instance) {
  engine = this;
  isInSession = false;

  ctx = new zmq::context_t();
  pub = new zmq::socket_t(*ctx, ZMQ_PUB);
  pub->bind("tcp://127.0.0.1:8085");

  fcitx::EventDispatcher *dispatcher = new fcitx::EventDispatcher();
  this->dispatcher = dispatcher;
  dispatcher->attach(&instance_->eventLoop());

  imsServer = new ImsServer();
  imsServer->setEngine(this);
  imsServer->setDispatcher(dispatcher);
  std::thread serverThread(&ImsServer::serve, imsServer);
  serverThread.detach();
}

ImsEngine::~ImsEngine() {
  delete pub;
  delete ctx;
  delete imsServer;
  dispatcher->detach();
  delete dispatcher;
}

void ImsEngine::activate(const fcitx::InputMethodEntry &entry,
                         fcitx::InputContextEvent &event) {
  FCITX_UNUSED(entry);
  ic = event.inputContext();
}

KeyEvent *fcitxKeyToProtoKey(fcitx::KeySym &fk) {
  switch (fk) {
  case fcitx::KeySym::FcitxKey_0:
    return new KeyEvent(NUM_0);
  case fcitx::KeySym::FcitxKey_1:
    return new KeyEvent(NUM_1);
  case fcitx::KeySym::FcitxKey_2:
    return new KeyEvent(NUM_2);
  case fcitx::KeySym::FcitxKey_3:
    return new KeyEvent(NUM_3);
  case fcitx::KeySym::FcitxKey_4:
    return new KeyEvent(NUM_4);
  case fcitx::KeySym::FcitxKey_5:
    return new KeyEvent(NUM_5);
  case fcitx::KeySym::FcitxKey_6:
    return new KeyEvent(NUM_6);
  case fcitx::KeySym::FcitxKey_7:
    return new KeyEvent(NUM_7);
  case fcitx::KeySym::FcitxKey_8:
    return new KeyEvent(NUM_8);
  case fcitx::KeySym::FcitxKey_9:
    return new KeyEvent(NUM_9);
  case fcitx::KeySym::FcitxKey_a:
    return new KeyEvent(A_LWR);
  case fcitx::KeySym::FcitxKey_b:
    return new KeyEvent(B_LWR);
  case fcitx::KeySym::FcitxKey_c:
    return new KeyEvent(C_LWR);
  case fcitx::KeySym::FcitxKey_d:
    return new KeyEvent(D_LWR);
  case fcitx::KeySym::FcitxKey_e:
    return new KeyEvent(E_LWR);
  case fcitx::KeySym::FcitxKey_f:
    return new KeyEvent(F_LWR);
  case fcitx::KeySym::FcitxKey_g:
    return new KeyEvent(G_LWR);
  case fcitx::KeySym::FcitxKey_h:
    return new KeyEvent(H_LWR);
  case fcitx::KeySym::FcitxKey_i:
    return new KeyEvent(I_LWR);
  case fcitx::KeySym::FcitxKey_j:
    return new KeyEvent(J_LWR);
  case fcitx::KeySym::FcitxKey_k:
    return new KeyEvent(K_LWR);
  case fcitx::KeySym::FcitxKey_l:
    return new KeyEvent(L_LWR);
  case fcitx::KeySym::FcitxKey_m:
    return new KeyEvent(M_LWR);
  case fcitx::KeySym::FcitxKey_n:
    return new KeyEvent(N_LWR);
  case fcitx::KeySym::FcitxKey_o:
    return new KeyEvent(O_LWR);
  case fcitx::KeySym::FcitxKey_p:
    return new KeyEvent(P_LWR);
  case fcitx::KeySym::FcitxKey_q:
    return new KeyEvent(Q_LWR);
  case fcitx::KeySym::FcitxKey_r:
    return new KeyEvent(R_LWR);
  case fcitx::KeySym::FcitxKey_s:
    return new KeyEvent(S_LWR);
  case fcitx::KeySym::FcitxKey_t:
    return new KeyEvent(T_LWR);
  case fcitx::KeySym::FcitxKey_u:
    return new KeyEvent(U_LWR);
  case fcitx::KeySym::FcitxKey_v:
    return new KeyEvent(V_LWR);
  case fcitx::KeySym::FcitxKey_w:
    return new KeyEvent(W_LWR);
  case fcitx::KeySym::FcitxKey_x:
    return new KeyEvent(X_LWR);
  case fcitx::KeySym::FcitxKey_y:
    return new KeyEvent(Y_LWR);
  case fcitx::KeySym::FcitxKey_z:
    return new KeyEvent(Z_LWR);
  case fcitx::KeySym::FcitxKey_A:
    return new KeyEvent(A_UPR);
  case fcitx::KeySym::FcitxKey_B:
    return new KeyEvent(B_UPR);
  case fcitx::KeySym::FcitxKey_C:
    return new KeyEvent(C_UPR);
  case fcitx::KeySym::FcitxKey_D:
    return new KeyEvent(D_UPR);
  case fcitx::KeySym::FcitxKey_E:
    return new KeyEvent(E_UPR);
  case fcitx::KeySym::FcitxKey_F:
    return new KeyEvent(F_UPR);
  case fcitx::KeySym::FcitxKey_G:
    return new KeyEvent(G_UPR);
  case fcitx::KeySym::FcitxKey_H:
    return new KeyEvent(H_UPR);
  case fcitx::KeySym::FcitxKey_I:
    return new KeyEvent(I_UPR);
  case fcitx::KeySym::FcitxKey_J:
    return new KeyEvent(J_UPR);
  case fcitx::KeySym::FcitxKey_K:
    return new KeyEvent(K_UPR);
  case fcitx::KeySym::FcitxKey_L:
    return new KeyEvent(L_UPR);
  case fcitx::KeySym::FcitxKey_M:
    return new KeyEvent(M_UPR);
  case fcitx::KeySym::FcitxKey_N:
    return new KeyEvent(N_UPR);
  case fcitx::KeySym::FcitxKey_O:
    return new KeyEvent(O_UPR);
  case fcitx::KeySym::FcitxKey_P:
    return new KeyEvent(P_UPR);
  case fcitx::KeySym::FcitxKey_Q:
    return new KeyEvent(Q_UPR);
  case fcitx::KeySym::FcitxKey_R:
    return new KeyEvent(R_UPR);
  case fcitx::KeySym::FcitxKey_S:
    return new KeyEvent(S_UPR);
  case fcitx::KeySym::FcitxKey_T:
    return new KeyEvent(T_UPR);
  case fcitx::KeySym::FcitxKey_U:
    return new KeyEvent(U_UPR);
  case fcitx::KeySym::FcitxKey_V:
    return new KeyEvent(V_UPR);
  case fcitx::KeySym::FcitxKey_W:
    return new KeyEvent(W_UPR);
  case fcitx::KeySym::FcitxKey_X:
    return new KeyEvent(X_UPR);
  case fcitx::KeySym::FcitxKey_Y:
    return new KeyEvent(Y_UPR);
  case fcitx::KeySym::FcitxKey_Z:
    return new KeyEvent(Z_UPR);
  case fcitx::KeySym::FcitxKey_comma:
    return new KeyEvent(COMMA);
  case fcitx::KeySym::FcitxKey_period:
    return new KeyEvent(PERIOD);
  case fcitx::KeySym::FcitxKey_question:
    return new KeyEvent(QEST_MARK);
  case fcitx::KeySym::FcitxKey_Excel:
    return new KeyEvent(EXCL_MARK);
  case fcitx::KeySym::FcitxKey_semicolon:
    return new KeyEvent(SEMI_COLON);
  case fcitx::KeySym::FcitxKey_quotedbl:
    return new KeyEvent(DBL_QUOTE);
  case fcitx::KeySym::FcitxKey_quoteleft:
    return new KeyEvent(SGL_QUOTE);
  case fcitx::KeySym::FcitxKey_quoteright:
    return new KeyEvent(SGL_QUOTE);
  case fcitx::KeySym::FcitxKey_BackSpace:
    return new KeyEvent(BACKSPACE);
  case fcitx::KeySym::FcitxKey_bracketleft:
    return new KeyEvent(BRKT_OPEN);
  case fcitx::KeySym::FcitxKey_bracketright:
    return new KeyEvent(BRKT_CLOSE);
  case fcitx::KeySym::FcitxKey_slash:
    return new KeyEvent(SLASH);
  case fcitx::KeySym::FcitxKey_backslash:
    return new KeyEvent(BACKSLASH);
  case fcitx::KeySym::FcitxKey_ellipsis:
    return new KeyEvent(ELLIPSIS);
  case fcitx::KeySym::FcitxKey_Return:
    return new KeyEvent(ENTER);
  case fcitx::KeySym::FcitxKey_space:
    return new KeyEvent(SPACE);
  case fcitx::KeySym::FcitxKey_minus:
    return new KeyEvent(MINUS);
  case fcitx::KeySym::FcitxKey_equal:
    return new KeyEvent(EQUAL);
  case fcitx::KeySym::FcitxKey_Up:
    return new KeyEvent(UP);
  case fcitx::KeySym::FcitxKey_Down:
    return new KeyEvent(DOWN);
  case fcitx::KeySym::FcitxKey_Left:
    return new KeyEvent(LEFT);
  case fcitx::KeySym::FcitxKey_Right:
    return new KeyEvent(RIGHT);
  case fcitx::KeySym::FcitxKey_Shift_L:
    return new KeyEvent(SHIFT);
  case fcitx::KeySym::FcitxKey_Shift_R:
    return new KeyEvent(SHIFT);
  case fcitx::KeySym::FcitxKey_Control_L:
    return new KeyEvent(CTRL);
  case fcitx::KeySym::FcitxKey_Control_R:
    return new KeyEvent(CTRL);
  case fcitx::KeySym::FcitxKey_Alt_L:
    return new KeyEvent(ALT);
  case fcitx::KeySym::FcitxKey_Alt_R:
    return new KeyEvent(ALT);
  default:
    return nullptr;
  }
}

void ImsEngine::keyEvent(const fcitx::InputMethodEntry &entry,
                         fcitx::KeyEvent &keyEvent) {
  FCITX_UNUSED(entry);

  if (keyEvent.isRelease() || keyEvent.key().states() ||
      keyEvent.key().isModifier() || keyEvent.key().isUAZ() ||
      (!keyEvent.key().isLAZ() && !inSession())) {
    return;
  }

  fcitx::KeySym key = keyEvent.key().sym();
  KeyEvent *protoKey = fcitxKeyToProtoKey(key);
  if (protoKey == nullptr) {
    keyEvent.forward();
    return;
  }
  FcitxEvent msg;
  msg.set_event(*protoKey);
  std::string serialized = msg.SerializeAsString();

  zmq::message_t keyMsg(serialized.size());
  memcpy(keyMsg.data(), serialized.data(), serialized.size());
  pub->send(keyMsg, zmq::send_flags::dontwait);

  keyEvent.filterAndAccept();
}

void ImsEngine::reset(const fcitx::InputMethodEntry &,
                      fcitx::InputContextEvent &event) {
  FCITX_UNUSED(event);
}

void ImsEngine::inSession(const bool isInSession) {
  this->mtxInSession.lock();
  this->isInSession = isInSession;
  this->mtxInSession.unlock();
}

bool ImsEngine::inSession() {
  this->mtxInSession.lock_shared();
  bool toReturn = this->isInSession;
  this->mtxInSession.unlock_shared();

  return toReturn;
}

fcitx::InputContext *ImsEngine::getInputContext() { return ic; }

fcitx::Instance *ImsEngine::getInstance() { return instance_; }

ImsServer::ImsServer() {
  ctx = new zmq::context_t();
  rep = new zmq::socket_t(*ctx, ZMQ_REP);
  rep->bind("tcp://127.0.0.1:8086");
}

ImsServer::~ImsServer() {
  delete ctx;
  delete rep;
}

void ImsServer::setEngine(ImsEngine *engine) { this->engine = engine; }

void ImsServer::setDispatcher(fcitx::EventDispatcher *dispatcher) {
  this->dispatcher = dispatcher;
}

void ImsServer::dispatch(CommandToFcitx *cmd) {
  if (engine == nullptr || engine->getInputContext() == nullptr) {
    return;
  }
  // Thanks to the author of Fcitx5 for pointing out the correct way of working
  // with EventDispatcher.
  dispatcher->schedule([engine = engine, cmd = *cmd]() {
    auto ic = engine->getInputContext();
    ic->inputPanel().reset();

    if (cmd.has_update_session_status()) {
      auto inSession = cmd.update_session_status().in_session();
      engine->inSession(inSession);
      return;
    }

    if (cmd.has_commit_text()) {
      auto text = cmd.commit_text().text();
      ic->commitString(text);
      return;
    }

    if (cmd.has_update_preedit()) {
      auto preedit = cmd.update_preedit().text();
      if (ic->capabilityFlags().test(fcitx::CapabilityFlag::Preedit)) {
        fcitx::Text text(preedit, fcitx::TextFormatFlag::HighLight);
        ic->inputPanel().setClientPreedit(text);
      } else {
        fcitx::Text text(preedit);
        ic->inputPanel().setPreedit(text);
      }
      ic->updatePreedit();
      return;
    }

    if (cmd.has_update_aux()) {
      auto candidates = cmd.update_aux().candidates();
      ic->inputPanel().setAuxDown(fcitx::Text(candidates));
    } else {
      ic->inputPanel().setAuxDown(fcitx::Text(std::string()));
    }
    ic->updateUserInterface(fcitx::UserInterfaceComponent::InputPanel);
  });
}

void ImsServer::serve() {
  zmq::message_t *msg = new zmq::message_t();
  zmq::message_t *empty = new zmq::message_t();
  while (true) {
    // Receive request.
    auto maybeSize = rep->recv(*msg);
    if (!maybeSize.has_value()) {
      continue;
    }
    auto size = maybeSize.value();

    // Process request.
    auto data = msg->data();
    CommandToFcitx cmd;
    if (cmd.ParseFromArray(data, size)) {
      dispatch(&cmd);
    }

    // Signal process completion.
    maybeSize = rep->send(*empty, zmq::send_flags::none);
  }
}

FCITX_ADDON_FACTORY(ImsEngineFactory);
