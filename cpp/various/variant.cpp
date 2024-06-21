#include <iostream>
#include <variant>

int main() {
  std::variant<unsigned int, float> r_int;
  r_int = 3.5f;
  try {
    std::cout << "int: " << std::get<unsigned int>(r_int) << "\n";
  } catch (const std::bad_variant_access& ex) {
    std::cout << ex.what() << '\n';
  }
}
