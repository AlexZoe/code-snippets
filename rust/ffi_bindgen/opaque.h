#ifndef OPAQUE_H
#define OPAQUE_H

#ifdef __cplusplus
extern "C" {
#endif

struct Opaque;

struct Opaque* make_new_opaque();
void print_name(struct Opaque* o);
void delete_opaque(struct Opaque* o);

#ifdef __cplusplus
}
#endif

#endif // OPAQUE_H
