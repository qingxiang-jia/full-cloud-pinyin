#pragma once
#include <inttypes.h>
#include <stdint.h>

extern uint32_t r_add(uint32_t, uint32_t);

typedef void (*callback)(uint32_t);

extern uint32_t r_add_cb(uint32_t, uint32_t, callback cb);