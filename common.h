#include <stdio.h>
#include <stdint.h>
#include <smmintrin.h>
#include <emmintrin.h>
#include <x86intrin.h>
#include <assert.h>

struct Input {
  const char* input;
  const uint64_t len;
};

struct Input download_day(const char* filepath);
