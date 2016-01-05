ifdef RUST_ROOT
	RUST_ROOT := $(RUST_ROOT)
else
	RUST_ROOT := /usr
endif

-include ./config.mk

RC := $(RUST_ROOT)/bin/rustc
RCFLAGS := -O -C code-model=kernel -C relocation-model=static


KER = $(shell uname -r)
OBJ = hello
RMODS = macros.rs raw.rs

obj-m = ${OBJ}.o
hello-objs := stub.o main.o

all: ${OBJ}.ko

${OBJ}.ko: stub.c main.o ${RMODS}
	make -C /lib/modules/$(KER)/build M=$(PWD) modules

%.o: %.rs
	$(RC) $(RCFLAGS) --crate-type lib -o $@ --emit obj $<

insmod:
	sudo insmod ${OBJ}.ko
	dmesg | tail

rmmod:
	sudo rmmod ${OBJ}
	dmesg | tail

clean:
	make -C /lib/modules/$(KER)/build M=$(PWD) clean

test: ${OBJ}.ko
	sudo insmod ${OBJ}.ko
	sudo rmmod ${OBJ}
	dmesg | tail -3

.PHONY: all clean insmod rmmod test
