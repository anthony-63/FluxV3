#include "beatmap.h"

#include <util/fs.h>
#include <string.h>
#include <stdio.h>

#define MAX_PATH_LEN 512

beatmap_t beatmap_from_folder(const char* path) {
    const char meta_path[MAX_PATH_LEN];
    sprintf(meta_path, "%s/meta.json", path);

    if(!file_exists(meta_path)) {
        
    }
}