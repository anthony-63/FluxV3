#include "log.h"
#include <stdarg.h>
#include <time.h>
#include <stdlib.h>

logger_t make_logger(const char* output_path) {
    return (logger_t) {
        .output_file = fopen(output_path, "w+"),
    };
}

void log_info(logger_t logger, const char* fmt, ...) {
    va_list args;
    va_start(args, fmt);

    time_t raw_time;
    struct tm * timeinfo;

    time(&raw_time);
    timeinfo = localtime(&raw_time);
    
    printf("[INFO %d:%d:%d] ", timeinfo->tm_hour, timeinfo->tm_min, timeinfo->tm_sec);
    vprintf(fmt, args);

    fprintf(logger.output_file, "[INFO %d:%d:%d] ", timeinfo->tm_hour, timeinfo->tm_min, timeinfo->tm_sec);
    vfprintf(logger.output_file, fmt, args);

    va_end(args);
}

void log_warn(logger_t logger, const char* fmt, ...) {
    va_list args;
    va_start(args, fmt);

    time_t raw_time;
    struct tm * timeinfo;

    time(&raw_time);
    timeinfo = localtime(&raw_time);
    
    printf("[WARNING %d:%d:%d] ", timeinfo->tm_hour, timeinfo->tm_min, timeinfo->tm_sec);
    vprintf(fmt, args);

    fprintf(logger.output_file, "[WARNING %d:%d:%d] ", timeinfo->tm_hour, timeinfo->tm_min, timeinfo->tm_sec);
    vfprintf(logger.output_file, fmt, args);
    
    va_end(args);
}

void log_err(logger_t logger, const char* fmt, ...) {
    va_list args;
    va_start(args, fmt);

    time_t raw_time;
    struct tm * timeinfo;

    time(&raw_time);
    timeinfo = localtime(&raw_time);
    
    printf("[ERROR %d:%d:%d] ", timeinfo->tm_hour, timeinfo->tm_min, timeinfo->tm_sec);
    vprintf(fmt, args);

    fprintf(logger.output_file, "[ERROR %d:%d:%d] ", timeinfo->tm_hour, timeinfo->tm_min, timeinfo->tm_sec);
    vfprintf(logger.output_file, fmt, args);
    
    va_end(args);

    exit(-1);
}
