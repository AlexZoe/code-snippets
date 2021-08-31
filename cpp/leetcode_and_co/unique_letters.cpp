/*
 * Analyze a given string of unknown length if it contains all letters
 * of the alphabet.
 */
#include <algorithm>
#include <iostream>
#include <string>

static bool containsAllUniqueLetters(const std::string& eval_string);

int main(int argc, char** argv) {
  bool ret;
  if (argc) {
    ret = containsAllUniqueLetters(std::string(argv[1]));
  } else {
    return 0;
  }

  if (ret) {
    std::cout << "contains all letters" << std::endl;
  } else {
    std::cout << "missing letters" << std::endl;
  }
  return 0;
}

bool containsAllUniqueLetters(const std::string& eval_string) {
  bool seen_letters[26] = {false};
  int unique_count = 0;
  int ascii_first_letter_start_offset = 97;  // offset of ASCII 'a'
  std::string eval_string_lowercase = eval_string;
  std::transform(eval_string_lowercase.begin(), eval_string_lowercase.end(),
                 eval_string_lowercase.begin(),
                 [](unsigned char c) { return std::tolower(c); });

  for (auto elem : eval_string_lowercase) {
    int letter_offset =
        static_cast<int>(elem) - ascii_first_letter_start_offset;
    if (!seen_letters[letter_offset]) {
      unique_count++;
      seen_letters[letter_offset] = true;
    }
  }

  std::cout << "count is: " << unique_count << std::endl;
  return unique_count == 26;
}
