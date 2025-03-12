.PHONY: all clean build

all: build

clean:
	rm -f library/ansible_module
	cargo clean

build:
	cargo build
	cp -v target/debug/ansible_module library/ansible_module
