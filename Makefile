CC=rustc
CFLAGS=
LDFLAGS=

all:  compile

compile:
	$(CC) $(CFLAGS) -o bin/cave_story $(LDFLAGS) src/main.rs

run:
	@bin/cave_story
