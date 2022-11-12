#pragma once
#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>

enum Key {
  A,
  B,
  C,
  D,
  E,
  F,
  G,
  H,
  I,
  J,
  K,
  L,
  M,
  N,
  O,
  P,
  Q,
  R,
  S,
  T,
  U,
  V,
  W,
  X,
  Y,
  Z,
  CTRL,
  SHIFT,
  ESC,
  SPACE,
  ENTER,
  BACKSPACE
};

void c_print_u32(uint32_t);

void key(enum Key key);

void commit(uint8_t idx);

void pageUp();

void pageDown();

// preedit should be a readonly pointer, it will be freed by Rust side
// the same is true for candidates and lens
// candidates points to an array of UTF-8 encoded CString
void setState(char *preedit, int8_t **candidates, uint8_t *lens, size_t cnt);
