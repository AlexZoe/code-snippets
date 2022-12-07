#include "program_options.h"

#include <getopt.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define ALPHABET_LEN 26
#define NUM_LOWER_UPPER_LETTERS (ALPHABET_LEN * 2)

#define MAX_ENTRIES (NUM_LOWER_UPPER_LETTERS)
#define MAX_SHORT_OPTION_LEN (NUM_LOWER_UPPER_LETTERS * 2)

#define TERMINATOR_LEN 1

#define HELP_INDENT_LEVEL 30
#define LEFT_MARGIN 7

typedef void (*param_setter)(char* optarg, void* param, int param_len);

struct OptionContext {
    param_setter setter;
    void* param;
    int param_len;
    const char* help_string;
};

struct ProgOptState {
    struct option long_options[MAX_ENTRIES];
    struct OptionContext option_contexts[MAX_ENTRIES];
} g_state = {0};

static int g_debug_set = 0;

static const char ASCII_UPPER_LETTER_FIRST = 'A';
static const char ASCII_UPPER_LETTER_LAST = 'Z';

static const char ASCII_LOWER_LETTER_FIRST = 'a';
static const char ASCII_LOWER_LETTER_LAST = 'z';

static const char* get_program_name(const char* program_name);
static void print_help_message(const char* program_name);
static void reset_program_options();
static void register_common_options();

static int is_lower_case_ascii_letter(char character);
static int is_upper_case_ascii_letter(char character);
static int calculate_table_offset(char short_name);

static void flag_setter(char* optarg, void* param, int param_len);
static void int_param_setter(char* optarg, void* param, int param_len);
static void float_param_setter(char* optarg, void* param, int param_len);
static void string_param_setter(char* optarg, void* param, int param_len);
static int can_register_option(char short_name, const char* long_name,
                               void* param, const char* help_string);
static void register_option(char short_name, const char* long_name,
                            int has_arg);
static void add_entry_for_param_setup(char short_name, void* param, int len,
                                      const char* help_string,
                                      param_setter setter);
static void construct_options_from_registered(struct option* register_options,
                                              char* short_options,
                                              struct option* long_options);

int is_debug()
{
    return g_debug_set;
}

int register_option_flag(char short_name, const char* long_name, int* flag,
                         const char* help_string)
{
    int has_arg = 0;
    if (!can_register_option(short_name, long_name, flag, help_string))
    {
        return FAIL;
    }

    register_option(short_name, long_name, has_arg);

    add_entry_for_param_setup(short_name, flag, 0, help_string, &flag_setter);

    return SUCCESS;
}

int register_option_int(char short_name, const char* long_name, int* param,
                        const char* help_string)
{
    int has_arg = 1;
    if (!can_register_option(short_name, long_name, param, help_string))
    {
        return FAIL;
    }

    register_option(short_name, long_name, has_arg);

    add_entry_for_param_setup(short_name, param, 0, help_string,
                              &int_param_setter);

    return SUCCESS;
}

int register_option_float(char short_name, const char* long_name, float* param,
                          const char* help_string)
{
    int has_arg = 1;
    if (!can_register_option(short_name, long_name, param, help_string))
    {
        return FAIL;
    }

    register_option(short_name, long_name, has_arg);

    add_entry_for_param_setup(short_name, param, 0, help_string,
                              &float_param_setter);

    return SUCCESS;
}

int register_option_string(char short_name, const char* long_name, char* param,
                           int len, const char* help_string)
{
    int has_arg = 1;
    if (!can_register_option(short_name, long_name, param, help_string))
    {
        return FAIL;
    }

    register_option(short_name, long_name, has_arg);

    add_entry_for_param_setup(short_name, param, len, help_string,
                              &string_param_setter);

    return SUCCESS;
}

static int can_register_option(char short_name, const char* long_name,
                               void* param, const char* help_string)
{
    if (!param || !long_name || !help_string ||
        (!is_lower_case_ascii_letter(short_name) &&
         !is_upper_case_ascii_letter(short_name)))
    {
        return 0;
    }
    else
    {
        return 1;
    }
}

static void register_option(char short_name, const char* long_name, int has_arg)
{
    int offset = calculate_table_offset(short_name);
    struct option new_option = {long_name, has_arg, NULL, short_name};
    memcpy(&g_state.long_options[offset], &new_option, sizeof(struct option));
}

static void add_entry_for_param_setup(char short_name, void* param, int len,
                                      const char* help_string,
                                      param_setter setter)
{
    int offset = calculate_table_offset(short_name);
    struct OptionContext* entry = &g_state.option_contexts[offset];
    entry->setter = setter;
    entry->param = param;
    entry->param_len = len;
    entry->help_string = help_string;
}

static int is_lower_case_ascii_letter(char character)
{
    return (character >= ASCII_LOWER_LETTER_FIRST &&
            character <= ASCII_LOWER_LETTER_LAST);
}

static int is_upper_case_ascii_letter(char character)
{
    return (character >= ASCII_UPPER_LETTER_FIRST &&
            character <= ASCII_UPPER_LETTER_LAST);
}

static int calculate_table_offset(char short_name)
{
    int offset;
    if (is_upper_case_ascii_letter(short_name))
    {
        // Use index 0 to alphabet length - 1 for upper case
        offset = (int)short_name - (int)ASCII_UPPER_LETTER_FIRST;
    }
    else
    {
        // Use index starting from alphabet length for lower case
        offset = (int)short_name - (int)ASCII_LOWER_LETTER_FIRST + ALPHABET_LEN;
    }
    return offset;
}

static void flag_setter(char* optarg, void* param, int param_len)
{
    *(int*)param = 1;
}

static void int_param_setter(char* optarg, void* param, int param_len)
{
    if (optarg)
    {
        char* end;
        long tmp = strtol(optarg, &end, 0);
        if (*end == '\0')
        {
            *(int*)param = (int)tmp;
        }
    }
}

static void float_param_setter(char* optarg, void* param, int param_len)
{
    if (optarg)
    {
        char* end;
        float tmp = strtof(optarg, &end);
        if (*end == '\0')
        {
            *(float*)param = tmp;
        }
    }
}

static void string_param_setter(char* optarg, void* param, int param_len)
{
    if (optarg)
    {
        snprintf(param, param_len, "%s", optarg);
    }
}

void parse_program_options(int argc, char** argv)
{
    int opt;
    int option_idx;
    char short_options[MAX_SHORT_OPTION_LEN] = {0};
    struct option long_options[MAX_ENTRIES + TERMINATOR_LEN] = {0};

    g_debug_set = 0;

    register_common_options();

    construct_options_from_registered(g_state.long_options, short_options,
                                      long_options);

    // Need to reset optind since it gets incremented for each parse attempt
    // Required for correct behavior when calling parse_prog_options multiple
    // times
    optind = 1;
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

            case 'h':
            {
                print_help_message(get_program_name(argv[0]));
                exit(EXIT_SUCCESS);
            }

            case 'v':
            {
                printf("%s %s\n", get_program_name(argv[0]),
                       "VERSION");
                exit(EXIT_SUCCESS);
            }

            default:
            {
                if (is_lower_case_ascii_letter(opt) ||
                    is_upper_case_ascii_letter(opt))
                {
                    int offset = calculate_table_offset(opt);
                    struct OptionContext* entry =
                            &g_state.option_contexts[offset];
                    if (entry->setter)
                    {
                        entry->setter(optarg, entry->param, entry->param_len);
                    }
                }
                break;
            }
        }
    }
    // Reset all state after parsing
    reset_program_options();
}

static void register_common_options()
{
    int dummy;
    register_option_flag('d', "debug", &g_debug_set, "enable debug output");
    register_option_flag('h', "help", &dummy, "show this message and exit");
    register_option_flag('v', "version", &dummy,
                         "print version number and exit");
}

static void construct_options_from_registered(struct option* registered_options,
                                              char* short_options,
                                              struct option* long_options)
{
    int num_entries = 0;
    int short_len = 0;
    for (int i = 0; i < MAX_ENTRIES; ++i)
    {
        if (registered_options[i].val)
        {
            memcpy(&long_options[num_entries++], &registered_options[i],
                   sizeof(struct option));
            short_options[short_len++] = registered_options[i].val;
            if (registered_options[i].has_arg) short_options[short_len++] = ':';
        }
    }
}

static void reset_program_options()
{
    memset(&g_state, 0, sizeof(g_state));
}

static const char* get_program_name(const char* program_name)
{
    const char* exec_name = strrchr(program_name, '/');

    return (exec_name) ? ++exec_name : program_name;
}

static void print_help_message(const char* program_name)
{
    printf("Usage: %s [OPTION]\n\n", program_name);
    printf("OPTION:\n");
    for (int i = 0; i < MAX_ENTRIES; ++i)
    {
        if (g_state.long_options[i].val)
        {
            int offset = calculate_table_offset(g_state.long_options[i].val);
            struct OptionContext* entry = &g_state.option_contexts[offset];
            int indent = HELP_INDENT_LEVEL - LEFT_MARGIN -
                         strlen(g_state.long_options[i].name);

            if (indent < 0)
            {
                printf("  -%c,--%s\n", (char)g_state.long_options[i].val,
                       g_state.long_options[i].name);
                printf("%*s%s\n", HELP_INDENT_LEVEL, "", entry->help_string);
            }
            else
            {
                printf("  -%c,--%s%*s%s\n", (char)g_state.long_options[i].val,
                       g_state.long_options[i].name, indent, "",
                       entry->help_string);
            }
        }
    }
}
