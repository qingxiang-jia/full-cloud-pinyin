# InputContext

```c++
commitString(std::string);
updatePreedit();
inputPanel();
inputPanel().candidateList();
inputPanel().setCandidateList(std::move(unique_ptr<Fcitx::CommonCandidateList>));
updateUserInterface(fcitx::UserInterfaceComponent::InputPanel);
capabilityFlags().test(fcitx::CapabilityFlag::Preedit);
```

# KeyEvent

```c++
key().keyListIndex(std::array<fcitx::Key>);
accept();
key().check(enum _FcitxKeySym);
key().checkKeyList(std::array<fcitx::Key>);
key().sym();
filterAndAccept();
```

# buffer_

```c++
backspace();
userInput();
type(uint32_t unicode);
type(std::string &s);
clear();
```

# QuweiState

```c++
keyEvent(fcitx::KeyEvent); <- QuweiEngine::keyEvent
setCandidateList();
updateUI();
getUpdateCandidatesRefreshUI();
getPreedit();
preeditRemoveFront(int lenToRemove);
```

# QuweiEngine

```c++
activate(const fcitx::InputMethodEntry &entry, fcitx::InputContextEvent &event);
keyEvent(const fcitx::InputMethodEntry &entry, fcitx::KeyEvent &keyEvent);
reset(const fcitx::InputMethodEntry &, fcitx::InputContextEvent &event);
```