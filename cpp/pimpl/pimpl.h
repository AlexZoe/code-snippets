#ifndef PIMPL_H
#define PIMPL_H

#include <memory>

template <typename T>
class Pimpl {
 public:
  Pimpl();
  template <typename... Args>
  Pimpl(Args&&...);
  ~Pimpl();
  T* operator->();
  T& operator*();

 private:
  std::unique_ptr<T> m;
};

#endif  // PIMPL_H
