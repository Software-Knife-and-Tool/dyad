#
# install makefile
#
.PHONY: install uninstall help

ROOT = /opt
BASE = dyad

help:
	@echo install - install $(BASE) in $(ROOT)/$(BASE) (needs sudo)
	@echo uninstall - remove $(BASE) from $(ROOT) (needs sudo)

install:
	@cat ./$(BASE)*.tgz | (cd $(ROOT); tar xfz -)

uninstall:
	@rm -rf $(ROOT)/$(BASE)
