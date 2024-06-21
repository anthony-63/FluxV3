#include "fs.h"

int file_exists(char *filename) {
  struct stat buffer;
  return (stat(filename, &buffer) == 0);
}