#include <iostream>
#include <memory>

#include "libconfig.h++"

namespace lib = libconfig;

[[nodiscard]] std::unique_ptr<lib::Config> load_file(std::string &&path) {
  std::unique_ptr<lib::Config> cfg = std::make_unique<lib::Config>();
  try {
    cfg->readFile(path.c_str());
  } catch (const lib::FileIOException &fioex) {
    std::cerr << "I/O error while reading file." << std::endl;
    return {};
  } catch (const lib::ParseException &pex) {
    std::cerr << "Parse error at " << pex.getFile() << ":" << pex.getLine()
              << " - " << pex.getError() << std::endl;
    return {};
  }

  return cfg;
}

int main() {
  auto cfg = std::move(load_file(std::string("../Input.cfg")));

  if (!cfg) return 1;

  const lib::Setting &root = cfg->getRoot();

  try {
    const lib::Setting &arr = root["Message"]["byte_array"];

    std::cout << "array?: " << arr.isArray() << "\n";
    std::cout << "length: " << arr.getLength() << "\n";
    for (auto &byte : arr) {
      std::cout << "byte: " << static_cast<int>(byte) << "\n";
    }

  } catch (const lib::SettingNotFoundException &nfex) {
    std::cerr << "Setting not found\n";
  }

  return 0;
}
