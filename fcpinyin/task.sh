#!/usr/bin/env bash

if [ $# == 0 ]; then 
    echo "No argument passed in"
    exit 0
fi

# If we specify staticlib in Cargo.toml, the main.rs can't properly reference contents in lib.rs.
if [ $1 == "gen-staticlib" ]; then
    cargo rustc --lib --release --crate-type staticlib
    exit 0
fi

if [ $1 == "gen-headers" ]; then
    cxxbridge src/ffi.rs > ffi.rs.h
    exit 0
fi

echo "Nothing done"