#
# dist makefile
#
.PHONY: dist clean

BASE = dyad
DOCS = ../doc
DIST = ../dist
RUNTIME = ./runtime
VERSION = 0.0.8

dist:
	@install -d $(BASE)
	@install -d $(BASE)/dyad
	@install -d $(BASE)/dyad/core
	@install -d $(BASE)/dyad/preface

	@make -f dyad.mk

	@install -d $(BASE)/bin
	@install -d $(BASE)/$(BASE)
	@install -d $(BASE)/doc
	@install -d $(BASE)/lib

	@install -m 644 $(DIST)/dyad.l $(BASE)/$(BASE)

	@install -m 755 $(RUNTIME) $(BASE)/bin
	@install -m 755 ./dyad.sh $(BASE)/bin/dyad

	@install -d $(BASE)/doc/html
	@install -m 644 $(DOCS)/refcards/*.pdf $(BASE)/doc
	@install -m 644 $(DOCS)/reference/html/*.html $(BASE)/doc/html

	@tar cfz $(BASE)-$(VERSION).tgz $(BASE)
	@rm -rf $(BASE)

clean:
	@rm -rf $(BASE)-$(VERSION).tgz $(BASE) $(RUNTIME)
