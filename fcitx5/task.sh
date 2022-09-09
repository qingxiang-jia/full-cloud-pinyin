#!/usr/bin/env bash

if [ $# == 0 ]; then 
    echo "No argument passed in"
    exit 0
fi

if [ $1 == "init" ]; then
    rm -rf ./build
    mkdir build
    cd ./build
    cmake -DCMAKE_INSTALL_PREFIX=/usr ..
    cd ../
    exit 0
fi

if [ $1 == "build" ]; then
    cd ./build
    cmake --build .
    cd ../
    exit 0
fi

if [ $1 == "install" ]; then
    cd ./build
    sudo cmake --install .
    cd ../
    exit 0
fi

if [ $1 == "uninstall" ]; then
    cd ./build
    sudo xargs -I{} rm "{}" < ./install_manifest.txt
    cd ../
    exit 0
fi

echo "Nothing done"