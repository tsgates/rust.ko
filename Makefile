KER = $(shell uname -r)
OBJ = hello

obj-m = ${OBJ}.o
hello-objs := stub.o main.o

all: hello.ko

hello.ko: stub.c main.o
	make -C /lib/modules/$(KER)/build M=$(PWD) modules
	rust run fixup.rs $@

%.o: %.rs
	rustc -O --lib -o $@ -c $<

insmod:
	sudo insmod ${OBJ}.ko
	dmesg | tail

rmmod:
	sudo rmmod ${OBJ}
	dmesg | tail

clean:
	make -C /lib/modules/$(KER)/build M=$(PWD) clean
	rm -f fixup~

test:
	sudo insmod ${OBJ}.ko
	sudo rmmod ${OBJ}
	dmesg | tail

.PHONY: all clean insmod rmmod test
