#include "cmd_args.hpp"

#include <string>
#include <atomic>
#include <mutex>
#include <iostream>

#include "boost/program_options.hpp"


namespace po = boost::program_options;

namespace args {

static std::mutex mtx;
// Initialization with constructor is not atomic but should not matter here
static std::atomic<bool> inited(false);
static po::options_description desc("Program Options");
static po::variables_map vm;


void init(int argc, char* argv[]) {
  std::lock_guard<std::mutex> lock(mtx);
  // Check if program options have been initialized already
  if (inited.load())
    return;

  desc.add_options()
      ("help", "Parses identifier-value positive integer pairs \
       and returns the identifiers of the highest [items] number of values")
      ("file", po::value<std::string>(),
       "Optional input file to parse instead of stdin")
      ("items", po::value<uint32_t>()->default_value(5),
       "Number of values to print");
  // Parse and store cmd options
  try {
    po::store(po::parse_command_line(argc, argv, desc), vm);
  } catch(const po::unknown_option& e) {
    // Only ignore unrecognized options but escalate malformed ones
  }

  if (vm.count("help")) {
    std::cout << desc << std::endl;
    exit(0);
  }

  inited.store(true);
}


bool hasOption(const std::string& name) {
  return (vm.count(name));
}


template <typename T>
T getOption(const std::string& name) {
  // Return commandline option or default value of type if not found
  return (hasOption(name)) ? vm[name].as<T>() : T();
}


template uint32_t getOption<uint32_t>(const std::string&);
template std::string getOption<std::string>(const std::string&);

}  // namespace args
