#!/usr/bin/make -f
## 0x0003
## --++--
RLS=./target/release
LIB_FLAGS=-p tito --release
PACK_OUT=../

.PHONY: clean lib

_: lib

lib:
	cargo build $(LIB_FLAGS)

test: lib
	cargo test $(LIB_FLAGS)

pack: 
	tar --zstd -cf $(PACK_OUT)/03.tar.zst ./ ## <- tar -I zstd -xvf 03.tar.zst --one-top-level

clean:
	cargo clean -p weather ;
	cargo clean -p bin ;
	cargo clean ;
	@rm -rf ./*/Cargo.lock
