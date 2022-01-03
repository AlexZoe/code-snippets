#include "container.hpp"

#include <iostream>
#include <map>
#include <string>
#include <vector>


NumMaxContainer::NumMaxContainer(uint32_t vals_to_keep) {
  vals_to_keep_ = vals_to_keep;
  val_count_ = 0;
}


void MapNumMaxContainer::storeIdForNumMaxVals(uint32_t val, std::string& id) {
  // Check if the key has already an identifier
  auto insert_candidate = storage_.find(val);
  if (insert_candidate != storage_.end()) {
    // Push to underlying vector if key is already present
    insert_candidate->second.push_back(id);
  } else {
    std::vector<std::string> tmp {id};
    // Otherwise, create a new node
    storage_.insert(std::pair<uint32_t, std::vector<std::string>>(val, tmp));
  }
  // Increase number of currently stored identifiers (Note: size() of map cannot
  // be used here)
  this->val_count_++;

  if (this->val_count_ > this->vals_to_keep_) {
    auto remove_candidate = storage_.begin();
    if (remove_candidate->second.size() > 1) {
      remove_candidate->second.pop_back();
    } else {
      storage_.erase(remove_candidate->first);
    }
    this->val_count_--;
  }
}


void MapNumMaxContainer::printIdsOfNumMaxVals() {
  for (auto it: storage_) {
    for (auto elem: it.second) {
      std::cout << elem << std::endl;
    }
  }
}
