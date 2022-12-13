#include <cstring>
#include <string>
#include <unordered_map>
#include <vector>

#include "database.h"

std::unordered_map<std::string, std::vector<char>> g_mock_db;

struct DBContext {};

struct DBContext* open_database() {
  return NULL;
}

void close_database(struct DBContext* ctx) {}

int set_item(struct DBContext* ctx, char* key, unsigned int size, void* value) {
  std::vector<char> container(size);
  memcpy(container.data(), value, size);
  std::string map_key(key);

  auto entry = g_mock_db.find(map_key);
  if (entry != g_mock_db.end()) {
    entry->second = container;
  } else {
    g_mock_db.insert({map_key, container});
  }
  return 0;
}

int get_item(struct DBContext* ctx, char* key, unsigned int size, void* value) {
  std::string map_key(key);

  auto entry = g_mock_db.find(map_key);
  if (entry != g_mock_db.end() && (entry->second.size() == size)) {
    memcpy(value, entry->second.data(), size);
    return 0;
  }
  return 1;
}
