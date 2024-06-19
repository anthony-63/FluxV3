#include "window.h"

#include <stdlib.h>
#include <raylib.h>

#include <log/log.h>
#include <content/map/beatmap.h>

extern beatmap_t g_selected_map;
extern logger_t g_logger;

window_t make_window() {
    log_info(g_logger, "initializing raylib\n");
    InitWindow(1280, 720, "FluxV3");

    window_t window = (window_t){};

    log_info(g_logger, "allocating game\n");
    window.game = make_game();

    window.state = WINDOW_STATE_LOADING;
    return window;
}

void run_window(window_t* window) {
    while(!WindowShouldClose()) {
        switch(window->state) {
            case WINDOW_STATE_LOADING: {
                g_selected_map = beatmap_from_folder("test/when_i_use_it");
                window->state = WINDOW_STATE_GAME;
            } break;
            case WINDOW_STATE_GAME: {
                ClearBackground(BLACK);
            } break;
        }
        PollInputEvents();
    }
}

void cleanup_window(window_t* window) {
    free(window->game);
}
