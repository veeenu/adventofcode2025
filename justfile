cflags := "-mfpmath=sse -mmmx -msse -msse2 -O3 -Wall"

@today:
  just r $(date +%d)

@r day:
  mkdir -p bin
  gcc day{{ day }}.c common.c {{ cflags }} -o bin/day{{ day }}
  ./bin/day{{ day }}
