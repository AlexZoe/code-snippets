#include "opaque.h"

#include <stdio.h>
#include <stdlib.h>

struct Opaque {
  char name[256];
};

struct Opaque* make_new_opaque() {
  struct Opaque* o = malloc(sizeof(struct Opaque));
  snprintf(o->name, sizeof(o->name), "my opaque");
  return o;
}

void print_name(struct Opaque* o) {
  if (o) {
    printf("%s\n", o->name);
  }
}

void delete_opaque(struct Opaque* o) {
  if (o) free(o);
}
