#include "gtest/gtest.h"
#include <getopt.h>

#include "program_options.h"

char* g_executable_name;

TEST(ProgOption, accept_register_new_flag_option)
{
    int enable;
    EXPECT_EQ(SUCCESS, register_option_flag('e', "enable", &enable,
                                               "enable this option"));
}

TEST(ProgOption, reject_null_variable_pointer)
{
    EXPECT_EQ(FAIL,
              register_option_flag('e', "enable", NULL, "enable this option"));
}

TEST(ProgOption, reject_register_flag_with_null_long_name)
{
    int enable;
    EXPECT_EQ(FAIL,
              register_option_flag('e', NULL, &enable, "enable this option"));
}

TEST(ProgOption, reject_register_flag_with_null_help_string)
{
    int enable;
    EXPECT_EQ(FAIL, register_option_flag('e', "", &enable, NULL));
}

TEST(ProgOption, accept_capital_short_name)
{
    int enable;
    EXPECT_EQ(SUCCESS,
              register_option_flag('E', "", &enable, "enable this option"));
}

TEST(ProgOption, reject_non_ascii_letter_character)
{
    int enable;
    EXPECT_EQ(FAIL,
              register_option_flag('?', "", &enable, "enable this option"));
}

TEST(ProgOption, flag_is_not_set_for_missing_commandline_argument)
{
    int enable = 0;
    char* argv[] = {g_executable_name};
    int argc = 1;

    EXPECT_EQ(SUCCESS,
              register_option_flag('e', "", &enable, "enable this option"));
    parse_program_options(argc, argv);
    EXPECT_EQ(0, enable);
}

TEST(ProgOption, flag_is_set_for_short_commandline_argument)
{
    int enable = 0;
    char* argv[] = {g_executable_name, const_cast<char*>("-e")};
    int argc = 2;

    EXPECT_EQ(SUCCESS,
              register_option_flag('e', "", &enable, "enable this option"));
    parse_program_options(argc, argv);
    EXPECT_EQ(1, enable);
}

TEST(ProgOption, flag_is_not_set_for_unrelated_short_argument)
{
    int enable = 0;
    char* argv[] = {g_executable_name, const_cast<char*>("-f")};
    int argc = 2;

    EXPECT_EQ(SUCCESS,
              register_option_flag('e', "", &enable, "enable this option"));
    parse_program_options(argc, argv);
    EXPECT_EQ(0, enable);
}

TEST(ProgOption, flag_is_set_for_long_commandline_argument)
{
    int enable = 0;
    char* argv[] = {g_executable_name, const_cast<char*>("--enable")};
    int argc = 2;

    EXPECT_EQ(SUCCESS, register_option_flag('e', "enable", &enable,
                                               "enable this option"));
    parse_program_options(argc, argv);
    EXPECT_EQ(1, enable);
}

TEST(ProgOption, debug_flag_set_without_registering)
{
    char* argv[] = {g_executable_name, const_cast<char*>("--debug")};
    int argc = 2;

    parse_program_options(argc, argv);
    EXPECT_TRUE(is_debug());
}

TEST(ProgOption, multiple_registering_overridden_by_last)
{
    char* argv[] = {g_executable_name, const_cast<char*>("--flag")};
    int argc = 2;
    int first = 0;
    int second = 0;

    EXPECT_EQ(SUCCESS,
              register_option_flag('f', "flag", &first, "enable this option"));

    EXPECT_EQ(SUCCESS,
              register_option_flag('f', "flag", &second, "enable this option"));

    parse_program_options(argc, argv);
    EXPECT_EQ(0, first);
    EXPECT_EQ(1, second);
}

TEST(ProgOption, default_flag_overrides_manually_registered_flag)
{
    char* argv[] = {g_executable_name, const_cast<char*>("--debug")};
    int argc = 2;
    int debug_is_set = 0;

    EXPECT_EQ(SUCCESS, register_option_flag('d', "debug", &debug_is_set,
                                               "enable debug setting"));

    parse_program_options(argc, argv);
    EXPECT_EQ(0, debug_is_set);
    EXPECT_EQ(1, is_debug());
}

TEST(ProgOption, default_flag_overrides_manually_registered_option)
{
    char* argv[] = {g_executable_name, const_cast<char*>("--debug"),
                    const_cast<char*>("1")};
    int argc = 2;
    int debug_is_set = 0;

    EXPECT_EQ(SUCCESS,
              register_option_int('d', "debug", &debug_is_set,
                                  "enable or disable debug setting"));

    parse_program_options(argc, argv);
    EXPECT_EQ(0, debug_is_set);
    EXPECT_EQ(1, is_debug());
}

TEST(ProgOption, return_value_for_valid_int_parameter)
{
    int tries = 0;
    char* argv[] = {g_executable_name, const_cast<char*>("--tries"),
                    const_cast<char*>("5")};
    int argc = 3;

    EXPECT_EQ(SUCCESS, register_option_int('t', "tries", &tries,
                                              "specify number of tries"));
    parse_program_options(argc, argv);
    EXPECT_EQ(5, tries);
}

TEST(ProgOption, do_not_update_param_for_invalid_int_parameter)
{
    int tries = 0;
    char* argv[] = {g_executable_name, const_cast<char*>("--tries"),
                    const_cast<char*>("32sdf")};
    int argc = 3;

    EXPECT_EQ(SUCCESS, register_option_int('t', "tries", &tries,
                                              "specify number of tries"));
    parse_program_options(argc, argv);
    EXPECT_EQ(0, tries);
}

TEST(ProgOption, accept_hex_int_parameters)
{
    int tries = 0;
    char* argv[] = {g_executable_name, const_cast<char*>("-t"),
                    const_cast<char*>("0xf")};
    int argc = 3;

    EXPECT_EQ(SUCCESS, register_option_int('t', "tries", &tries,
                                              "specify number of tries"));
    parse_program_options(argc, argv);
    EXPECT_EQ(15, tries);
}

TEST(ProgOption, return_value_for_valid_float_parameter)
{
    float precision = 0.f;
    char* argv[] = {g_executable_name, const_cast<char*>("--precision"),
                    const_cast<char*>("0.3")};
    int argc = 3;

    EXPECT_EQ(SUCCESS, register_option_float('p', "precision", &precision,
                                                "specify argument precision"));
    parse_program_options(argc, argv);
    EXPECT_FLOAT_EQ(0.3, precision);
}

TEST(ProgOption, do_not_update_param_for_invalid_float_parameter)
{
    float precision = 0.f;
    char* argv[] = {g_executable_name, const_cast<char*>("-p"),
                    const_cast<char*>("0.3asf")};
    int argc = 3;

    EXPECT_EQ(SUCCESS, register_option_float('p', "precision", &precision,
                                                "specify argument precision"));
    parse_program_options(argc, argv);
    EXPECT_FLOAT_EQ(0.f, precision);
}

TEST(ProgOption, return_value_for_valid_string_parameter)
{
    char file_path[16] = {0};
    char* argv[] = {g_executable_name, const_cast<char*>("-p"),
                    const_cast<char*>("/tmp/file.txt")};
    int argc = 3;

    EXPECT_EQ(SUCCESS,
              register_option_string('p', "path", file_path, sizeof(file_path),
                                     "specify input file path"));
    parse_program_options(argc, argv);
    EXPECT_STREQ("/tmp/file.txt", file_path);
}

TEST(ProgOption, cut_off_when_argument_is_too_long)
{
    char very_long_path[] = "/opt/company/configuration_files/parameters.cfg";
    char file_path[16] = {'\0'};
    char* argv[] = {g_executable_name, const_cast<char*>("-p"), very_long_path};
    int argc = 3;

    EXPECT_EQ(SUCCESS,
              register_option_string(
                      'p', "path", file_path,
                      // Only use half of the available characters
                      sizeof(file_path) / 2, "specify input file path"));
    parse_program_options(argc, argv);

    // Artificially cut off input string
    very_long_path[sizeof(file_path) / 2] = '\0';
    EXPECT_STREQ(very_long_path, file_path);
}

int main(int argc, char** argv)
{
    g_executable_name = argv[0];

    testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
