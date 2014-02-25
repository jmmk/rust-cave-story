CC=rustc
CFLAGS=
LDFLAGS=-L /usr/local/lib

all:  compile

compile:
	$(CC) $(CFLAGS) -o bin/cave_story $(LDFLAGS) src/main.rs

run:
	@bin/cave_story
