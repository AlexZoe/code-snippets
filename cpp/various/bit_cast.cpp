#include <iostream>
#include <bit>

int main() {
  float int_a = 3.5f;

  std::cout << "int: " << std::bit_cast<int>(int_a) << "\n";
}

