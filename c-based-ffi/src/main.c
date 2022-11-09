#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>
#include "rust_api.h"

int main(void) {
  uint32_t sum = r_add(1, 2);
  printf("%d\n", sum);
}
