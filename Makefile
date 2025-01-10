DIR = $(shell pwd)
BUILD_DIR = $(DIR)/build

# GO_ENV_VARS = GOOS=linux GOARCH=amd64
GO_ENV_VARS =

GO_FLAGS_SHARED = -buildmode=c-shared
GO_FLAGS_STATIC = -buildmode=c-archive

.PHONY: all build-go-lib run-on-linux run-on-windows run-on-macos clean

run-on-linux: build-go-lib
	cd foo && cargo run && cd ..
	cd bar && cargo run && cd ..

run-on-windows: main.go
	$(GO_ENV_VARS) go build $(GO_FLAGS_STATIC) -o libgo-static.lib $<
	ls
	cd bar && cargo run && cd ..

run-on-windows-shared: main.go
	$(GO_ENV_VARS) go build $(GO_FLAGS_STATIC) -o libgo-shared.dll $<
	ls
	cd foo && cargo run && cd ..

run-on-macos: build-go-lib-static
	cd bar && cargo run && cd ..

run-on-macos-shared: build-go-lib-shared
	cd foo && cargo run && cd ..

build-go-lib: mk-dir build-go-lib-shared build-go-lib-static

build-go-lib-shared: main.go
	$(GO_ENV_VARS) go build $(GO_FLAGS_SHARED) -o $(BUILD_DIR)/libgo-shared.so $<

build-go-lib-static: main.go
	$(GO_ENV_VARS) go build $(GO_FLAGS_STATIC) -o $(BUILD_DIR)/libgo-static.a $<

clean:
	cd foo && cargo clean && cd ..
	cd bar && cargo clean && cd ..
	- rm -rf $(BUILD_DIR)

mk-dir:
	- mkdir -p $(BUILD_DIR)
