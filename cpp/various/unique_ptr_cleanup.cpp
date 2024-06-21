#include <functional>
#include <iostream>
#include <memory>

struct Some {
  int a{0};
};

Some* create() { return new Some; }

void destroy(Some* some, void* other) {
  std::cout << "destroy\n";
  delete some;
}

int main() {
  auto del_some = [](Some* some) { destroy(some, nullptr); };
  std::unique_ptr<Some, void (*)(Some*)> ptr(create(), del_some);
}
