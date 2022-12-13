#include "database.h"

#include <stdint.h>
#include <stdio.h>

#define SOME_KEY "Some_Key_to_Store"
#define SOME_OTHER_KEY "Some_Key_to_Get"

static void existing_key() {
  struct DBContext* ctx = open_database();

  uint32_t val = 42;
  set_item(ctx, SOME_KEY, sizeof(val), &val);

  uint32_t get_val = 0;
  get_item(ctx, SOME_KEY, sizeof(get_val), &get_val);

  if (get_val == val) {
    printf("existing key ok\n");
  } else {
    printf("existing key error\n");
  }

  close_database(ctx);
}

static void missing_key() {
  struct DBContext* ctx = open_database();

  uint32_t get_val = 0;
  if (get_item(ctx, SOME_OTHER_KEY, sizeof(get_val), &get_val))
  {
    printf("missing key ok\n");
  } else {
    printf("missing key error\n");
  }

  close_database(ctx);
}

int main() {
  existing_key();
  missing_key();

  return 0;
}
