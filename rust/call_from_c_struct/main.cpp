#include <iostream>

struct Coordinate {
    int x;
    int y;
};


extern "C" void share_struct_by_val(struct Coordinate coord);
extern "C" void share_struct_by_ptr(struct Coordinate *coord);

void call_initialized_val() {
    struct Coordinate coord;
    coord.x = 4;
    coord.y = 2;

    share_struct_by_val(coord);
}

/*
The compiler won't complain here but runtime analysis via valgrind will show
that rust complains about uninitialized data.
 */
void call_uninitialized_val() {
    struct Coordinate coord;
    share_struct_by_val(coord);
}

void call_initialized_ptr() {
    struct Coordinate coord;
    coord.x = 8;
    coord.y = 3;
    share_struct_by_ptr(&coord);
}

int main() {
    call_uninitialized_val();
    call_initialized_val();

    call_initialized_ptr();

    return 0;
}
