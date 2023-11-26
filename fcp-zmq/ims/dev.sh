#!/usr/bin/env bash

if [ $1 == "init_cmake" ]; then
    mkdir -p build
    cd ./build
    cmake -DCMAKE_INSTALL_PREFIX=/usr -DCMAKE_EXPORT_COMPILE_COMMANDS=1 -DCMAKE_BUILD_TYPE=Debug ..
    cd ../
    exit 0
fi

if [ $1 == "clean" ]; then
    if [ -d ./build ]; then
        rm -rf ./build
    fi
    exit 0
fi

if [ $1 == "build" ]; then
    cmake --build ./build
    exit 0
fi

echo "Nothing done"
