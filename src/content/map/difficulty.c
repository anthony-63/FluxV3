#include "difficulty.h"

#include <stdio.h>

#include <log/log.h>
#include <tiny-json.h>
#include <string.h>
#include <util/fs.h>

extern logger_t g_logger;

difficulty_t difficulty_from_file(char* path) {
    if(!file_exists(path)) {
        log_warn(g_logger, "failed to load map: cannot find difficulty\nmap: %s\n", path);
        return (difficulty_t) {
            .broken = 1,
        };
    }
    
    FILE* diff_file = fopen(path, "r");
    fseek(diff_file, 0, SEEK_END);
    size_t diff_size = ftell(diff_file);
    rewind(diff_file);

    char diff_buffer[diff_size+1];
    fread(diff_buffer, 1, diff_size, diff_file);
    diff_buffer[diff_size] = '\0';

    size_t diff_max_fields = 3;

    char* notes_ptr = strstr(diff_buffer, "_notes");
    while(notes_ptr) {
        if(*(notes_ptr + 1) == 0) break;

        if(*notes_ptr == '_' && (*(notes_ptr + 1) == 'x' || *(notes_ptr + 1) == 'y' || *(notes_ptr + 1) == 't')) {
            diff_max_fields++;
        }
        notes_ptr++;
    }

    diff_max_fields *= 2;

    json_t diff_pool[diff_max_fields];
    const json_t* parent = json_create(diff_buffer, diff_pool, diff_max_fields);
    if(parent == NULL) {
        log_warn(g_logger, "failed to parse difficulty\nmap: %s\n", path);
        return (difficulty_t) {
            .broken = 1,
        };
    }

    const json_t* version_field = json_getProperty(parent, "_version");
    if(version_field == NULL || json_getType(version_field) != JSON_INTEGER) {
        log_warn(g_logger, "failed to parse difficulty(version)\nmap: %s\n", path);
        return (difficulty_t) {
            .broken = 1,
        };
    }
    uint8_t version = (uint8_t)json_getInteger(version_field);

    const json_t* name_field = json_getProperty(parent, "_name");
    if(name_field == NULL || json_getType(name_field) != JSON_TEXT) {
        log_warn(g_logger, "failed to parse difficulty(name)\nmap: %s\n", path);
        return (difficulty_t) {
            .broken = 1,
        };
    }
    const char* name = json_getValue(name_field);

    const json_t* note_list = json_getProperty(parent, "_notes");
    if(note_list == NULL || json_getType(note_list) != JSON_ARRAY) {
        log_warn(g_logger, "failed to parse metadata(mappers)\nmap: %s\n", path);
        return (difficulty_t) {
            .broken = 1,
        };
    }

    int i = 0;
    int note_count = 0;
    const json_t* note;
    for(note = json_getChild(note_list); note != 0; note = json_getSibling(note)) {
        note_count++;
    }
    
    notedata_t notes[note_count];

    for(note = json_getChild(note_list); note != 0; note = json_getSibling(note)) {
        const json_t* x_field = json_getProperty(note, "_x");
        if(x_field == NULL || json_getType(x_field) != JSON_INTEGER) {
            log_warn(g_logger, "failed to parse note %d(cannot find _x)\nmap: %s\n", i, path);
            return (difficulty_t) {
                .broken = 1,
            };
        }
        float x = (float)json_getReal(x_field);

        const json_t* y_field = json_getProperty(note, "_y");
        if(y_field == NULL || json_getType(y_field) != JSON_INTEGER) {
            log_warn(g_logger, "failed to parse note %d(cannot find _y)\nmap: %s\n", i, path);
            return (difficulty_t) {
                .broken = 1,
            };
        }
        float y = (float)json_getReal(y_field);
        
        const json_t* time_field = json_getProperty(note, "_time");
        if(time_field == NULL || (json_getType(time_field) != JSON_REAL && json_getType(time_field) != JSON_INTEGER)) {
            log_warn(g_logger, "failed to parse note %d(cannot find _time)\nmap: %s\n", i, path);
            return (difficulty_t) {
                .broken = 1,
            };
        }

        float time = 0;

        switch(json_getType(time_field)) {
            JSON_REAL: time = (float)json_getReal(time_field); break;
            JSON_INTEGER: time = (float)json_getInteger(time_field); break;
        }

        notes[i] = (notedata_t){
            .x = x,
            .y = y,
            .time = time,
        };

        i++;
    }

    return (difficulty_t) {
        .broken = 0,
        .version = version,
        .name = name,
        .notes = notes,
    };
}