#pragma once

#include <sys/stat.h>

static int file_exists(char *filename) {
  struct stat buffer;
  return (stat(filename, &buffer) == 0);
}