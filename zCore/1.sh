SMP=5 cargo build --features "linux board-fu740" --no-default-features --target riscv64.json -Z build-std=core,alloc -Z build-std-features=compiler-builtins-mem --release
rust-objcopy --binary-architecture=riscv64 ../target/riscv64/release/zcore --strip-all -O binary ../target/riscv64/release/zcore.bin
gzip -9 -cvf ../target/riscv64/release/zcore.bin > ./zcore.bin.gz
mkimage -f ../prebuilt/firmware/riscv/fu740_fdt.its ./zcore-fu740.itb