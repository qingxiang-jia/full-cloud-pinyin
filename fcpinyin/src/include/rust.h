#pragma once

#include <cstdint>
#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>

typedef void (*FnCommit)(uint16_t);

typedef void (*FnVoid)();

typedef void (*FnSetCandidates)(int16_t **candidates, size_t cnt);

typedef void (*FnSetPreedit)(char *preedit);

extern "C" void on_key_press(uint16_t);

extern "C" void register_fn_commit(FnCommit callback);

extern "C" void register_fn_void(FnVoid callback);

extern "C" void register_fn_set_candidates(FnSetCandidates callback);

extern "C" void register_fn_set_preedit(FnSetPreedit callback);