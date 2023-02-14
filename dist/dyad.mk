#
# dyad makefile
#
.PHONY: dyad
SRC = ../src

CORE = \
	core.l	     	\
        compile.l    	\
        exceptions.l 	\
	fixnums.l  	\
	closures.l  	\
        format.l     	\
        lambda.l     	\
        lists.l      	\
        load.l       	\
        macro.l      	\
	read.l       	\
	symbol-macro.l	\
	read-macro.l	\
        quasiquote.l 	\
	sequences.l  	\
        streams.l    	\
        strings.l	\
	symbols.l	\
	vectors.l

PREFACE = \
	preface.l   	\
	common.l   	\
	compile.l	\
	describe.l 	\
	elf64.l		\
	lists.l	    	\
	metrics.l	\
	repl.l	    	\
	require.l   	\
	state.l	    	\
        environment.l	\
        print.l

dyad:
	@cp -r $(SRC)/core dyad/dyad
	@cp -r $(SRC)/preface dyad/dyad
