#include <thread>
#include <iostream>

class MyClass {
 public:
    void myPublicFunc(int arg) {
        std::thread th1 = std::thread(&MyClass::myPrivateFunc, this, arg);
        th1.join();

        std::thread th2 = std::thread(myStaticPrivateFunc, arg);
        th2.join();
    }

 private:
    void myPrivateFunc(int arg) {
        std::cout << "Arg is: " << arg << std::endl;
    }
    static void myStaticPrivateFunc(int arg) {
        std::cout << "Arg is: " << arg << std::endl;
    }
};

int main() {
    MyClass inst;
    inst.myPublicFunc(5);

    return 0;
}
