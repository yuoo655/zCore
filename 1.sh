dd if=/dev/zero of=nvme.img bs=1M count=16
cd zCore && SMP=1 cargo build --features "linux" --no-default-features --target riscv64.json -Z build-std=core,alloc -Z build-std-features=compiler-builtins-mem --release
rust-objcopy --binary-architecture=riscv64 ../target/riscv64/release/zcore --strip-all -O binary ../target/riscv64/release/zcore.bin
cd ..
qemu-system-riscv64 -smp 1 -machine virt -bios default -m 2G -no-reboot -serial mon:stdio -serial file:/tmp/serial.out -kernel target/riscv64/release/zcore.bin  -append "LOG=warn" -drive file=nvme.img,if=none,id=nvm -device nvme,serial=deadbeef,drive=nvm

cat | head -c 1024 ./nvme.img | xxd
