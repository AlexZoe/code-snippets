#ifndef DATABASE_H
#define DATABASE_H

#ifdef __cplusplus
extern "C" {
#endif

struct DBContext* open_database();
void close_database(struct DBContext* ctx);

int set_item(struct DBContext* ctx, char* key, unsigned int size, void* value);
int get_item(struct DBContext* ctx, char* key, unsigned int size, void* value);

#ifdef __cplusplus
}
#endif

#endif // DATABASE_H
