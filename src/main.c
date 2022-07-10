#include "rust_interop.h"

#include <stdio.h>

int main() {
  printf("get_int(): %d\n", get_int());
  
  char s[100];
  int len = get_string(s, 100);
  printf("get_string(): %s, len=%d\n", s, len);
  
  len = get_string(s, 10);
  printf("get_string(): %s, len=%d\n", s, len);

  return 0;
}

