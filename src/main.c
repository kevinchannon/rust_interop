#include "rust_interop.h"

#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>

void print_user(bool success, User* user) {
  if (success) {
    printf("Found user with ID %lu\n", user->id);
  } else {
    printf("FATAL: Failed to find user\n");
    exit(1);
  }

}

int main() {
  printf("get_int(): %d\n", get_int());
  
  char s[100];
  int len = fill_string(s, 100);
  printf("fill_string(): %s, len=%d\n", s, len);
  
  len = fill_string(s, 10);
  printf("fill_string(): %s, len=%d\n", s, len);

  User user;
  print_user(get_user(1, &user), &user);
  print_user(get_user(100, &user), &user);
  
  return 0;
}

