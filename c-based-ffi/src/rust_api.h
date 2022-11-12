#pragma once
#include <inttypes.h>
#include <stdint.h>

extern uint32_t r_add(uint32_t, uint32_t);

typedef void (*callback)(uint32_t);

typedef void (*FnCommit)(uint8_t);

typedef void (*FnVoid)();

typedef void (*FnSetState)(char *, int8_t **, uint8_t *, size_t);

extern void r_run_callbacks(FnCommit, FnVoid, FnVoid, FnSetState);

extern uint32_t r_add_cb(uint32_t, uint32_t, callback cb);