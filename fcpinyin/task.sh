#!/usr/bin/env bash

if [ $# == 0 ]; then 
    echo "No argument passed in"
    exit 0
fi

# If we specify cdylib or staticlib in Cargo.toml, the main.rs can't properly reference contents in lib.rs.
if [ $1 == "gen-lib" ]; then
    cargo -Z unstable-options rustc --lib --release --crate-type staticlib -- --print native-static-libs
    exit 0
fi

if [ $1 == "gen-cxx" ]; then
    cxxbridge src/core/ffi.rs > ffi.cc
    cxxbridge src/core/ffi.rs --header > ffi.h
    exit 0
fi

echo "Nothing done"