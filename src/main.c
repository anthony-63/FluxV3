#include <stdio.h>

#include <log/log.h>
#include <window/window.h>

#include <content/map/beatmap.h>

logger_t g_logger;
beatmap_t g_selected_map;
int g_selected_diff_idx = 0;

int main() {
    g_logger = make_logger("log.txt");

    log_info(g_logger, "setting raylib log level\n");
    SetTraceLogLevel(LOG_WARNING);

    window_t window = make_window();
    run_window(&window);
    cleanup_window(&window);
}