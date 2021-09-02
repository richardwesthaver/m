#!/usr/bin/make -f
## 0x0002
## --++--
RLS=./target/release
LIB_FLAGS=-p weather --release
BIN_FLAGS=-p bin --release --features='full'
COORDS=40.7580 -73.9855
PACK_OUT=../

.PHONY: bin clean lib

bin:
	cargo build $(BIN_FLAGS)

_: bin
	@echo "use 'make demo' to try it out"

demo: bin
	$(RLS)/get $(COORDS)

lib:
	cargo build $(LIB_FLAGS)

pack:
	tar --zstd -cf $(PACK_OUT)/02.tar.zst ./ ## <- tar -I zstd -xvf 02.tar.zst --one-top-level

clean:
	cargo clean -p weather ;
	cargo clean -p bin ;
	cargo clean
