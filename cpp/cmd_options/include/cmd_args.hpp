#ifndef CMD_ARGS_HPP_
#define CMD_ARGS_HPP_

#include <string>

namespace args {
  void init(int argc, char* argv[]);

  bool hasOption(const std::string& name);

  template <typename T>
  T getOption(const std::string& name);
}

#endif  // CMD_ARGS_HPP_
