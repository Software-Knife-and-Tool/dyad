#!/usr/bin/env bash
BASE_CORE=/opt/dyad/dyad/core

/opt/dyad/bin/mu-runtime \
	$BASE_CORE/core.l	     	\
        $BASE_CORE/compile.l    	\
        $BASE_CORE/exceptions.l 	\
	$BASE_CORE/fixnums.l            \
	$BASE_CORE/functions.l  	\
        $BASE_CORE/format.l     	\
        $BASE_CORE/lambda.l     	\
        $BASE_CORE/lists.l      	\
        $BASE_CORE/load.l       	\
        $BASE_CORE/macro.l      	\
	$BASE_CORE/read.l       	\
	$BASE_CORE/symbol-macro.l	\
	$BASE_CORE/read-macro.l 	\
        $BASE_CORE/quasiquote.l 	\
	$BASE_CORE/sequences.l  	\
        $BASE_CORE/streams.l    	\
        $BASE_CORE/strings.l    	\
	$BASE_CORE/symbols.l    	\
	$BASE_CORE/vectors.l
