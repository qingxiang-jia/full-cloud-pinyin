## Motivation

The current way of using CXX.rs to do FFI between C++ and Rust is good enough. But it's very inflexible. To accommodate this inflexibility, I was able to structure the entire project around it so only C++ to Rust direction is needed. But this means that majority of the code is written in C++ (which I want to minimize) and I had to use a hack to make it async (I would much rather use Rust's async).

To fix this, I want to restructure the project so C++ only does minimial work and use Rust's async. But this requires two-way communication (C++ calls Rust, Rust calls C++). Currently, CXX doesn't support callback functions which means if we want to achieve two-way communication, we need to have a separate set of FFI for the other direction. But because CXX puts all generated code in one file, we need to separate them otherwise it's very easy to get circular dependency on header files (forward declaration only takes you so far). I did the separation and it works. But requires so much work (parsing the generated header file) and using CXX brings more work than it saves.

## Goal

The idea is to expose key C++ APIs of Fcitx 5 to Rust and vice versa using C as FFI.

C (Fcitx 5) -> Rust (fcpinyin): the function that receives a key event.

Rust -> C:    
- the function that sets state (preedit, candidates)
- the function that commit a candidate or preedit
- the function that pages up the candidate list
- the function that pages down the candidate list

## Plan

The C->Rust part is done by:
- Rust FFI on Rust's side.
- Write a .h file to the API on Rust side.

The Rust-> part is done by callbacks.
- Rust defines all callbacks in Rust.
- Write a .h file that describes the function on Rust side that will take the callback.
- C defines all callbacks in C.
- C provides .h and .c of the concrete functions and assign them to the callbacks and call Rust side.