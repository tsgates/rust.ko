RUST_ROOT :=

-include ./config.mk

RC := $(RUST_ROOT)/bin/rustc


KER = $(shell uname -r)
OBJ = hello
RMODS = macros.rs raw.rs

obj-m = ${OBJ}.o
hello-objs := stub.o main.o

all: ${OBJ}.ko

${OBJ}.ko: stub.c main.o ${RMODS} fixup
	make -C /lib/modules/$(KER)/build M=$(PWD) modules
	./fixup $@

fixup: fixup.rs
	$(RC) fixup.rs

%.o: %.rs
	$(RC) -O --crate-type lib -o $@ --emit obj $<

insmod:
	sudo insmod ${OBJ}.ko
	dmesg | tail

rmmod:
	sudo rmmod ${OBJ}
	dmesg | tail

clean:
	make -C /lib/modules/$(KER)/build M=$(PWD) clean
	rm -f fixup~

test: ${OBJ}.ko
	sudo insmod ${OBJ}.ko
	sudo rmmod ${OBJ}
	dmesg | tail -3

.PHONY: all clean insmod rmmod test
