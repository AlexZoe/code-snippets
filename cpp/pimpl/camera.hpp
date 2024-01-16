#ifndef CAMERA_HPP
#define CAMERA_HPP

#include <string>

#include "pimpl.h"

class Camera {
 public:
  struct Config {
    std::string name;
  };

 public:
  explicit Camera(const Config& config);
  ~Camera();

  bool open();
  void close();

 private:
  class Impl;
  Pimpl<Impl> impl_;
};

#endif  // CAMERA_HPP
