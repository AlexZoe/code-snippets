#include <iostream>
#include <string>
#include <fstream>

#include "boost/tokenizer.hpp"

#include "cmd_args.hpp"

int main(int argc, char* argv[]) {
  std::string line;

  args::init(argc, argv);

  if (args::hasOption("file")) {
    std::ifstream stream;
    stream.open(args::getOption<std::string>("file"), std::ios::in);

    if (stream.is_open()) {
      while(getline(stream, line)) {
        std::cout << "val: " << line << std::endl;
      }
    }
  } else {
    uint32_t id, num;
    boost::char_separator<char> delim{" ", "\t"};
    while (getline(std::cin, line)) {
      if (line.empty())
        break;
      std::cout << "got: " << line << std::endl;
      boost::tokenizer<boost::char_separator<char>> tokens{line, delim};
      if (std::distance(tokens.begin(), tokens.end()) != 2) {
        break;
      } else {
        auto it = tokens.begin();
        id = std::atoi(it->c_str());
        num = std::atoi((++it)->c_str());
        std::cout << "val: " << id << " " << num << std::endl;
      }
    }
  }
  return 0;
}
