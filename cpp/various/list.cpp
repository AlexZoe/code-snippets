#include <cstdio>
#include <forward_list>
#include <iostream>
#include <vector>

int main() {
  std::vector<int> v{5, 4, 2};
  std::forward_list<int> l{4, 5, 3};
  std::forward_list<int> l2{4, 5, 3};

  printf("v first: %p\n", &(*v.begin()));
  printf("v last: %p\n", &(*v.end()));
  printf("l first: %p\n", &(*l.begin()));
  printf("l last: %p\n", &(*l.end()));
  printf("l2 last: %p\n", &(*l2.end()));
  // if (l.end() == NULL) std::cout << "is null\n";
}
