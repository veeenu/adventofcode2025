#include "common.h"

inline i32 index_of(const char* input, char c, u32 length) {
  char buf[16] __attribute__((aligned(16)));
  strncpy(buf, input, 16);
  i32 ret = 32;
  i32 i = 0;

  while (ret == 32 && i < length) {
    __m128i iinput = _mm_loadu_si128((__m128i*)input);
    __m128i mask = _mm_cmpeq_epi8(iinput, _mm_set1_epi8(c));
    ret = __builtin_ctz(_mm_movemask_epi8(mask));
    i += 16;
  }

  return ret;
}

int cmp_char(const void* a, const void* b) {
  return (*(char*)b - *(char*)a);
}

u64 part1(struct Input* input) {
  usize offset = 0;
  u64 count = 0;

  while (offset < input->len) {
    const char* data = input->input + offset;
    usize length = index_of(data, '\n', input->len - offset);
    if (length == -1) {
      break;
    }

    u64 val = 0;
    i32 index = 0;
    for (char digit = '9'; digit >= '1'; digit--) {
      index = index_of(data, digit, length);
      if (index == 32) {
        continue;
      }

      val = digit - '0';
      break;
    }

    int index2 = 0;
    for (char digit = '9'; digit >= '1'; digit--) {
      index2 = index_of(data + index + 1, digit, length - index);
      printf("32 == %d? %d\n", index2, index2 == 32);
      if (index2 == 32) {
        continue;
      }

      val *= 10;
      val += (digit - '0');
      break;
    }

    count += val;

    offset += length + 1;
  }

  return count;
}

u64 part2(struct Input* input) {
  return 0;
}

int main(int argc, const char** argv) {
  struct Input input = get_input(argc, argv);
  printf("\033[32;1mPart 1\033[0m: %ld\n", part1(&input));
  printf("\033[32;1mPart 2\033[0m: %ld\n", part2(&input));
}
