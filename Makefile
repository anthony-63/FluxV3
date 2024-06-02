SRC = src/*.c

ifeq ($(OS),Windows_NT)
    OUT = bin/flux.dll
else
    OUT = bin/flux.so
endif

CFLAGS = -fPIC -O3
INCLUDE = -Iinclude -Isrc

all:
	gcc $(SRC) -shared -o $(OUT) $(CFLAGS) $(INCLUDE)
