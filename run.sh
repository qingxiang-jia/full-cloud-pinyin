#!/usr/bin/env bash

if [ $# == 0 ]; then 
    echo "No argument passed in"
    exit 0
fi

# If we specify cdylib or staticlib in Cargo.toml, the main.rs can't properly reference contents in lib.rs.
if [ $1 == "gen-lib" ]; then
    cd ./fcpinyin
    cargo -Z unstable-options rustc --lib --release --crate-type staticlib -- --print native-static-libs
    cd ../
    exit 0
fi

if [ $1 == "rc" ]; then
    cd ./c-based-ffi
    cargo -Z unstable-options rustc --lib --release --crate-type staticlib -- --print native-static-libs
    cd ../
    exit 0
fi

if [ $1 == "gen-cxx" ]; then
    cd ./fcpinyin
    cxxbridge src/ffi.rs > ffi.h
    cd ../
    exit 0
fi

if [ $1 == "init" ]; then
    cd fcitx5
    rm -rf ./build
    mkdir build
    cd ./build
    cmake -DCMAKE_INSTALL_PREFIX=/usr -DCMAKE_EXPORT_COMPILE_COMMANDS=1 ..
    cd ../../
    exit 0
fi

if [ $1 == "build" ]; then
    cd ./fcitx5/build
    cmake --build .
    cd ../../
    exit 0
fi

if [ $1 == "install" ]; then
    cd ./fcitx5/build
    sudo cmake --install .
    cd ../../
    exit 0
fi

if [ $1 == "uninstall" ]; then
    cd ./fcitx5/build
    sudo xargs -I{} rm {} < ./install_manifest.txt
    cd ../../
    exit 0
fi

echo "Nothing done"