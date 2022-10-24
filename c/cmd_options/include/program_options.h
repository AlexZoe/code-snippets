#ifndef COMMON_OPTIONS_H
#define COMMON_OPTIONS_H

#ifdef __cplusplus
extern "C" {
#endif

enum ProgOptCode { SUCCESS = 0, FAIL };

int register_option_flag(char short_name, const char* long_name, int* flag,
                         const char* help_string);
int register_option_int(char short_name, const char* long_name, int* param,
                        const char* help_string);
int register_option_float(char short_name, const char* long_name, float* param,
                          const char* help_string);
int register_option_string(char short_name, const char* long_name, char* param,
                           int len, const char* help_string);
void parse_program_options(int argc, char** argv);

int is_debug();

#ifdef __cplusplus
}
#endif

#endif // COMMON_OPTIONS_H
