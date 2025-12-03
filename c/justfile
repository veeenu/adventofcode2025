cflags := "-O3 -ggdb -Wall -march=native"
day := shell("date +%d")

@today:
  just r {{day}}

@r day input="":
  mkdir -p bin
  gcc day{{ day }}.c common.c {{ cflags }} -o bin/day{{ day }}
  ./bin/day{{ day }} {{ input }}
