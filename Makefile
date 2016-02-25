# Path to the kbuild Makefile of the kernel to compile against
export KERNEL_BUILD_PATH := /lib/modules/$(shell uname -r)/build
# Name of this kernel module
export KERNEL_MODULE     := hello
# Path to the directory where kernel build artifacts should be stored
export BUILD_DIRECTORY   := build
# List of C files to compile into this kernel module
export C_FILES           := $(wildcard src/*.c)
# List of all Rust files that will be compiled into this kernel module
export RUST_FILES        := $(wildcard src/*.rs)
# Base directory of the Rust compiler
export RUST_ROOT         := /usr

# Rust compiler settings
export CARGO      = $(RUST_ROOT)/bin/cargo
export CARGOFLAGS =
export RCFLAGS    =
export RELEASE    =

-include ./config.mk

ifeq (${RELEASE},1)
	CARGOFLAGS += --release
	STRIP := strip --strip-debug
else
	STRIP := @true
endif
# Top-level project directory
export BASE_DIR := $(patsubst %/,%,$(dir $(abspath $(lastword $(MAKEFILE_LIST)))))


all modules: ${BUILD_DIRECTORY}/Makefile
	@$(MAKE) -C "${KERNEL_BUILD_PATH}" M="${BASE_DIR}/${BUILD_DIRECTORY}" modules
	cp "${BUILD_DIRECTORY}/${KERNEL_MODULE}.ko" "${KERNEL_MODULE}.ko"
	$(STRIP) "${KERNEL_MODULE}.ko"

# Make sure there always is a target `Makefile` for kbuild in place
${BUILD_DIRECTORY}/Makefile: kbuild.mk
	@mkdir -p "${BUILD_DIRECTORY}/src"
	cp "kbuild.mk" "${BUILD_DIRECTORY}/Makefile"

insmod:
	sudo insmod "${KERNEL_MODULE}.ko"
	dmesg | tail

rmmod:
	sudo rmmod "${KERNEL_MODULE}"
	dmesg | tail

clean:
	rm -rf "${BUILD_DIRECTORY}"
	$(CARGO) clean

test: ${KERNEL_MODULE}.ko
	sudo insmod "${KERNEL_MODULE}.ko"
	sudo rmmod  "${KERNEL_MODULE}"
	dmesg | tail -3

.PHONY: all modules clean insmod rmmod test
