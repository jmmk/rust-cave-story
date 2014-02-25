CC=rustc
CFLAGS=
LDFLAGS=-L /usr/lib -L /usr/local/lib

all:  compile

compile:
	$(CC) $(CFLAGS) -o bin/cave_story $(LDFLAGS) src/main.rs
