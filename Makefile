BUILDTYPE ?= debug

RUST_FLAGS = --target aarch64-unknown-none --features="bsp_rpi3"
ifeq ($(BUILDTYPE), release)
RUST_FLAGS += "--release"
endif


QEMU_FLAGS = -s -M raspi3b -cpu cortex-a53 -serial null -serial stdio -display none

CMD_PREFIX.Darwin.x86_64=aarch64-elf-
CMD_PREFIX.Linux.x86_64=rust-
CMD_PREFIX.Linux.aarch64=
CMD_PREFIX += $(CMD_PREFIX.$(shell uname -s).$(shell uname -m))

all: kernel.img

clean:
	cargo clean
	rm kernel8.dump kernel8.img

kernel.img: kernel
	$(CMD_PREFIX)objcopy target/aarch64-unknown-none/$(BUILDTYPE)/kernel -O binary kernel8.img

kernel:
	RUSTFLAGS="-C link-arg=linker.ld" cargo rustc $(RUST_FLAGS)
	$(CMD_PREFIX)objdump -D target/aarch64-unknown-none/$(BUILDTYPE)/kernel > kernel8.dump

qemu: kernel.img
	qemu-system-aarch64 $(QEMU_FLAGS) -kernel target/aarch64-unknown-none/$(BUILDTYPE)/kernel

qemus: kernel.img
	qemu-system-aarch64 $(QEMU_FLAGS) -S -kernel target/aarch64-unknown-none/$(BUILDTYPE)/kernel

gdb:
	RUST_GDB=gdb-multiarch $(CMD_PREFIX)gdb -q --eval-command="target remote localhost:1234" --symbols=target/aarch64-unknown-none/$(BUILDTYPE)/kernel
