#include <getopt.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "common_options.h"

#define MAX_ENTRIES 256

// Should be set by built system
#ifndef VERSION
#define VERSION "1.0.0"
#endif

static char* g_shared_short_options = "dhv";
static struct option g_shared_long_options[] = {
        {"debug", no_argument, NULL, 'd'},
        {"help", no_argument, NULL, 'h'},
        {"version", no_argument, NULL, 'v'},
        {NULL, 0, NULL, 0}};

static int g_debug_set = 0;

static const char* get_program_name(const char* program_name);
static void print_help_message(const char* program_name);

int is_debug()
{
    return g_debug_set;
}

void init(int argc, char** argv, struct UserCmdOptions* user_options,
          void* user_data)
{
    int opt;
    int option_idx;

    int num_user_args = 0;
    char* user_flags = NULL;

    int num_shared_args =
            sizeof(g_shared_long_options) / sizeof(g_shared_long_options[0]);
    if (user_options)
    {
        user_flags = user_options->flags;
        // Cut off arguments that would exceed the maximum number of entries
        num_user_args = user_options->num_arg_entries;
        if (num_user_args + num_shared_args > MAX_ENTRIES)
        {
            num_user_args = MAX_ENTRIES - num_shared_args;
        }
    }

    // Short commandline arguments to look for, e.g. '-h'
    char short_options[MAX_ENTRIES];
    snprintf(short_options, sizeof(short_options), "%s%s", user_flags,
             g_shared_short_options);

    // Commandline arguments to look for, e.g. '--help'
    struct option long_options[MAX_ENTRIES];
    for (int entry = 0; entry < num_user_args; ++entry)
    {
        memcpy(&long_options[entry], &user_options->args[entry],
               sizeof(struct option));
    }
    for (int entry = 0; entry < num_shared_args; ++entry)
    {
        memcpy(&long_options[num_user_args + entry],
               &g_shared_long_options[entry], sizeof(struct option));
    }

    while ((opt = getopt_long(argc, argv, short_options, long_options,
                              &option_idx)) != -1)
    {
        switch (opt)
        {
            // Long options set the appropriate field in short_options
            // automatically
            case 0:
            {
                break;
            }

            case 'd':
            {
                g_debug_set = 1;
                break;
            }

            case 'h':
            {
                print_help_message(get_program_name(argv[0]));
                exit(EXIT_SUCCESS);
            }

            case 'v':
            {
                printf("%s %s\n", get_program_name(argv[0]), VERSION);
                exit(EXIT_SUCCESS);
            }

            default:
            {
                if (user_options->parser_func)
                {
                    user_options->parser_func(opt, optarg,
                                              user_options->user_data);
                }
                break;
            }
        }
    }
}

static const char* get_program_name(const char* program_name)
{
    const char *exec_name = strrchr(program_name, '/');

    return (exec_name) ? ++exec_name : program_name;
}

static void print_help_message(const char* program_name)
{
    printf("Usage: %s [OPTION]\n\n", program_name);
    printf("OPTION:\n");
    printf("  -d,--debug             \t\tenable debug output\n");
    printf("  -h,--help              \t\tshow this message and exit\n");
    printf("  -v,--version           \t\tprint version number and exit\n");
    print_prog_help();
}
