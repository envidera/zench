all: check test clippy

github: check clippy

print:
	@echo from makefile!!!


# --------------------------------------------------------------------
zench-warn:
	ZENCH=warn cargo test --release

zench-panic:
	ZENCH=panic cargo test --release

# ---------------------------------------------------------------------

# Some checks

check:
	cargo fmt --all
	cargo check --workspace

test:
	cargo test

clippy:
	cargo clippy

# ---------------------------------------------------------------------

# create the book

BOOK_PATH := zench-book

install-mdbook:
	@if ! command -v mdbook &> /dev/null; then \
		echo "Instaling mdbook..."; \
		cargo install mdbook; \
	fi

book-serve: install-mdbook
	cd $(BOOK_PATH) && mdbook serve

book: install-mdbook
	cd $(BOOK_PATH) && mdbook build


# =====================================================================
# LOCAL DEV FUNCTIONS
# =====================================================================


# msrv find minimum rust version:

install-minimum-version:
	@if ! command -v msrv &> /dev/null; then \
		echo "Instaling rust..."; \
		cargo install cargo-msrv; \
	fi

# Then use a badge for readme.md
# ![MSRV](https://img.shields.io/badge/MSRV-1.80-brightgreen.svg)
#
# and put version on Cargo.toml
#
# [package]
# name = "zench"
# version = "0.1.0"
# rust-version = "1.87"  <<-- here

minimum-version:
	cargo msrv find


# ---------------------------------------------------------------------

# Generates the full crate documentation (with all features enabled)
# and opens it in the default browser.
#
# The `--cfg docsrs` flag simulates the docs.rs environment locally. It's great for testing
# how your documentation will appear on docs.rs and ensures that items marked with
# `#[cfg(docsrs)]` are included.
#
# The `--no-deps` flag skips documentation for external dependencies, speeding up the process.
#
# The `--open` flag automatically opens the generated docs in the browser.

doc:
	RUSTDOCFLAGS="--cfg docsrs" cargo doc -p zench --no-deps --open


# ---------------------------------------------------------------------

# backup project


# Variables
PATH_BACKUP := .tmp/backup
DATE := $(shell date +%Y-%m-%d-%H:%M:%S)
BACKUP_FILE := $(PATH_BACKUP)/$(DATE).tar.gz

# List of exclusions
EXCLUDES := Cargo.lock target .tmp
EXCLUDE_FLAGS := $(foreach e,$(EXCLUDES),--exclude=$(e))




.PHONY: backup
backup:
	@echo "Creating backup directory if not exists..."
	@mkdir -p $(PATH_BACKUP)

	@echo "Creating backup: $(BACKUP_FILE)"
	@tar -czf $(BACKUP_FILE) $(EXCLUDE_FLAGS) * .*

	@echo "Backup completed! File saved at: $(BACKUP_FILE)"
