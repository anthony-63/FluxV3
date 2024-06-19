#include <stdio.h>

#include "log/log.h"

logger_t g_logger;

int main() {
    g_logger = make_logger("log.txt");

    log_info(g_logger, "test %d\n", 2);
    log_warn(g_logger, "test %d\n", 2);
    log_err(g_logger, "test %d\n", 2);
}