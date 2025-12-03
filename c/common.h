#include <stdio.h>
#include <stdint.h>
#include <string.h>
#include <emmintrin.h>
#include <immintrin.h>
#include <smmintrin.h>
#include <tmmintrin.h>
#include <x86intrin.h>
#include <assert.h>

#define u8 uint8_t
#define u16 uint16_t
#define u32 uint32_t
#define u64 uint64_t
#define i8 int8_t
#define i16 int16_t
#define i32 int32_t
#define i64 int64_t
#define f32 float
#define f64 double

struct Input {
  const char* input;
  const uint64_t len;
};

struct Input get_input(int argc, const char** argv);
void print_m128(__m128i in);
void print_m256(__m256i in);
