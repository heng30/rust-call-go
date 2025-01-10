DIR = $(shell pwd)
BUILD_DIR = $(DIR)/build

# GO_ENV_VARS = GOOS=linux GOARCH=amd64
GO_ENV_VARS = GOARCH=amd64

GO_FLAGS_SHARED = -buildmode=c-shared
GO_FLAGS_STATIC = -buildmode=c-archive

.PHONY: all build run clean

all: mk-dir build

run: build
	ls build
	cd foo && cargo run && cd ..
	cd bar && cargo run && cd ..

build: mk-dir build-with-go-shared build-with-go-static

build-with-go-shared: build-go-lib-shared
	cd foo && cargo build && cd ..

build-with-go-static: build-go-lib-static
	cd bar && cargo build && cd ..

build-go-lib-shared: main.go
	$(GO_ENV_VARS) go build $(GO_FLAGS_SHARED) -o $(BUILD_DIR)/libgo-shared.so $<
	- cp -f $(BUILD_DIR)/libgo-shared.so  $(BUILD_DIR)/libgo-shared.dll

build-go-lib-static: main.go
	$(GO_ENV_VARS) go build $(GO_FLAGS_STATIC) -o $(BUILD_DIR)/libgo-static.a $<
	- cp -f $(BUILD_DIR)/libgo-static.a  $(BUILD_DIR)/libgo-static.lib

clean:
	cd foo && cargo clean && cd ..
	cd bar && cargo clean && cd ..
	- rm -rf $(BUILD_DIR)

mk-dir:
	- mkdir -p $(BUILD_DIR)
