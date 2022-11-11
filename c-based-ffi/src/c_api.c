#include "c_api.h"
#include <stdint.h>
#include <stdio.h>

void c_print_u32(uint32_t n) { printf("N=%d\n", n); }

void key(enum Key key) {
  switch (key) {
  case A:
    printf("A\n");
    break;
  case B:
    printf("B\n");
    break;
  case C:
    printf("C\n");
    break;
  case D:
    printf("D\n");
    break;
  case E:
    printf("E\n");
    break;
  case F:
    printf("F\n");
    break;
  case G:
    printf("G\n");
    break;
  case H:
    printf("H\n");
    break;
  case I:
    printf("I\n");
    break;
  case J:
    printf("J\n");
    break;
  case K:
    printf("K\n");
    break;
  case L:
    printf("L\n");
    break;
  case M:
    printf("M\n");
    break;
  case N:
    printf("N\n");
    break;
  case O:
    printf("O\n");
    break;
  case P:
    printf("P\n");
    break;
  case Q:
    printf("Q\n");
    break;
  case R:
    printf("R\n");
    break;
  case S:
    printf("S\n");
    break;
  case T:
    printf("T\n");
    break;
  case U:
    printf("U\n");
    break;
  case V:
    printf("V\n");
    break;
  case W:
    printf("W\n");
    break;
  case X:
    printf("X\n");
    break;
  case Y:
    printf("Z\n");
    break;
  case Z:
    printf("CTRL\n");
    break;
  case CTRL:
    printf("CTRL\n");
    break;
  case SHIFT:
    printf("SHIFT\n");
    break;
  case ESC:
    printf("ESC\n");
    break;
  case SPACE:
    printf("SPACE\n");
    break;
  case ENTER:
    printf("ENTER\n");
    break;
  case BACKSPACE:
    printf("BACKSPACE\n");
    break;
  }
}

void commit(uint8_t idx) { printf("commit(%d)\n", idx); }

void pageUp() { printf("pageUp()\n"); }

void pageDown() { printf("pageDown()\n"); }

void setState(char *preedit, int8_t **candidates, uint8_t *lens, uint16_t cnt) {
  printf("Setting state BEGIN\n");

  printf("preedit: %s\n", preedit);
  printf("candidates address is %p\n", candidates);

  for (uint16_t i = 0; i < cnt; i++) {
    printf("candidate matching length: %d\n", lens[i]);
  }
  
  printf("Setting state END\n");
}