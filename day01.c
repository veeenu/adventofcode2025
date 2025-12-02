#include "common.h"
#include <emmintrin.h>
#include <string.h>

void pr(__m128i i) {
  uint8_t j[16];
  _mm_store_si128((__m128i*)j, i);
  for (int k = 0; k < 16; k++) {
    printf("%02x", j[k]);
  }
  printf("\n");
}

const int16_t POWERS[5][8] = {
  {0, 0, 0, 0, 0, 0, 0, 0},
  {1, 0, 0, 0, 0, 0, 0, 0},
  {10, 1, 0, 0, 0, 0, 0, 0},
  {100, 10, 1, 0, 0, 0, 0, 0},
  {1000, 10, 1, 0, 0, 0, 0, 0},
};

int32_t parse_line(const char* input) {
  __m128i iinput = _mm_loadu_si128((const __m128i*)input);

  __m128i mask = _mm_cmpeq_epi8(iinput, _mm_set1_epi8('\n'));
  unsigned int length = __builtin_ctz(_mm_movemask_epi8(mask));

  assert (length <= 4);
  __m128i digits = _mm_cvtepu8_epi16(_mm_srli_si128(_mm_sub_epi8(iinput, _mm_set1_epi8('0')), 1));
  __m128i powers = _mm_loadu_si128((const __m128i*)POWERS[length - 1]);
  __m128i madd   = _mm_madd_epi16(digits, powers);
  __m128i result = _mm_hadd_epi32(madd, madd);
  int r = _mm_cvtsi128_si32(result);

  if (input[0] == 'L') {
    r *= -1;
  }

  return r;
}

const char* next_line(const char* input) {
  __m128i iinput = _mm_loadu_si128((const __m128i*)input);
  __m128i mask = _mm_cmpeq_epi8(iinput, _mm_set1_epi8('\n'));
  unsigned int length = __builtin_ctz(_mm_movemask_epi8(mask));

  return input + length + 1;
}

const char* str =
"L68\n"
"L30\n"
"R48\n"
"L5\n"
"R60\n"
"L55\n"
"L1\n"
"L99\n"
"R14\n"
"L82\n";

int main(void) {
  struct Input input = download_day(__FILE__);
  // struct Input input = {
  //   .input = str,
  //   .len = strlen(str)
  // };

  const char* ptr = input.input;
  int32_t dial = 50;
  int32_t count = 0;

  while(ptr < input.input + input.len) {
    int32_t val = parse_line(ptr);
    ptr = next_line(ptr);
    printf("%d + %d -> ", dial, val);
    dial += val;
    dial %= 100;
    while (dial < 0) {
      dial += 100;
    }
    printf("%d\n", dial);
    if (dial == 0) {
      count++;
    }
  }

  printf("%d\n", count);
}
