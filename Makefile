SHELL	:= /bin/bash
KERNEL	:= hanami
ISODIR	:= iso
SRCDIR	:= src
CFGDIR	:= config
ASMDIR	:= $(SRCDIR)/asm
BOOTDIR	:= $(ISODIR)/boot
GRUBDIR	:= $(BOOTDIR)/grub
TRIPLE	:= x86_64-unknown-$(KERNEL)-gnu

.PHONY: clean
default: build

cargo:
	xargo build --release --target $(TRIPLE)

target/boot.o: $(ASMDIR)/boot.asm
	mkdir -p target
	nasm -felf64 $(ASMDIR)/boot.asm -o target/boot.o

target/$(KERNEL).bin: cargo target/boot.o
	ld -n -o target/$(KERNEL).bin -T $(CFGDIR)/linker.ld target/boot.o target/$(TRIPLE)/release/lib$(KERNEL).a

target/$(KERNEL).iso: target/$(KERNEL).bin
	mkdir -p target/$(ISODIR)
	mkdir -p target/$(GRUBDIR)
	cp $(CFGDIR)/grub.cfg target/$(GRUBDIR)/
	cp target/$(KERNEL).bin target/$(BOOTDIR)/
	grub-mkrescue -o target/$(KERNEL).iso target/$(ISODIR)

build: cargo target/$(KERNEL).iso

run: build
	qemu-system-x86_64 -curses -cdrom target/$(KERNEL).iso

clean:
	cargo clean
