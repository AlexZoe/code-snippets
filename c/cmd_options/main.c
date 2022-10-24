#include <stdio.h>

#include "program_options.h"

struct CmdOptions {
    int val;
};


int main(int argc, char** argv)
{
    struct CmdOptions opt = {0};
    register_option_int('a', "arg", &(opt.val), "set value");
    parse_program_options(argc, argv);

    printf("has debug: %i\n", is_debug());
    printf("val: %i\n", opt.val);

    return 0;
}
