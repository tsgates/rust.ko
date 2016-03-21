#############################
# kbuild support build file #
#############################
# This file tells kbuild how to build this project. In order to work it  needs  to  be  included  by
# the kernel build system and executed in the environment exported by the main  `Makefile`  of  this
# project.

# Name of object file to create by Rust
rust-target := lib${KERNEL_MODULE}.a

# Tell kbuild which files to build
obj-m                 := ${KERNEL_MODULE}.o
${KERNEL_MODULE}-objs := $(patsubst %.c,%.o,${C_FILES}) ${rust-target}

# Tell kbuild where the source files are
src := ${BASE_DIR}

# Fix file paths (since this script will be run from the kbuild's working directory)
C_FILES    := $(foreach filepath,${C_FILES}   ,${BASE_DIR}/$(filepath))
RUST_FILES := $(foreach filepath,${RUST_FILES},${BASE_DIR}/$(filepath))

# Build rule for Rust target object
# Note: UTS_MACHINE is the architecture of the target kernel, Rust compilation will  fail  unless  a
#       target file (such as "x86_64-unknown-none-gnu.json") was created for the architecture of the
#       kernel you're trying to compile this module for.
$(obj)/${rust-target}: ${RUST_FILES}
	cd "${BASE_DIR}" && $(CARGO) build --verbose ${CARGOFLAGS} --target="${UTS_MACHINE}-unknown-none-gnu" -- ${RCFLAGS}
	cp "${BASE_DIR}"/target/x86_64-unknown-none-gnu/debug/${rust-target} $(obj)
