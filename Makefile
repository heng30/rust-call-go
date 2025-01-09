#!/bin/sh

DIR = $(shell pwd)
BUILD_DIR = $(DIR)/build

GO_ENV_VARS = GOOS=linux GOARCH=amd64
GO_FLAGS = -buildmode=c-shared

.PHONY: all build clean

all: mk-dir build-go-lib build

run: build
	cd foo && cargo run && cd ..

build: build-go-lib
	cd foo && cargo build && cd ..

build-go-lib: main.go
	$(GO_ENV_VARS) go build $(GO_FLAGS) -o $(BUILD_DIR)/libgo.so $<

clean:
	cd foo && cargo clean && cd ..
	- rm -rf $(BUILD_DIR)

mk-dir:
	- mkdir -p $(BUILD_DIR)
