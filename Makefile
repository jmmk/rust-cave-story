CC=rustc
RUSTLIBDIR=/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib/

all:  compile

compile:
	mkdir -p bin
	$(CC) -o bin/cave_story src/main.rs

deps:
	git submodule update
	cd lib/rust-sdl2; make clean && make
	cp lib/rust-sdl2/build/lib/libsdl2* $(RUSTLIBDIR)

run:
	@bin/cave_story
