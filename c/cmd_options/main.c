#include <getopt.h>
#include <stdio.h>
#include <stdlib.h>

#include "common_options.h"

struct CmdOptions {
    int val;
};

static struct option long_options[] = {{"arg", required_argument, NULL, 'a'}};
static char* short_options = "a:";

void print_prog_help()
{
    printf("  -a,--arg INT           \t\tset value\n");
}

void parse_cmd_options(char opt, char* optarg, void* user_data)
{
    struct CmdOptions* options = (struct CmdOptions*)user_data;

    switch (opt)
    {
        case 'a':
        {
            options->val = atoi(optarg);
            break;
        }

        default:
        {
            break;
        }
    }
}

void setup_user_options(struct UserCmdOptions* user_options,
                        struct CmdOptions* options)
{
    user_options->flags = short_options;
    user_options->args = long_options;
    user_options->num_arg_entries =
            sizeof(long_options) / sizeof(long_options[0]);
    user_options->parser_func = parse_cmd_options;
    user_options->user_data = options;
}

int main(int argc, char** argv)
{
    struct CmdOptions options = {0};
    struct UserCmdOptions user_options;

    setup_user_options(&user_options, &options);

    init(argc, argv, &user_options, &options);

    printf("has debug: %i\n", is_debug());
    printf("val: %i\n", options.val);

    return 0;
}
