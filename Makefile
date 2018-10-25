Arch = x86_64

CompileMode = release
ifeq ($(ver), debug)
CompileFlags = 
else
CompileFlags = --release
endif

all: build build/harddrive.bin

clean:
	rm build/*

qemu: build/harddrive.bin
	qemu-system-x86_64 -m 8G -drive format=raw,file="$<"

debug: build/harddrive.bin
	qemu-system-x86_64 -m 8G -drive format=raw,file="$<" -s -S

build/kernel: build/libkernel.a
	ld --gc-sections -z max-page-size=0x1000 -T kernel/linkers/$(Arch).ld -o $@ $<
	objcopy --only-keep-debug $@ $@.sym
	objcopy --strip-debug $@

build/libkernel.a:
	cargo xbuild --manifest-path kernel/Cargo.toml --target kernel/target_conf/$(Arch)-unknown-none.json $(CompileFlags)
	mv kernel/target/$(Arch)-unknown-none/$(CompileMode)/libkernel.a $@

build/harddrive.bin: build/kernel
	nasm -ibootloader/$(Arch)/ bootloader/$(Arch)/disk.asm -D KERNEL=$< -o $@

kernel/linkers/$(Arch).ld:

build:
	mkdir -p build
