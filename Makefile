include gccpath.mk

RUST_FLAGS = --target aarch64-unknown-none --release
QEMU_FLAGS = -s -M raspi3 -cpu cortex-a53 -serial null -serial stdio

all: kernel.img

kernel.img: kernel.elf

kernel.elf:
	cargo rustc  $(RUST_FLAGS)
	$(GCCPATH)objdump -D target/aarch64-unknown-none/release/kernel > kernel.elf.dump

dump: kernel.elf

qemu: kernel.elf
	qemu-system-aarch64 $(QEMU_FLAGS) -kernel target/aarch64-unknown-none/release/kernel

gdb:
	aarch64-linux-gnu-gdb -q