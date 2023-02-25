#
# tests makefile
#
.PHONY: help cargo clean commit report report-all summary mu core results

TMPF:=$(shell mktemp)
NAMESPACES=\
	mu	\
	core

help:
	@echo "dyad test makefile -----------------"
	@echo
	@echo "--- test options"
	@echo "    cargo - run rust tests"
	@echo "    namespaces - list namespaces"
	@echo "    commit - create test summary"
	@echo "    list - tests in \$$NAMESPACE"
	@echo "    \$$NAMESPACE - run all tests in namespace, raw output"
	@echo "    summary - run all tests in all namespaces and print summary"
	@echo "    report - individual test: NAMESPACE=namespace TEST=test and formatted output"
	@echo "    report-all - report all tests in all namespaces"

cargo:
	@cargo test | grep "test result"

mu:
	@for test in `cat mu/tests`; do 	\
	    bash mu/run-test mu/$$test;		\
	done

core:
	@for test in `cat core/tests`; do 	\
	    bash core/run-test core/$$test;	\
	done

namespaces:
	@echo $(NAMESPACES)

list:
	@cat $$NAMESPACE/tests

run:
	@for test in `cat $$NAMESPACE/tests`; do \
	    bash $$NAMESPACE/run-test $$NAMESPACE/$$test;	\
	done

report-all:
	@for namespace in $(NAMESPACES); do	\
	    for test in `cat $$namespace/tests`; do 	\
	        bash $$namespace/run-test $$namespace/$$test > $(TMPF);	\
	        python3 report.py $(TMPF);		\
	    done;					\
	done

report:
	@bash $(NAMESPACE)/run-test $(NAMESPACE)/$(TEST) > $(TMPF)
	@python3 report.py $(TMPF)
	@rm -f $(TMPF)

commit:
	@for namespace in $(NAMESPACES); do	\
	    make $$namespace >> $(TMPF);	\
	done
	@python3 summary.py $(TMPF) > tests.summary
	@git diff --unified=0 tests.summary

summary:
	@for namespace in $(NAMESPACES); do	\
	    make $$namespace >> $(TMPF);	\
	done
	@python3 summary.py $(TMPF) > tests.summary
	@cat tests.summary
	@rm -f $(TMPF)

clean:
	@rm -f *.diff *.out
