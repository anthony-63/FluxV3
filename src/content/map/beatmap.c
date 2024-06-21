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
    const json_t* parent = json_create(meta_buffer, meta_pool, META_MAX_FIELDS);
    if(parent == NULL) {
        log_warn(g_logger, "failed to parse metadata\nmap: %s\n", path);
        return (beatmap_t) {
            .broken = 1,
        };
    }

    const json_t* version_field = json_getProperty(parent, "_version");
    if(version_field == NULL || json_getType(version_field) != JSON_INTEGER) {
        log_warn(g_logger, "failed to parse metadata(version)\nmap: %s\n", path);
        return (beatmap_t) {
            .broken = 1,
        };
    }
    uint8_t version = (uint8_t)json_getInteger(version_field);

    const json_t* title_field = json_getProperty(parent, "_title");
    if(title_field == NULL || json_getType(title_field) != JSON_TEXT) {
        log_warn(g_logger, "failed to parse metadata(title)\nmap: %s\n", path);
        return (beatmap_t) {
            .broken = 1,
        };
    }
    const char* title = json_getValue(title_field);

    size_t mapper_size = 0;
    const json_t* mapper_list = json_getProperty(parent, "_mappers");
    if(mapper_list == NULL || json_getType(mapper_list) != JSON_ARRAY) {
        log_warn(g_logger, "failed to parse metadata(mappers)\nmap: %s\n", path);
        return (beatmap_t) {
            .broken = 1,
        };
    }

    const json_t* mapper;
    for(mapper = json_getChild(mapper_list); mapper != 0; mapper = json_getSibling(mapper)) {
        const char* mapper_str = json_getValue(mapper);
        mapper_size += sizeof(mapper_str);
    }

    char** mappers = (char**)malloc(mapper_size);
    int i = 0;
    for(mapper = json_getChild(mapper_list); mapper != 0; mapper = json_getSibling(mapper)) {
        char* mapper_str = (char*)json_getValue(mapper);
        mappers[i] = mapper_str;
        i++;
    }

    difficulty_t* difficulties = (difficulty_t*)malloc(sizeof(difficulty_t));
    const json_t* diff_list = json_getProperty(parent, "_difficulties");
    if(diff_list == NULL || json_getType(diff_list) != JSON_ARRAY) {
        log_warn(g_logger, "failed to parse metadata(mappers)\nmap: %s\n", path);
        return (beatmap_t) {
            .broken = 1,
        };
    }

    i = 0;
    const json_t* difficulty;
    for(difficulty = json_getChild(diff_list); difficulty != 0; difficulty = json_getSibling(difficulty)) {
        const char* diff_str = json_getValue(difficulty);
        char diff_path[strlen(diff_str) + strlen(path) + 1];
        sprintf(diff_path, "%s/%s", path, diff_str);
        difficulty_t diff = difficulty_from_file(diff_path);
        difficulties = (difficulty_t*)realloc(difficulties, sizeof(difficulties) + sizeof(diff) + 1);
        difficulties[i] = diff;
        i++;
    }
    

    log_info(g_logger, "done loading map\n");

    return (beatmap_t){
        .artist = "",
        .broken = 0,
        .mappers = mappers,
        .cover = (uint8_t[]){},
        .path = path,
        .version = version,
        .title = title,
        .difficulties = difficulties,
    };
}