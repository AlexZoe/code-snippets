#include <cstring>
#include <iostream>

struct S {
  S(int val) : val_(val) { std::cout << "ctr with " << val_ << "\n"; }
  ~S() { std::cout << "dtr\n"; }
  void print_val() { std::cout << "val: " << val_ << "\n"; }

 private:
  int val_{0};
};

struct Base {
  virtual ~Base() = default;
  virtual void print_type() = 0;
};

struct Derived1 : public Base {
  Derived1(int a, int b) : a_(a), b_(b) { std::cout << "ctr Derived1\n"; }
  virtual ~Derived1() { std::cout << "dtr Derived1\n"; }
  Derived1& operator=(const Derived1& o) {
    std::cout << "cpy assgin dtr Derived1\n";
    this->a_ = o.a_;
    this->b_ = o.b_;
    return *this;
  }
  Derived1(const Derived1& o) {
    std::cout << "cpy dtr Derived1\n";
    this->a_ = o.a_;
    this->b_ = o.b_;
  }

  void print_type() override {
    std::cout << "Derived2 with " << a_ << ", " << b_ << "\n";
  }

 private:
  int a_{0};
  int b_{0};
};

struct Derived2 : public Base {
  Derived2(int a) : a_(a) { std::cout << "ctr Derived2\n"; }
  virtual ~Derived2() { std::cout << "dtr Derived2\n"; }

  void print_type() override { std::cout << "Derived2 with " << a_ << "\n"; }

 private:
  int a_{0};
};

Base& construct_obj(char* mem, int select) {
  if (select == 1) {
    // note:
    // the following line does not work since it assumes an already
    // initialized object at the target location; however, the vtable pointer of
    // the target memory location is set to '0' due to not being constructed
    // properly
    //*(reinterpret_cast<Derived1*>(mem)) = Derived1(1, 2);
    Derived1 d(1, 2);
    // warning:
    // https://stackoverflow.com/questions/37913814/when-memcpy-from-this-to-a-new-object-in-a-child-class-warning-destination-for
    // note: the actual type is copied here so this should be ok
    memcpy(mem, &d, sizeof(d));
  } else {
    *(reinterpret_cast<Derived2*>(mem)) = Derived2(3);
  }
  return *reinterpret_cast<Base*>(mem);
}

int main(int argc, char** argv) {
  if (argc < 2) return -1;

  char mem[32] = {0};

  // note: no vtable so the following is ok
  S* static_s = reinterpret_cast<S*>(mem);
  *static_s = S(5);
  static_s->print_val();

  std::cout << "size: " << sizeof(Derived1) << "\n";

  std::cout << "construct\n";
  Base& ref = construct_obj(mem, atoi(argv[1]));
  std::cout << "completed construct\n";
  ref.print_type();
}
