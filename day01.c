#include "common.h"
#include <emmintrin.h>

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

int part1(struct Input* input) {
  const char* ptr = input->input;
  int32_t dial = 50;
  int32_t count = 0;

  while(ptr < input->input + input->len) {
    int32_t val = parse_line(ptr);
    ptr = next_line(ptr);
    dial += val;
    dial %= 100;
    if (dial < 0) {
      dial += 100;
    }
    if (dial == 0) {
      count++;
    }
  }

  return count;
}

int part2(struct Input* input) {
  const char* ptr = input->input;
  int32_t dial = 50;
  int32_t count = 0;

  while(ptr < input->input + input->len) {
    int32_t old_dial = dial;
    int32_t val = parse_line(ptr);
    ptr = next_line(ptr);
    dial += val;
    while (dial >= 100) {
      dial -= 100;
      count++;
    }
    while (dial < 0) {
      dial += 100;
      count++;
    }
    if (dial == 0) {
      count++;
    }
    if (old_dial == 0) {
      count--;
    }
  }

  return count;
}

int main(int argc, const char** argv) {
  struct Input input = get_input(argc, argv);
  printf("%d\n", part1(&input));
  printf("%d\n", part2(&input));
}
