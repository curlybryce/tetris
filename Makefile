MODE = debug
BUILD = build --$(MODE)

ifeq ($(MODE), debug)
	BUILD = build
endif

clean:
	cargo clean

build:
	cargo $(BUILD)
	mkdir -p target/$(MODE)/assets/
	cp -r src/assets/ target/$(MODE)/

all: build