RUSTLIBDIR=/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib/

.PHONY: all clean compile deps run

all:  compile

clean:
	rm -rf build/**

compile:
	mkdir -p build
	rustc -o build/cave_story src/main.rs

deps:
	git submodule foreach git pull origin master
	cd lib/rust-sdl2; make clean && make
	cp lib/rust-sdl2/build/lib/libsdl2* $(RUSTLIBDIR)
	git add -A
	git commit -m "Update dependency versions"

run:
	@build/cave_story
