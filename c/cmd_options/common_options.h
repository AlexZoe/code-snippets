#ifndef COMMON_OPTIONS_H
#define COMMON_OPTIONS_H

// Add additional commandline flags/arguments
struct UserCmdOptions {
    char* flags;
    struct option* args;
    int num_arg_entries;
    void (*parser_func)(char opt, char* optarg, void* user_data);
    void* user_data;
};

// To be defined by user application if it adds additional commandline options
extern void print_prog_help();

// Sets up environment and parses commandline arguments
void init(int argc, char** argv, struct UserCmdOptions* user_options,
          void* user_data);

int is_debug();

#endif // COMMON_OPTIONS_H
