#include "camera.hpp"

#include <iostream>

#include "pimpl_impl.h"

class Camera::Impl {
 public:
  Impl(const Camera::Config& config) {
    std::cout << "name: " << config.name << "\n";
    setup();
  }
  ~Impl() { shutdown(); }

  void setup() { std::cout << "setup\n"; }

  void shutdown() { std::cout << "shutdown\n"; }
};

Camera::Camera(const Camera::Config& config) : impl_(config) {}

Camera::~Camera() {}

bool Camera::open() {
  impl_->setup();
  return true;
}

void Camera::close() { impl_->shutdown(); }
