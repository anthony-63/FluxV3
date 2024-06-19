#include "fs.h"

bool file_exists(char *filename) {
  struct stat buffer;
  return (stat(filename, &buffer) == 0);
}