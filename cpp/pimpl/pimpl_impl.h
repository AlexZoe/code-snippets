#ifndef PIMPL_IMPL_H
#define PIMPL_IMPL_H

#include <iostream>
#include <memory>
#include <utility>

#include "pimpl.h"

template <typename T>
Pimpl<T>::Pimpl() : m{std::make_unique<T>()} {
  std::cout << "no args\n";
}

template <typename T>
template <typename... Args>
Pimpl<T>::Pimpl(Args&&... args)
    : m{std::make_unique<T>(std::forward<Args>(args)...)} {
  std::cout << "forwarded\n";
}

template <typename T>
Pimpl<T>::~Pimpl() {}

template <typename T>
T* Pimpl<T>::operator->() {
  return m.get();
}

template <typename T>
T& Pimpl<T>::operator*() {
  return *m.get();
}

#endif  // PIMPL_IMPL_H
