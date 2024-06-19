SRC = src/*.c src/*/*.c src/*/*/*.c

ifeq ($(OS),Windows_NT)
    OUT = bin/flux.exe
else
    OUT = bin/flux
endif

CFLAGS = -fPIC -O3 -Llib -lraylib -lopengl32 -lgdi32 -lwinmm
INCLUDE = -Iinclude -Isrc

build:
	gcc $(SRC) -o $(OUT) $(CFLAGS) $(INCLUDE)
run: build
	./$(OUT)
