#pragma once

#include <cstdint>
#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>

/* BEGIN CALLBACK TYPES */
typedef void (*FnCommit)(uint16_t);
typedef void (*FnVoid)();
typedef void (*FnSetCandidates)(int16_t **candidates, size_t cnt);
typedef void (*FnSetPreedit)(char *preedit);
/* EDN CALLBACK TYPES */

/* BEGIN C++ -> Rust */
extern "C" void on_key_press(uint16_t);
/* END C++ -> Rust */

/* BEGIN TEST CALLBACK PASSING */
extern "C" void register_fn_commit(FnCommit callback);
extern "C" void register_fn_void(FnVoid callback);
extern "C" void register_fn_set_candidates(FnSetCandidates callback);
extern "C" void register_fn_set_preedit(FnSetPreedit callback);
/* END TEST CALLBACK PASSING */

/* BEGIN UI */
extern "C" void set_loading();
extern "C" void set_candidates(int16_t **candidates, size_t cnt);
extern "C" void append_candidates(int16_t **candidates, size_t cnt);
extern "C" void set_preedit(char *preedit);
/* END UI */

/* BEGIN TABLE */
extern "C" void page_up();
extern "C" void page_down();
extern "C" void prev();
extern "C" void next();
/* END TABLE */

/* BEGIN ENGINE */
extern "C" void commit(uint16_t idx);
extern "C" void commit_candidate_by_fixed_key();
/* END ENGINE */