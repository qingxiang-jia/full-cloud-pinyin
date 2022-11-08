#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>

extern uint32_t r_add(uint32_t, uint32_t);

int main(void) {
  uint32_t sum = r_add(1, 2);
  printf("%d\n", sum);
}
