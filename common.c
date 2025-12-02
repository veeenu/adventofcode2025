#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <sys/wait.h>
#include <unistd.h>

#include "common.h"

static int get_day(const char* filepath) {
  const char* name = strstr(filepath, "day");
  if (!name) {
    return -1;
  }

  int day = 0;
  sscanf(name, "day%02d", &day);

  return day;
}

struct Input download_day(const char* filepath) {
  int day = get_day(filepath);

  struct stat st = {0};
  if (stat("input", &st) == -1) {
    mkdir("input", 0700);
  }

  char input_path[256];
  snprintf(input_path, 256, "input/%02d.txt", day);

  if (stat(input_path, &st) == -1) {
    char url[256];
    snprintf(url, 256, "https://adventofcode.com/2025/day/%d/input", day);

    char cookie[256];
    FILE* fp = fopen(".cookie", "rb");
    fgets(cookie, 256, fp);
    fclose(fp);

    pid_t pid = fork();

    if (pid == 0) {
      execl("/usr/bin/curl", "curl", "--cookie", cookie, "-L", url, "-o", input_path, NULL);
      perror("execl failed");
      exit(1);
    }

    int status;
    waitpid(pid, &status, 0);
  }

  if (stat(input_path, &st) == -1) {
    perror("Couldn't read input file\n");
    exit(1);
  }

  char* buf = malloc(st.st_size);
  FILE* fp = fopen(input_path, "rb");
  fread(buf, st.st_size, 1, fp);
  fclose(fp);

  return (struct Input){
    .input = buf,
    .len = st.st_size
  };
}
