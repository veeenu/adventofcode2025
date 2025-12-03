#include "common.h"
#include <emmintrin.h>
#include <smmintrin.h>

struct Range {
  u64 min;
  u64 max;
};

inline __m128i shift(__m128i in, u8 count) {
  static const char shuffle[32] = {
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15
  };

  return _mm_shuffle_epi8(in, _mm_loadu_si128((__m128i*)&shuffle[count]));
}

inline u64 parse_int64(__m128i digits, u32 length) {
  assert (length <= 8);

  const __m128i tens = _mm_setr_epi8(10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1);
  const __m128i hundreds = _mm_setr_epi16(100, 1, 100, 1, 100, 1, 100, 1);
  const __m128i tens_thousands = _mm_setr_epi16(10000, 1, 10000, 1, 10000, 1, 10000, 1);

  __m128i sums = _mm_maddubs_epi16(digits, tens);
  sums = _mm_madd_epi16(sums, hundreds);
  sums = _mm_packs_epi32(sums, sums);
  sums = _mm_madd_epi16(sums, tens_thousands);
  u64 qw = _mm_cvtsi128_si64(sums);
  u32 high = (qw & 0xFFFFFFFF);
  u32 low = ((qw >> 32) & 0xFFFFFFFF);
  u64 ret = 100000000 * high + low;

  return ret;
}

// Unfortunately the above fails for digits > 2**32
// But we can split it
inline u64 parse_int(const char* input, u32 length) {
  assert (length <= 16);

  __m128i iinput = _mm_loadu_epi8(input);
  __m128i digits = shift(_mm_sub_epi8(iinput, _mm_set1_epi8('0')), length);

  if (length <= 8) {
    return parse_int64(digits, length);
  } else {
    u32 rem = length - 8;
    u64 high = parse_int64(shift(digits, 8), 8) * 100000000;
    u64 low = parse_int64(_mm_and_si128(digits, _mm_set_epi32(-1, -1, 0, 0)), rem);
    return high + low;
  }
}

inline u32 index_of(__m128i input, char c) {
  __m128i mask = _mm_cmpeq_epi8(input, _mm_set1_epi8(c));
  return __builtin_ctz(_mm_movemask_epi8(mask));
}

const char* parse_next(const char* input, struct Range* output) {
  int8_t is_end = 0;
  char buf[16] __attribute__((aligned(16)));
  memcpy(buf, input, 16);

  u32 dash_pos = index_of(_mm_loadu_epi8(buf), '-');
  u64 first_num = parse_int(input, dash_pos);

  memcpy(buf, input + dash_pos + 1, 16);
  u32 comma_pos = index_of(_mm_loadu_epi8(buf), ',');
  if (comma_pos >= 16) {
    comma_pos = index_of(_mm_loadu_epi8(buf), '\n');
    is_end = 1;
  }
  if (comma_pos >= 16) {
    comma_pos = index_of(_mm_loadu_epi8(buf), '\0');
    is_end = 1;
  }
  if (comma_pos >= 16) {
    return NULL;
  }
  u64 second_num = parse_int(buf, comma_pos);

  output->min = first_num;
  output->max = second_num;

  if (!is_end) {
    return input + comma_pos + dash_pos + 2;
  } else {
    return NULL;
  }
}

int check_invalid(u64 input) {
  char digits[16] __attribute__((aligned(16)));
  memset(digits, 0, 16);

  u32 i;
  for (i = 0; i < 16; i++) {
    digits[15 - i] = input % 10;
    input /= 10;
    if (input == 0) {
      break;
    }
  }
  i++;
  if (i % 2 != 0) {
    return 0;
  }

  char upper_half[16] __attribute__((aligned(16)));
  memset(upper_half, 0, 16);

  for (int j = 0; j < i / 2; j++) {
    upper_half[16 - i / 2 + j] = digits[16 - i + j];
    digits[16 - i + j] = 0;
  }

  __m128i digitsm = _mm_load_si128((__m128i*)digits);
  __m128i upper_halfm = _mm_load_si128((__m128i*)upper_half);
  __mmask8 eq = _mm_cmpeq_epi16_mask(digitsm, upper_halfm);

  return eq == 0xff;
}

u64 part1(struct Input* input) {
  struct Range range = {0};
  const char* s = input->input;

  u64 acc = 0;

  while (s) {
    s = parse_next(s, &range);
    for (u64 i = range.min; i <= range.max; i++) {
      if (check_invalid(i)) {
        printf("Invalid %ld\n", i);
        acc += i;
      }
    }
  }

  return acc;
}

u64 part2(struct Input* input) {
  return 0;
}

int main(int argc, const char** argv) {
  struct Input input = get_input(argc, argv);
  printf("\033[32;1mPart 1\033[0m: %ld\n", part1(&input));
  printf("\033[32;1mPart 2\033[0m: %ld\n", part2(&input));
}
