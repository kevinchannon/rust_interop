#include <stdio.h>

extern int get_int();
extern int get_string(char* buffer, unsigned int buffer_size);

int main() {
  printf("get_int(): %d\n", get_int());

  return 0;
}

