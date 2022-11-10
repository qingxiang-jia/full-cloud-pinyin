#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>
#include "rust_api.h"
#include "c_api.h"

int main(void) {
  uint32_t sum = r_add_cb(12, 10, c_print_u32);
  printf("The sum is %d, end of main.\n", sum);
}
