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

struct Input get_input(int argc, const char** argv);
