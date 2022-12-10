#pragma once

#include <cstdint>
#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>

/* ↓ CALLBACK TYPES */
typedef void (*FnCommit)(uint16_t);
typedef void (*FnVoid)();
typedef bool (*FnCanPageUp)();
typedef void (*FnSetCandidates)(int16_t **candidates, size_t cnt);
typedef void (*FnSetPreedit)(char *preedit);
/* ↑ CALLBACK TYPES */

/* ↓ C++ -> Rust */
typedef struct FcpOpaque {
} FcpOpaque;

extern "C" const FcpOpaque* new_fcp();
extern "C" void register_callbacks(
    FcpOpaque *opaque, FnVoid set_loading, FnSetCandidates set_candidates,
    FnSetCandidates append_candidates, FnVoid clear_candidates,
    FnSetPreedit set_preedit, FnCanPageUp can_page_up, FnVoid page_up,
    FnVoid page_down, FnVoid prev, FnVoid next, FnCommit commit,
    FnSetPreedit commit_preedit, FnVoid commit_candidate_by_fixed_key);
extern "C" bool on_key_press(uint16_t);
/* ↑ C++ -> Rust */

/* ↓ Rust -> C++ */
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