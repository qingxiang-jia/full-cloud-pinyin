#!/usr/bin/env bash

if [ $# == 0 ]; then 
    echo "No argument passed in"
    exit 0
fi

# If we specify cdylib or staticlib in Cargo.toml, the main.rs can't properly reference contents in lib.rs.
if [ $1 == "gen-lib" ]; then
    cargo rustc --lib --release --crate-type cdylib
    exit 0
fi

if [ $1 == "gen-cxx" ]; then
    cxxbridge src/ffi.rs > ffi.cc
    cxxbridge src/ffi.rs --header > ffi.h
    exit 0
fi

echo "Nothing done"