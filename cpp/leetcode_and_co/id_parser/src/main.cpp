#include <string>

#include "boost/tokenizer.hpp"

#include "cmd_args.hpp"
#include "container.hpp"
#include "scanner.hpp"


int main(int argc, char* argv[]) {
  args::init(argc, argv);

  MapNumMaxContainer storage(args::getOption<uint32_t>("items"));

  if (args::hasOption("file")) {
    FileScanner scanner(args::getOption<std::string>("file"));
    scanner.scan(&storage);
  } else {
    StdInScanner scanner;
    scanner.scan(&storage);
  }

  storage.printIdsOfNumMaxVals();

  return 0;
}
