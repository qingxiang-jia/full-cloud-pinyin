![Peek 2024-01-04 20-32](https://github.com/qingxiang-jia/full-cloud-pinyin/assets/5571586/6fc3a74f-f206-439d-aae7-2f255b202c6f)

Full Cloud Pinyin is an input method that uses [Google Input Tools](https://www.google.com/inputtools/try/) as the backend. It uses [fcitx5-bridge](https://github.com/qingxiang-jia/fcitx5-bridge) so you can type pinyin with excellent prediction on Linux with Fcitx5. Historically, a version made with IBus is also available [here](https://github.com/qingxiang-jia/ibus-cloud-pinyin).

## Why?

I want an input method that provides good prediction. The [pinyin](https://github.com/fcitx/fcitx5-chinese-addons) came close but compared Google Input Tools, it still has room to improve. So why not bridge Google Input Tools to Linux desktop? To do this, we need two parts:

1. fcitx5-bridge that allows talking to Fcitx5 on your Linux desktop (see above links).
1. a input method that takes your keyboard input to Google Input Tools and gets the candidates back (this project).

## Features

For Pinyin:
- Self-made phrase support
- Caching support (so for a given pinyin, it only access the Internet the first time you type it)

For Nepali:
- Caching support (no self-made phrase support becasue Google Input Tools lacks this feature for Nepali)

## Installation

Go to [releases](https://github.com/qingxiang-jia/full-cloud-pinyin/releases) and see instruction there. If you are using Ubuntu that has outdated glibc, you might need to compile from source. To do that:
1. Ensure you can build fcitx5-bridge.
1. Read [release.nu](https://github.com/qingxiang-jia/full-cloud-pinyin/blob/main/release.nu) and install [Nushell](https://www.nushell.sh/) (you do not need to set it as your default shell). Run `release.nu` to build and follow instructions in releases.