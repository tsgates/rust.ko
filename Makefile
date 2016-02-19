# Version of the kernel to compile this module for
KERNEL_VERSION  := $(shell uname -r)
# Name of this kernel module
KERNEL_MODULE   := hello
# List of C files to compile into this kernel module
C_FILES         := $(wildcard *.c)
# Name of the main Rust
RUST_FILE_MAIN  := main.rs
# List of all Rust files that will be compiled into this kernel module
RUST_FILES      := $(wildcard *.rs)
# Name of target object file created by Rust compiler
RUST_TARGET_OBJ := ${KERNEL_MODULE}-rust.o
# Base directory of the Rust compiler
RUST_ROOT       := /usr


-include ./config.mk

# Rust compiler settings
RC        = $(RUST_ROOT)/bin/rustc
RCFLAGS   = -O -C code-model=kernel -C relocation-model=static
MAKEFLAGS = --no-print-directory


# Used by kbuild to determine which files to build:
obj-m := ${KERNEL_MODULE}.o
${KERNEL_MODULE}-objs := $(patsubst %.c,%.o,${C_FILES}) ${RUST_TARGET_OBJ}

all: ${KERNEL_MODULE}.ko

${RUST_TARGET_OBJ}: ${RUST_FILE_MAIN} ${RUST_FILES}
	$(RC) $(RCFLAGS) --crate-type lib -o "$@" --emit obj "${RUST_FILE_MAIN}"

${KERNEL_MODULE}.ko: ${RUST_TARGET_OBJ} ${C_FILES}
	$(MAKE) -C "/lib/modules/${KERNEL_VERSION}/build" M="$(PWD)" modules



insmod:
	sudo insmod "${KERNEL_MODULE}.ko"
	dmesg | tail

rmmod:
	sudo rmmod "${KERNEL_MODULE}"
	dmesg | tail

clean:
	$(MAKE) -C "/lib/modules/${KERNEL_VERSION}/build" M="$(PWD)" clean

test: ${KERNEL_MODULE}.ko
	sudo insmod "${KERNEL_MODULE}.ko"
	sudo rmmod  "${KERNEL_MODULE}"
	dmesg | tail -3

.PHONY: all clean insmod rmmod test
