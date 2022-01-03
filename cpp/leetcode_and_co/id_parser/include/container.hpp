#ifndef CONTAINER_HPP_
#define CONTAINER_HPP_

#include <map>
#include <string>
#include <vector>

/**
 *  \class NumMaxContainer
 *  \brief Abstract class to serve as interface for trying out different data
 *         structures for storing value-id pairs.
 *
 *  Only the top N (vals_to_keep) number of identifiers are stored within the
 *  container. A single 'val' (or key) may be associated with multiple
 *  identifiers.
 */
class NumMaxContainer {
 public:
  NumMaxContainer(uint32_t vals_to_keep);
  virtual ~NumMaxContainer() = default;

  /**
   *  \brief Try to store value-identifier pair if it is part of the top N.
   *         Replace smaller value-identifier pairs if necessary
   *
   *  \param[in]  val   The value to compare to decide if it is within the top N
   *  \param[in]  id    The corresponding value to be extracted later
   */
  virtual void storeIdForNumMaxVals(uint32_t val, std::string& id) = 0;

  /**
   *  \brief Print all stored identifiers.
   *
   *  Note that only up to N ('vals_to_keep') identifiers are kept within the
   *  data structure.
   */
  virtual void printIdsOfNumMaxVals() = 0;

 protected:
  // Keep identifiers of the N largest keys (value)
  uint32_t vals_to_keep_;
  // Current number of stored identifiers (a single key might have multiple
  // identifier so size() of the underlying container does not work)
  uint32_t val_count_;
};

/**
 *  \brief Container class using std::map to store value-id pairs
 *
 *  'val' passed to storeIdForNumMaxVals() is used as key whereas 'id' is the
 *  associated value. Ids are stored within a std::vector to accomodate multiple
 *  'id's for a single 'val'. If a key has multiple values, new ones are
 *  inserted at the end of the vector. Similarly, values are removed from the
 *  end of the vector if necessary. Thus, the first seen 'id' is replaced last.
 *  If the vector only holds one item, the whole key-value pair is replaced.
 */
class MapNumMaxContainer : public NumMaxContainer {
 public:
  MapNumMaxContainer(uint32_t vals_to_keep) : NumMaxContainer(vals_to_keep) {}
  ~MapNumMaxContainer() = default;

  void storeIdForNumMaxVals(uint32_t val, std::string& id) override;
  void printIdsOfNumMaxVals() override;

 private:
  std::map<uint32_t, std::vector<std::string>> storage_;
};

#endif  // CONTAINER_HPP_
