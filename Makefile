BUILDTYPE ?= debug

RUST_FLAGS = --target aarch64-unknown-none
ifeq ($(BUILDTYPE), rlease)
    RUST_FLAGS +=" --release"
endif
QEMU_FLAGS = -s -M raspi3b -cpu cortex-a53 -serial stdio -serial null -vnc :1 -S

GDB.Linux.x86_64=aarch64-linux-gnu-gdb
GDB.Linux.aarch64=gdb
GDB += $(GDB.$(shell uname -s).$(shell uname -m))

all: kernel.img

clean:
	cargo clean
	rm kernel.elf.dump

kernel.img: kernel.elf

kernel.elf:
	RUSTFLAGS="-C link-arg=linker.ld" cargo rustc $(RUST_FLAGS)
	aarch64-linux-gnu-objdump -D target/aarch64-unknown-none/$(BUILDTYPE)/kernel > kernel.elf.dump

dump: kernel.elf

qemu:
	qemu-system-aarch64 $(QEMU_FLAGS) -kernel target/aarch64-unknown-none/$(BUILDTYPE)/kernel

qemus: kernel.elf
	qemu-system-aarch64 $(QEMU_FLAGS) -kernel target/aarch64-unknown-none/$(BUILDTYPE)/kernel

gdb:
	$(GDB) -q