#!/usr/bin/make -f
## 0x0004
## --++--
RLS=./target/release
LIB_FLAGS=-p tito --release
PACK_OUT=../

.PHONY: clean lib

_: pack

pack: 
	tar --zstd -cf $(PACK_OUT)/04.tar.zst ./ ## <- tar -I zstd -xvf 04.tar.zst --one-top-level

clean:
	@rm -rf ./nullz
