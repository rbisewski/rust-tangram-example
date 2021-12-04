PROJECT_NAME = 'rust-tangram-example'

# Version
VERSION = `date +%y.%m`

# If unable to grab the version, default to N/A
ifndef VERSION
    VERSION = "n/a"
endif

FLAGS =
LLD := $(shell which lld)

ifeq ($(LLD), /usr/bin/lld)
	FLAGS = -C link-arg=-fuse-ld=lld
endif

#
# Makefile options
#


# State the "phony" targets
.PHONY: all debug build run clean

all: build

debug:
	@echo 'Building debug binary...'
	@cargo build

build:
	@echo 'Building release...'
	@cargo build --release

train:
	@tangram train --file heart_disease_training_data.csv --target diagnosis

run:
	@RUSTFLAGS="${FLAGS}" cargo run

lint:
	@cargo clippy

clean:
	@echo 'Cleaning...'
	@cargo clean
