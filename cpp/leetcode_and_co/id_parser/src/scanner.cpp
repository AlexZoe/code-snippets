#include "scanner.hpp"

#include <fstream>
#include <iostream>
#include <string>

#include "boost/tokenizer.hpp"

#include "container.hpp"


void Scanner::extractIdValuePair(std::string& line, NumMaxContainer* storage) {
  boost::char_separator<char> delim{" ", "\t"};
  boost::tokenizer<boost::char_separator<char>> tokens{line, delim};
  uint32_t val;
  std::string id;

  if (std::distance(tokens.begin(), tokens.end()) != 2) {
    return;
  } else {
    auto it = tokens.begin();
    id = *it;
    val = std::atoi((++it)->c_str());
    storage->storeIdForNumMaxVals(val, id);
  }
}


FileScanner::FileScanner(const std::string& file) {
  file_ = file;
}


void FileScanner::scan(NumMaxContainer* storage) {
  std::ifstream stream;
  std::string line;

  if (!storage) return;

  stream.open(file_, std::ios::in);
  if (stream.is_open()) {
    while(getline(stream, line)) {
      extractIdValuePair(line, storage);
    }
  }
  stream.close();
}


void StdInScanner::scan(NumMaxContainer* storage) {
  std::string line;

  if (!storage)
    return;

  while (getline(std::cin, line)) {
    if (line.empty()) break;

    extractIdValuePair(line, storage);
  }
}
