#
# dyad makefile
#
.PHONY: world run clean doc dist format tests runtime release
RUNTIME=target/debug/runtime

help:
	@echo "dyad top-level makefile -----------------"
	@echo
	@echo "--- build options"
	@echo "    world - build release and package dist"
	@echo "    debug - build runtime for debug"
	@echo "    release - build runtime for release"
	@echo "--- distribution options"
	@echo "    doc - generate documentation"
	@echo "    dist - build distribution image"
	@echo "    install - install distribution (needs sudo)"
	@echo "    uninstall - uninstall distribution (needs sudo)"
	@echo "--- development options"
	@echo "    clean - remove build artifacts"
	@echo "    commit - run clippy, rustfmt, make test summary"
	@echo "    tags - make etags"
	@echo "--- commit test options"
	@echo "    tests/rust - rust tests"
	@echo "    tests/summary - external test summary"
	@echo "--- performance options (not yet)"
	@echo "    perf/base - establish base"
	@echo "    perf/perf - run performance tests"
	@echo "    perf/report - test report"

world: release dist

tags:
	@etags `find src/mu -name '*.rs' -print`

release:
	@cargo build --release
	@cp target/release/runtime dist

debug:
	@cargo build
	@cp target/debug/runtime dist

dist:
	@make -C ./dist --no-print-directory

doc:
	@make -C ./doc --no-print-directory

install:
	@make -C ./dist -f install.mk install

tests/rust:
	@make -C tests cargo --no-print-directory

tests/summary:
	@make -C tests summary --no-print-directory

commit:
	@cargo fmt
	@echo ";;; rust tests"
	@cargo -q test | sed -e '/^$$/d'
	@echo ";;; clippy tests"
	@cargo clippy
	@make -C tests commit

clean:
	@rm -f TAGS
	@make -C docker clean --no-print-directory
	@make -C dist clean --no-print-directory
	@make -C tests clean --no-print-directory
