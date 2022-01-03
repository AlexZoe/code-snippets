#ifndef SCANNER_HPP_
#define SCANNER_HPP_

#include <string>

// Forward declaration (avoid unnecessarily spilling includes)
class NumMaxContainer;

/**
 *  \class Scanner
 *  \brief Abstract interface class used for implementing different ways to
 *  obtain value-identifier pairs.
 */
class Scanner {
 public:
  virtual ~Scanner() = default;
  /**
   *  \brief Scan for value-identifier pairs and add them to the storage
   *
   *  \param[in] storage  Pointer to the storage in which to store (value-)
   *                      idenifiers
   */
  virtual void scan(NumMaxContainer* storage) = 0;

 protected:
  // Extract value-identifier pairs from a line (obtained from e.g. file)
  virtual void extractIdValuePair(std::string& line, NumMaxContainer* storage);
};

/**
 *  \class FileScanner
 *  \brief Concrete implementation of 'Scanner' which obtains value-identifier
 *         pairs from a file.
 */
class FileScanner : public Scanner {
 public:
  explicit FileScanner(const std::string& file);
  void scan(NumMaxContainer* storage) override;

 private:
  std::string file_;
};

/**
 *  \class StdInScanner
 *  \brief Concrete implementation of 'Scanner' which obtains value-identifier
 *         pairs from a stdin stream.
 */
class StdInScanner : public Scanner {
 public:
  void scan(NumMaxContainer* storage) override;
};

#endif  // SCANNER_HPP_
