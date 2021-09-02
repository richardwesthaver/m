#!/usr/bin/make -f
## 0x0001
## --++--
PACK_OUT=../

.PHONY: clean
default: helloworld.cc
	clang++ -Wall helloworld.cc -ohelloworld -lrocksdb
clean:
	rm -rf helloworld infodb

pack: clean
	tar --zstd -cf $(PACK_OUT)/01.tar.zst ./ ## <- tar -I zstd -xvf 01.tar.zst --one-top-level
