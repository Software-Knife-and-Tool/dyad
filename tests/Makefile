#
# tests makefile
#
.PHONY: all clean

all:
	@cargo test | grep "test result"

tests:
	@./run-mu-tests mu:mu-chars
	@./run-mu-tests mu:mu-compile
	@./run-mu-tests mu:mu-core
	@./run-mu-tests mu:mu-lists
	@./run-mu-tests mu:mu-namespace
	@./run-mu-tests mu:mu-numbers
	@./run-mu-tests mu:mu-reader
	@./run-mu-tests mu:mu-special-forms
	@./run-mu-tests mu:mu-streams
	@./run-mu-tests mu:mu-symbols
	@./run-mu-tests mu:mu-vectors

report:
	@make tests > tests.out
	@python3 report.py tests.out
	@rm -f tests.out

summary:
	@make tests > tests.out
	@python3 summary.py tests.out
	@rm -f tests.out

clean:
	@rm -f *.diff *.out
