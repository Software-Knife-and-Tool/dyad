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
	@./run-core-tests core:compile
	@./run-core-tests core:core
	@./run-core-tests core:exceptions
	@./run-core-tests core:format
	@./run-core-tests core:functions
	@./run-core-tests core:lambda
	@./run-core-tests core:lists
	@./run-core-tests core:macro
	@./run-core-tests core:read
	@./run-core-tests core:sequences
	@./run-core-tests core:streams
	@./run-core-tests core:strings

report:
	@make tests > tests.out
	@python3 report.py tests.out
	@rm -f tests.out

summary:
	@make tests > tests.out
	@python3 summary.py tests.out | sort | uniq
	@rm -f tests.out

clean:
	@rm -f *.diff *.out
