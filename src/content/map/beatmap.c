#include "beatmap.h"

#include <string.h>
#include <stdio.h>
#include <tiny-json.h>

#include <util/fs.h>
#include <log/log.h>

#define MAX_PATH_LEN 512
#define META_MAX_FIELDS 64

extern logger_t g_logger;

beatmap_t beatmap_from_folder(char* path) {
    log_info(g_logger, "loading map: %s\n", path);

    char meta_path[MAX_PATH_LEN];
    sprintf(meta_path, "%s/meta.json", path);

    if(!file_exists(meta_path)) {
        log_warn(g_logger, "failed to load map: no meta.json\nmap: %s\n", path);
        return (beatmap_t) {
            .broken = 1,
        };
    }

    FILE* meta_file = fopen(meta_path, "r");
    fseek(meta_file, 0, SEEK_END);
    size_t meta_size = ftell(meta_file);
    rewind(meta_file);

    char meta_buffer[meta_size+1];
    fread(meta_buffer, 1, meta_size, meta_file);
    meta_buffer[meta_size] = '\0';

    json_t meta_pool[META_MAX_FIELDS];
    json_t const* parent = json_create(meta_buffer, meta_pool, META_MAX_FIELDS);
    if(parent == NULL) {
        log_warn(g_logger, "failed to parse metadata\nmap: %s\n", path);
        return (beatmap_t) {
            .broken = 1,
        };
    }

    json_t const* version_field = json_getProperty(parent, "_version");
    if(version_field == NULL || json_getType(version_field) != JSON_INTEGER) {
        log_warn(g_logger, "failed to parse metadata(version)\nmap: %s\n", path);
        return (beatmap_t) {
            .broken = 1,
        };
    }
    uint8_t version = (uint8_t)json_getInteger(version_field);

    json_t const* title_field = json_getProperty(parent, "_title");
    if(title_field == NULL || json_getType(title_field) != JSON_TEXT) {
        log_warn(g_logger, "failed to parse metadata(title)\nmap: %s\n", path);
        return (beatmap_t) {
            .broken = 1,
        };
    }
    char* title = json_getValue(title_field);

    
}