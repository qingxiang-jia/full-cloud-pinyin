#!/usr/bin/env bash

if [ $# == 0 ]; then 
    echo "No argument passed in"
    exit 0
fi

if [ $1 == "init" ]; then
    rm -rf ./build
    mkdir build
    cd ./build
    cmake -DCMAKE_EXPORT_COMPILE_COMMANDS=1 ..
    cd ../
    exit 0
fi

if [ $1 == "build" ]; then
    cd ./build
    cmake --build .
    cd ../
    exit 0
fi

if [ $1 == "run" ]; then
    cd ./build/src
    ./program
    exit 0
fi

echo "Nothing done"