#include <iostream>
extern "C" void print_string_with_string(char *string, int size);
extern "C" void print_string_with_u8(const char *string, int size);
extern "C" void print_float_array(const float *f, int size);
extern "C" void print_float(float f);

void print_string_v1() {
    char c[] = "hello rust\0";
    print_string_with_string(c, sizeof(c));
}

void print_string_null_ptr() {
    print_string_with_string(NULL, 2);
}

void print_string_out_of_bounds() {
    char c[] = "hello rust\0";
    printf("C thinks string lengths is %li\n", sizeof(c));
    print_string_with_string(c, sizeof(c)+2);
}

void print_string_v2() {
    char c[] = "hello rust\0";
    print_string_with_u8(c, sizeof(c));
}

void print_float_v1() {
    float f = 3.14;
    print_float_array(&f, 1);
}

void print_float_v2() {
    float f = 3.14;
    print_float(f);
}

void print_native(const char* string, int size) {
    std::cout << string << std::endl;
}

void call_native() {
    char c[] = "hello c\0";
    print_native(c, sizeof(c));
}

int main() {
    print_string_v1();
    print_string_out_of_bounds();
    print_string_null_ptr();
//    print_string_v2();
//    call_native();
//    print_float_v1();
    print_float_v2();

    return 0;
}
