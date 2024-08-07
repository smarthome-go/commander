#!make
SHELL:=/usr/bin/env bash

RUST_TARGET_DIR := ./target
DIST_DIR := ./dist
BINARY_NAME := commander
BINARY_NAME_W_SUFFIX := $(BINARY_NAME)-musl
DOCKER_IMAGE := mikmuellerdev/caltrack

.PHONY setup:

build: build-rust

build-rust: $(wildcard src/**/*.rs)
	mkdir -p $(DIST_DIR)
	cargo b --release --target-dir=$(RUST_TARGET_DIR)/native
	cp $(RUST_TARGET_DIR)/native/release/$(BINARY_NAME) $(DIST_DIR)/

	SQLX_OFFLINE=true \
	CROSS_ROOTLESS_CONTAINER_ENGINE=1 cross b \
		--release \
		--target-dir=$(RUST_TARGET_DIR)/musl \
		--target=x86_64-unknown-linux-musl
	cp $(RUST_TARGET_DIR)/musl/x86_64-unknown-linux-musl/release/$(BINARY_NAME) $(DIST_DIR)/$(BINARY_NAME_W_SUFFIX)

package: build
	cp ./commander.service $(DIST_DIR)
	cp ./install.sh $(DIST_DIR)
	tar cvzf $(DIST_DIR).tar.gz $(DIST_DIR)

clean:
	rm -rf target
