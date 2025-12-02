#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>

#include "common.h"

struct InputFile {
  char path[256];
  ssize_t len;
};

void download_day(int day, struct InputFile* download) {
  struct stat st = {0};
  if (stat("input", &st) == -1) {
    mkdir("input", 0700);
  }

  snprintf(download->path, 256, "input/%02d.txt", day);

  if (stat(download->path, &st) == -1) {
    char url[256];
    snprintf(url, 256, "https://adventofcode.com/2025/day/%d/input", day);

    char cookie[256];
    FILE *fp = fopen(".cookie", "rb");
    fgets(cookie, 256, fp);
    fclose(fp);

    pid_t pid = fork();

    if (pid == 0) {
      execl("/usr/bin/curl", "curl", "--cookie", cookie, "-L", url, "-o",
            download->path, NULL);
      perror("execl failed");
      exit(1);
    }

    int status;
    waitpid(pid, &status, 0);
  }

  if (stat(download->path, &st) == -1) {
    perror("Couldn't read input file\n");
    exit(1);
  }

  download->len = st.st_size;
}

struct Input get_input(int argc, const char **argv) {
  const char *bin_name = strstr(argv[0], "day");
  if (!bin_name) {
    fprintf(stderr, "Couldn't parse filename\n");
    exit(1);
  }

  struct InputFile input_file;
  if (argc <= 1) {
    int day = 0;
    sscanf(bin_name, "day%02d", &day);

    download_day(day, &input_file);
  } else {
    struct stat st = {0};
    if (stat(argv[1], &st) == -1) {
      perror("Couldn't stat input file");
      exit(1);
    }
    strncpy(input_file.path, argv[1], 255);
    input_file.len = st.st_size;
  }

  char *buf = malloc(input_file.len);
  FILE *fp = fopen(input_file.path, "rb");
  fread(buf, input_file.len, 1, fp);
  fclose(fp);

  return (struct Input){.input = buf, .len = input_file.len};
}
