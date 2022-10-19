#include gccpath.mk

BUILDTYPE ?= debug

RUST_FLAGS = --target aarch64-unknown-none
ifeq ($(BUILDTYPE), rlease)
    RUST_FLAGS +=" --release"
endif
QEMU_FLAGS = -s -M raspi3b -cpu cortex-a53 -serial null -serial stdio -S

all: kernel.img

clean:
	cargo clean
	rm kernel.elf.dump

kernel.img: kernel.elf

kernel.elf:
	cargo rustc $(RUST_FLAGS)
	$(GCCPATH)objdump -D target/aarch64-unknown-none/$(BUILDTYPE)/kernel > kernel.elf.dump

dump: kernel.elf

qemu:
	qemu-system-aarch64 $(QEMU_FLAGS) -kernel target/aarch64-unknown-none/$(BUILDTYPE)/kernel

qemus: kernel.elf
	qemu-system-aarch64 $(QEMU_FLAGS) -kernel target/aarch64-unknown-none/$(BUILDTYPE)/kernel

gdb:
	gdb -q