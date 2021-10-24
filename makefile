#!/usr/bin/make -f -j8
OX_SETUP ?= ./ox.setup
HD ?= dmz
P ?= html md txt pdf org
MD = mkdir -p $(@)

.PHONY: c

o:o/x;shed pack $< $<.tz;
	$(foreach p,$(P), shed pack $</${p} $@/${p}.tz;)

w:;ssh $(HD) rm -rf dmz/w/*.html;\
	scp -r $</x/html/*.html $(HD):dmz/w;\
	scp -r $</*.tz $(HD):dmz/w/x

c:;rm -rf o

o/x:$(OX_SETUP) $(PUB);$(MD)/org;cp ./*.org $< o/x/org;$(PUB);\
	ssh $(HD) rm -rf dmz/w/*.html; scp -r $@/html/*.html $(HD):dmz/w/;\
	shed pack $@ o/x.tz;scp -r $@ $(HD):dmz/w
