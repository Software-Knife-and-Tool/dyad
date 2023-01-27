#
# docs makefile
#
.PHONY: reference clobber

# SRCDIRS = foreward core mu hrafn preface system
SRCDIRS = foreward core/

# page break <div style="page-break-after: always"></div>
BOOK= foreward/index.md core/2-1core.md core/2-2reader.md core/2-3compile.md core/2-4lambda.md core/2-5functions.md core/2-6macros.md core/2-7sequences.md core/2-8exceptions.md core/2-9lists.md core/2-10streams.md core/2-11utilities.md core/2-12core-symbols.md

reference:
	@for dir in $(SRCDIRS); do		\
	    for md in $$dir/*.md; do		\
	    	pandoc -s --css=./reference.css $$md -f markdown -o ./html/`basename $$md .md`.html;	\
	    done 				\
	done

clobber:
	@rm -f html/*.html
