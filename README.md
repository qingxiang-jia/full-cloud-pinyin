## Introduction

Using pinyin on Linux has been improved a lot, especially with Fcitx (both 4 and 5). But I tried Google Input Tools and personally I feel it gives much better prediction. So I want to bring to Linux. Note, both IBus and Fcitx have cloud pinyin that kind of does this already. But it has a few issues that hinders usability:

1. There's only one candidate from cloud. I can see that cloud pinyin is mostly good at long pinyin so taking just one candidate strikes a good balance between efforts and feature. But having all candidates provides better experience.

2. It seems the HTTP connection of getting that candidate from cloud is re-established every time one types. This creates a lot of latency for cloud piniyin. Cloud pinyin can be fast. In my area (North America), the latency has been consistenly between 70ms-150ms. With async, this is more than acceptable.

## Project Structure

 - fcitx5 - the implementation based on Fcitx5.
 - ibus - the implementation based on IBus.
 - fcpinyin - the part that gets candidates from Google Input Tools.
 - cpp-17-async - play around ways to do async in C++ 17.
 - cpp-rust-interop - play around ways to do C++/Rust interop.

## How to Build

### Fcitx5

Enter the folder, the build system is CMake. But because the fcitx5 verison of the input method uses Rust code to get the candidates (called by C++ code), to ease the binding work, I used [CXX](cxx.rs). The idea is to first compile the Rust code get a statically linked object file. Then, use the header file generated by CXX so C++ knows how to access functions and variables in the object file.

The whole build step looks like this:

1. Build Rust code.
2. Build C++ code.
3. Install binary to OS to test.

So instead of running CMake directly, using commands in `task.sh` is probably easier. In particular, in `fcpinyin/`

- `./task.sh gen-lib` to compile the Rust code into a statically linked object file
- `./task.sh gen-cxx` to generate header files for that object file using CXX

In `fcitx5/`,

- `./task.sh --init` to get the out-of-source build directory created and a bunch of CMake intialization work
- `./task.sh --build` to build the C++ code and link with the previously built object files
- `./task.sh --install` to install the binary to OS to test

Note: the installation directory is specific to Arch. If you use something else, the installtion direction might need to be adjusted. I am new to both Rust and C++ so a lot of these setup may be overly complicated. Let me know if there's a better approach.

## Future Plans
    
-   Less web requests: we could do caching, so common pinyin doesn't need to go through the internet.
-   Better privacy: user defines what words (in pinyin) that shouldn't go to the internet.
-   We could also support user-created words, something regular pinyin input methods already do but cloud ones can't.