CC=rustc
CFLAGS=
LDFLAGS=

all:  compile

compile:
	mkdir -p bin
	$(CC) $(CFLAGS) -o bin/cave_story $(LDFLAGS) src/main.rs

run:
	@bin/cave_story
