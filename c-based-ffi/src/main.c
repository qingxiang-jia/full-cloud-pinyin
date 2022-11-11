#include "c_api.h"
#include "rust_api.h"
#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>

int main(void) {
  uint32_t sum = r_add_cb(12, 10, c_print_u32);
  printf("The sum is %d, end of main.\n", sum);

  key(SPACE);

  commit(3);

  pageUp();

  pageDown();

  char *preedit = "abc";

  int8_t *candidates[4] = {
      (int8_t[]){228, 184, 128}, // 一
      (int8_t[]){233, 148, 174}, // 键
      (int8_t[]){228, 184, 137}, // 三 
      (int8_t[]){232, 191, 158}, // 连
  };

  uint8_t lens[] = {1, 2, 3, 4};

  uint16_t cnt = 4;

  setState(preedit, candidates, lens, cnt);
}
