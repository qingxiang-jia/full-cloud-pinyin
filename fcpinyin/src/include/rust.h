#pragma once

#include <cstdint>
#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>

/* ↓ CALLBACK TYPES */
typedef void (*FnCommit)(uint16_t);
typedef void (*FnVoid)();
typedef void (*FnSetCandidates)(int16_t **candidates, size_t cnt);
typedef void (*FnSetPreedit)(char *preedit);
/* ↑ CALLBACK TYPES */

/* ↓ C++ -> Rust */
extern "C" bool on_key_press(uint16_t);
/* ↑ C++ -> Rust */

/* ↓ Rust -> C++ */
/* ↓ CALLBACK REGISTRATION */
extern "C" void
register_callbacks(FnVoid set_loading, FnSetCandidates set_candidates,
                   FnSetCandidates append_candidates, FnSetPreedit set_preedit,
                   FnVoid page_up, FnVoid page_down, FnVoid prev, FnVoid next,
                   FnCommit commit, FnVoid commit_candidate_by_fixed_key);
/* ↑ CALLBACK REGISTRATION */

/* ↓ TEST CALLBACK PASSING */
extern "C" void register_fn_commit(FnCommit callback);
extern "C" void register_fn_void(FnVoid callback);
extern "C" void register_fn_set_candidates(FnSetCandidates callback);
extern "C" void register_fn_set_preedit(FnSetPreedit callback);
/* ↑ TEST CALLBACK PASSING */

/* ↓ UI */
extern "C" void set_loading();
extern "C" void set_candidates(int16_t **candidates, size_t cnt);
extern "C" void append_candidates(int16_t **candidates, size_t cnt);
extern "C" void clear_candidates();
extern "C" void set_preedit(char *preedit);
/* ↑ UI */

/* ↓ TABLE */
extern "C" bool can_page_up();
extern "C" void page_up();
extern "C" void page_down();
extern "C" void prev();
extern "C" void next();
/* ↑ TABLE */

/* ↓ ENGINE */
extern "C" void commit(uint16_t idx);
extern "C" void commit_candidate_by_fixed_key();
extern "C" void commit_preedit(char *preedit);
/* ↑ ENGINE */
/* ↑ Rust -> C++ */