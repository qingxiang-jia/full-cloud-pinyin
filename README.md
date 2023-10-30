Use GTK panel:

![Peek 2023-05-28 10-31](https://github.com/qingxiang-jia/full-cloud-pinyin/assets/5571586/a366969e-40ba-4ec5-b2e7-ae25808e0fcd)

Use Kimpanel:

![kimpanel](https://github.com/qingxiang-jia/full-cloud-pinyin/assets/5571586/e107b938-5411-445e-8986-462d202d7f58)


## Introduction

Using pinyin on Linux has been improved a lot, especially with Fcitx (both 4 and 5). But I tried Google Input Tools and personally I feel it gives much better prediction. So I want to bring to Linux. Note, both IBus and Fcitx have cloud pinyin that kind of does this already. But it has a few issues that hinders usability:

1. There's only one candidate from cloud. I can see that cloud pinyin is mostly good at long pinyin so taking just one candidate strikes a good balance between efforts and feature. But based my experience, usually the first 5 or so candidates are also pretty good.

2. It seems the HTTP connection of getting that candidate from cloud is re-established every time one types. This creates a lot of latency for cloud piniyin. Cloud pinyin can be fast. In my area, the latency has been consistenly between 70ms-150ms. With async, this is more than acceptable. A good example is [vscode-google-pinyin](https://github.com/zyctree/vscode-google-pinyin), try it in VSCode to get a sense of the latency.

## Project Structure

- fcp-goibus: This was my original attempt. @sarim [built](https://github.com/sarim/goibus) an IBus implementation on top of [godbus](https://github.com/godbus/dbus). It's very straightforward and I was able to build a proof of concept very quickly.
- fcp-fcitx5: This was my second attempt. Because the easy success with goibus, I decided to do something more interesting: bridge the C++ based Fcitx5 code with a Rust input method engine. The reason for Rust is that I want to write some real software with it and its async support makes a lot of sense for a web base input method engine. The input method works and for a while, I used it daily, but it's unstable. Even with all the careful locking, reducing the number of async workers to 1, and applying advice from the author of Fcitx5, it still crashes now and then.  Maybe [cxx-async](https://github.com/pcwalton/cxx-async) will work but I am tired.
- fcp-zbus: This is my third attempt, from the observation that, if we could generate IBus's Rust interface from its DBus XML, we could skip all the efforts of builing the C binding (like I did for Fcitx5), we could have a "pure" Rust implementation and never worry about cmake. It turns out both [dbus-rs](https://github.com/diwic/dbus-rs) and [zbus](https://github.com/dbus2/zbus/) do exactly that. In the end, I used zbus.
