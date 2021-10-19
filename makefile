#!/usr/bin/make -f -j8
# --- meta makefile
MD = mkdir -p $(@)
PUB := ./pub.el
OX_SETUP := ./ox.setup
JK ?= jekyll
HD ?= dmz
P ?= html md txt pdf org

.PHONY: c
o:c o/x
c:;rm -rf o

o/x:$(OX_SETUP) $(PUB);$(MD)/org;cp ./*.org $< o/x/org;$(PUB);\
	ssh $(HD) rm -rf dmz/w/*.html; scp -r $@/html/*.html $(HD):dmz/w/;\
	shed pack $@ o/x.tz;scp -r $@ $(HD):dmz/w

o/p:o/x;$(MD);$(foreach p,$(P),\
	shed pack $^/${p} $@/${p}.tz;ssh $(HD) rm -rf dmz/w/p/${p}.tz;)\
	scp -r $@ $(HD):pkg/d;scp -r $< $(HD):cdn/d
