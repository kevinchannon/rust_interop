#include "rust_interop.h"

#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>

///////////////////////////////////////////////////////////////////////////////

int main() {
  UserHandle user_handle = add_user(1234);
  printf("Added user with handle: %lu\n", user_handle);
  
  ResultCode rc = set_user_name(user_handle, "Uluru");
  if ( RC_ERROR == rc) {
    printf("Failed to set user's name\n");
  }
  
  char s[100];
  rc = get_user_name(user_handle, s, 100);
  if ( RC_ERROR == rc) {
    printf("Failed to set user's name\n");
  } else {
    printf("User's name set to %s\n", s);
  }
  
  return 0;
}

///////////////////////////////////////////////////////////////////////////////

