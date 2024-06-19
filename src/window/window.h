#pragma

#include <game/game.h>

typedef enum {
    WINDOW_STATE_LOADING,
    WINDOW_STATE_GAME,
} window_state_t;

typedef struct {
    game_t* game;
    window_state_t state;
} window_t;