cflags := "-O3 -ggdb -Wall -march=native"

@today:
  just r $(date +%d)

@r day:
  mkdir -p bin
  gcc day{{ day }}.c common.c {{ cflags }} -o bin/day{{ day }}
  ./bin/day{{ day }}
