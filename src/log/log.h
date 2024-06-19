#pragma once

#include <stdio.h>

typedef struct {
    FILE* output_file;
} logger_t;

logger_t make_logger(const char* output_path);
void log_info(logger_t logger, const char* fmt, ...);
void log_warn(logger_t logger, const char* fmt, ...);
void log_err(logger_t logger, const char* fmt, ...);