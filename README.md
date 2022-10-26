# zCore

[![CI](https://github.com/rcore-os/zCore/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/rcore-os/zCore/actions)
[![Docs](https://img.shields.io/badge/docs-pages-green)](https://rcore-os.github.io/zCore/)
[![Coverage Status](https://coveralls.io/repos/github/rcore-os/zCore/badge.svg?branch=master)](https://coveralls.io/github/rcore-os/zCore?branch=master)
[![issue](https://img.shields.io/github/issues/rcore-os/zCore)](https://github.com/rcore-os/zCore/issues)
[![forks](https://img.shields.io/github/forks/rcore-os/zCore)](https://github.com/rcore-os/zCore/fork)
![stars](https://img.shields.io/github/stars/rcore-os/zCore)
![license](https://img.shields.io/github/license/rcore-os/zCore)

基于 zircon 并提供 Linux 兼容性的操作系统内核。

## 原版README

  Reimplement `Zircon` microkernel in safe Rust as a userspace program!

- zCore设计架构概述
- 支持bare-metal模式的Zircon & Linux
- 支持libos模式的Zircon & Linux
- 支持的图形应用程序等更多指导请查看[原版README文档](README-arch.md)。

## 启动内核

   ```bash
   cargo qemu --arch riscv64
   ```

   这个命令会使用 qemu-system-riscv64 启动 zCore。

   默认的文件系统中将包含 busybox 应用程序和 musl-libc 链接器。它们是用自动下载的 musl-libc RISC-V 交叉编译工具链编译的。

## 目录

- [启动内核](#启动内核)
- [项目构建](#项目构建)
  - [构建命令](#构建命令)
  - [命令参考](#命令参考)
- [平台支持](#平台支持)
  - [Qemu/virt](#qemuvirt)
  - [全志/哪吒](#全志哪吒)
  - [赛昉/星光](#赛昉星光)
  - [晶视/cr1825](#晶视cr1825)

## 项目构建

项目构建采用 [xtask 模式](https://github.com/matklad/cargo-xtask)，常用操作被封装成 cargo 命令。

另外，还通过 [Makefile](Makefile) 提供 make 调用，以兼容一些旧脚本。

目前已测试的开发环境包括 Ubuntu20.04、Ubuntu22.04 和 Debian11，Ubuntu22.04 不能正确编译 x86_64 的 libc 测试。若不需要烧写到物理硬件，使用 WSL2 或其他虚拟机的操作与真机并无不同之处。

### 构建命令

命令的基本格式为 `cargo <command> [--args [value]]`，这实际上是 `cargo run --package xtask --release -- <command> [--args [value]]` 的简写。`command` 被传递给 xtask 应用程序，解析并执行。

许多命令的效果受到仓库环境的影响，也会影响仓库的环境。为了使用方便，如果一个命令依赖于另一个命令的效果，它们被设计为递归的。命令的递归关系图如下，对于它们的详细解释在下一节：

---

> **NOTICE** 建议使用等宽字体

---

```text
┌────────────┐ ┌─────────────┐ ┌─────────────┐
| update-all | | check-style | | zircon-init |
└────────────┘ └─────────────┘ └─────────────┘
┌─────┐ ┌──────┐  ┌─────┐  ┌─────────────┐ ┌─────────────────┐
| asm | | qemu |─→| bin |  | linux-libos | | libos-libc-test |
└─────┘ └──────┘  └─────┘  └─────────────┘ └─────────────────┘
                     |            └───┐┌─────┘   ┌───────────┐
                     ↓                ↓↓      ┌──| libc-test |
                 ┌───────┐        ┌────────┐←─┘  └───────────┘
                 | image |───────→| rootfs |←─┐ ┌────────────┐
                 └───────┘        └────────┘  └─| other-test |
                 ┌────────┐           ↑         └────────────┘
                 | opencv |────→┌───────────┐
                 └────────┘  ┌─→| musl-libc |
                 ┌────────┐  |  └───────────┘
                 | ffmpeg |──┘
                 └────────┘
-------------------------------------------------------------------
图例：A 递归执行 B（A 依赖 B 的结果，执行 A 时自动先执行 B）
┌───┐  ┌───┐
| A |─→| B |
└───┘  └───┘
```

### 命令参考

如果下面的命令描述与行为不符，或怀疑此文档更新不及时，亦可直接查看[内联文档](xtask/src/main.rs#L48)。
如果发现 `error: no such subcommand: ...`，查看[命令简写](.cargo/config.toml)为哪些命令设置了别名。

---

> **NOTICE** 内联文档也是中英双语

---

#### **update-all**

更新工具链、依赖和 git 子模块。

如果没有递归克隆子模块，可以使用这个命令克隆。

```bash
cargo update-all
```

#### **check-style**

静态检查。设置多种编译选项，检查代码能否编译。

```bash
cargo check-style
```

#### **zircon-init**

下载 zircon 模式所需的二进制文件。

```bash
cargo zircon-init
```

#### **asm**

反汇并保存编指定架构的内核。默认保存到 `target/zcore.asm`。

```bash
cargo asm -m virt-riscv64 -o z.asm
```

#### **bin**

生成内核 raw 镜像到指定位置。默认输出到 `target/{arch}/release/zcore.bin`。

```bash
cargo bin -m virt-riscv64 -o z.bin
```

#### **qemu**

在 Qemu 中启动 zCore。这需要 Qemu 已经安装好了。

```bash
cargo qemu --arch riscv64 --smp 4
```

支持将 qemu 连接到 gdb：

```bash
cargo qemu --arch riscv64 --smp 4 --gdb 1234
```

#### **rootfs**

重建 Linux rootfs。直接执行这个命令会清空已有的为此架构构造的 rootfs 目录，重建最小的 rootfs。

```bash
cargo rootfs --arch riscv64
```

#### **musl-libs**

将 musl 动态库拷贝到 rootfs 目录对应位置。

```bash
cargo musl-libs --arch riscv64
```

#### **ffmpeg**

将 ffmpeg 动态库拷贝到 rootfs 目录对应位置。

```bash
cargo ffmpeg --arch riscv64
```

#### **opencv**

将 opencv 动态库拷贝到 rootfs 目录对应位置。如果 ffmpeg 已经放好了，opencv 将会编译出包含 ffmepg 支持的版本。

```bash
cargo opencv --arch riscv64
```

#### **libc-test**

将 libc 测试集拷贝到 rootfs 目录对应位置。

```bash
cargo libc-test --arch riscv64
```

#### **other-test**

将其他测试集拷贝到 rootfs 目录对应位置。

```bash
cargo other-test --arch riscv64
```

#### **image**

从 rootfs 目录构建 Linux rootfs 镜像文件。

```bash
cargo image --arch riscv64
```

#### **linux-libos**

在 linux libos 模式下启动 zCore 并执行位于指定路径的应用程序。

> **NOTICE** libos 模式只能执行单个应用程序，完成就会退出。

```bash
cargo linux-libos --args "/bin/busybox"
```

可以直接给应用程序传参数：

```bash
cargo linux-libos --args "/bin/busybox ls"
```

## 平台支持

### Qemu/virt

直接使用命令启动，参见[启动内核](#启动内核)和 [`qemu` 命令](#qemu)。

### 全志/哪吒

使用以下命令构造系统镜像：

```bash
cargo bin -m nezha -o z.bin
```

然后使用 [rustsbi-d1](https://github.com/rustsbi/rustsbi-d1) 将镜像部署到 Flash 或 DRAM。

另: 可以查看[README for D1 文档](docs/README-D1.md)获知更多D1开发板有关的操作指导。

### 赛昉/星光

使用以下命令构造系统镜像：

```bash
cargo bin -m visionfive -o z.bin
```

然后根据[此文档](docs/README-visionfive.md)的详细说明通过 u-boot 网络启动系统。

### 晶视/cr1825

使用以下命令构造系统镜像：

```bash
cargo bin -m cr1825 -o z.bin
```

然后通过 u-boot 网络启动系统。

## 其他

- [An English README](docs/README_EN.md)
- [开发者注意事项（草案）](docs/for-developers.md)
- [构建系统更新日志](xtask/CHANGELOG.md)


cd zCore
SMP=5 cargo build --features "linux board-fu740" --no-default-features --target riscv64.json -Z build-std=core,alloc -Z build-std-features=compiler-builtins-mem --release
rust-objcopy --binary-architecture=riscv64 ../target/riscv64/release/zcore --strip-all -O binary ../target/riscv64/release/zcore.bin
gzip -9 -cvf ../target/riscv64/release/zcore.bin > ./zcore.bin.gz
mkimage -f ../prebuilt/firmware/riscv/fu740_fdt.its ./zcore-fu740.itb
cp ./zcore-fu740.itb /home/y/tftproot/

setenv ipaddr 192.168.0.254
setenv serverip 192.168.0.107
tftp 0xa0000000 zcore-fu740.itb
pci enum
bootm 0xa0000000




sudo dd if=/dev/zero of=/dev/nvme0n1 ibs=512 count=2
sudo dd of=1.out if=/dev/nvme0n1 ibs=512 count=2


Usage: nvme io-passthru <device> [OPTIONS]

Send a user-defined IO command to the specified device via IOCTL passthrough,
return results.

Options:
  [  --opcode=<NUM>, -o <NUM> ]         --- opcode (required)
  [  --flags=<NUM>, -f <NUM> ]          --- command flags
  [  --prefill=<NUM>, -p <NUM> ]        --- prefill buffers with known
                                            byte-value, default 0
  [  --rsvd=<NUM>, -R <NUM> ]           --- value for reserved field
  [  --namespace-id=<NUM>, -n <NUM> ]   --- desired namespace
  [  --data-len=<NUM>, -l <NUM> ]       --- data I/O length (bytes)
  [  --metadata-len=<NUM>, -m <NUM> ]   --- metadata seg. length (bytes)
  [  --timeout=<NUM>, -t <NUM> ]        --- timeout value, in milliseconds
  [  --cdw2=<NUM>, -2 <NUM> ]           --- command dword 2 value
  [  --cdw3=<NUM>, -3 <NUM> ]           --- command dword 3 value
  [  --cdw10=<NUM>, -4 <NUM> ]          --- command dword 10 value
  [  --cdw11=<NUM>, -5 <NUM> ]          --- command dword 11 value
  [  --cdw12=<NUM>, -6 <NUM> ]          --- command dword 12 value
  [  --cdw13=<NUM>, -7 <NUM> ]          --- command dword 13 value
  [  --cdw14=<NUM>, -8 <NUM> ]          --- command dword 14 value
  [  --cdw15=<NUM>, -9 <NUM> ]          --- command dword 15 value
  [  --input-file=<FILE>, -i <FILE> ]   --- write/send file (default stdin)
  [  --raw-binary, -b ]                 --- dump output in binary format
  [  --show-command, -s ]               --- print command before sending
  [  --dry-run, -d ]                    --- show command instead of sending
  [  --read, -r ]                       --- set dataflow direction to receive
  [  --write, -w ]                      --- set dataflow direction to send



sudo nvme write --start-block=0 --block-count=1 --data-size=512 --metadata-size=0 --ref-tag=0 -data=2.in --app-tag-mask=0 --app-tag=0 --dsm=0 -v



  [  --start-block=<IONUM>, -s <IONUM> ] --- 64-bit addr of first block to
                                            access
  [  --block-count=<NUM>, -c <NUM> ]    --- number of blocks (zeroes based)
                                            on device to access
  [  --data-size=<IONUM>, -z <IONUM> ]  --- size of data in bytes
  [  --metadata-size=<IONUM>, -y <IONUM> ] --- size of metadata in bytes
  [  --ref-tag=<NUM>, -r <NUM> ]        --- reference tag (for end to end PI)
  [  --data=<FILE>, -d <FILE> ]         --- data file
  [  --metadata=<FILE>, -M <FILE> ]     --- metadata file
  [  --prinfo=<NUM>, -p <NUM> ]         --- PI and check field
  [  --app-tag-mask=<NUM>, -m <NUM> ]   --- app tag mask (for end to end PI)
  [  --app-tag=<NUM>, -a <NUM> ]        --- app tag (for end to end PI)
  [  --limited-retry, -l ]              --- limit num. media access attempts
  [  --force-unit-access, -f ]          --- force device to commit data
                                            before command completes
  [  --dir-type=<NUM>, -T <NUM> ]       --- directive type (for write-only)
  [  --dir-spec=<NUM>, -S <NUM> ]       --- directive specific (for
                                            write-only)
  [  --dsm=<NUM>, -D <NUM> ]            --- dataset management attributes
                                            (lower 16 bits)
  [  --show-command, -v ]               --- show command before sending
  [  --dry-run, -w ]                    --- show command instead of sending
  [  --latency, -t ]                    --- output latency statistics





The following are all implemented sub-commands:
  list                      List all NVMe devices and namespaces on machine
  list-subsys               List nvme subsystems
  id-ctrl                   Send NVMe Identify Controller
  id-ns                     Send NVMe Identify Namespace, display structure
  id-ns-granularity         Send NVMe Identify Namespace Granularity List, display structure
  list-ns                   Send NVMe Identify List, display structure
  list-ctrl                 Send NVMe Identify Controller List, display structure
  nvm-id-ctrl               Send NVMe Identify Controller NVM Command Set, display structure
  primary-ctrl-caps         Send NVMe Identify Primary Controller Capabilities
  list-secondary            List Secondary Controllers associated with a Primary Controller
  cmdset-ind-id-ns          I/O Command Set Independent Identify Namespace
  ns-descs                  Send NVMe Namespace Descriptor List, display structure
  id-nvmset                 Send NVMe Identify NVM Set List, display structure
  id-uuid                   Send NVMe Identify UUID List, display structure
  id-iocs                   Send NVMe Identify I/O Command Set, display structure
  id-domain                 Send NVMe Identify Domain List, display structure
  list-endgrp               Send NVMe Identify Endurance Group List, display structure
  create-ns                 Creates a namespace with the provided parameters
  delete-ns                 Deletes a namespace from the controller
  attach-ns                 Attaches a namespace to requested controller(s)
  detach-ns                 Detaches a namespace from requested controller(s)
  get-ns-id                 Retrieve the namespace ID of opened block device
  get-log                   Generic NVMe get log, returns log in raw format
  telemetry-log             Retrieve FW Telemetry log write to file
  fw-log                    Retrieve FW Log, show it
  changed-ns-list-log       Retrieve Changed Namespace List, show it
  smart-log                 Retrieve SMART Log, show it
  ana-log                   Retrieve ANA Log, show it
  error-log                 Retrieve Error Log, show it
  effects-log               Retrieve Command Effects Log, show it
  endurance-log             Retrieve Endurance Group Log, show it
  predictable-lat-log       Retrieve Predictable Latency per Nvmset Log, show it
  pred-lat-event-agg-log    Retrieve Predictable Latency Event Aggregate Log, show it
  persistent-event-log      Retrieve Presistent Event Log, show it
  endurance-event-agg-log   Retrieve Endurance Group Event Aggregate Log, show it
  lba-status-log            Retrieve LBA Status Information Log, show it
  resv-notif-log            Retrieve Reservation Notification Log, show it
  boot-part-log             Retrieve Boot Partition Log, show it
  get-feature               Get feature and show the resulting value
  device-self-test          Perform the necessary tests to observe the performance
  self-test-log             Retrieve the SELF-TEST Log, show it
  supported-log-pages       Retrieve the Supported Log pages details, show it
  set-feature               Set a feature and show the resulting value
  set-property              Set a property and show the resulting value
  get-property              Get a property and show the resulting value
  format                    Format namespace with new block format
  fw-commit                 Verify and commit firmware to a specific slot (fw-activate in old version < 1.2)
  fw-download               Download new firmware
  admin-passthru            Submit an arbitrary admin command, return results
  io-passthru               Submit an arbitrary IO command, return results
  security-send             Submit a Security Send command, return results
  security-recv             Submit a Security Receive command, return results
  get-lba-status            Submit a Get LBA Status command, return results
  capacity-mgmt             Submit Capacity Management Command, return results
  resv-acquire              Submit a Reservation Acquire, return results
  resv-register             Submit a Reservation Register, return results
  resv-release              Submit a Reservation Release, return results
  resv-report               Submit a Reservation Report, return results
  dsm                       Submit a Data Set Management command, return results
  copy                      Submit a Simple Copy command, return results
  flush                     Submit a Flush command, return results
  compare                   Submit a Compare command, return results
  read                      Submit a read command, return results
  write                     Submit a write command, return results
  write-zeroes              Submit a write zeroes command, return results
  write-uncor               Submit a write uncorrectable command, return results
  verify                    Submit a verify command, return results
  sanitize                  Submit a sanitize command
  sanitize-log              Retrieve sanitize log, show it
  reset                     Resets the controller
  subsystem-reset           Resets the subsystem
  ns-rescan                 Rescans the NVME namespaces
  show-regs                 Shows the controller registers or properties. Requires character device
  discover                  Discover NVMeoF subsystems
  connect-all               Discover and Connect to NVMeoF subsystems
  connect                   Connect to NVMeoF subsystem
  disconnect                Disconnect from NVMeoF subsystem
  disconnect-all            Disconnect from all connected NVMeoF subsystems
  gen-hostnqn               Generate NVMeoF host NQN
  show-hostnqn              Show NVMeoF host NQN
  dir-receive               Submit a Directive Receive command, return results
  dir-send                  Submit a Directive Send command, return results
  virt-mgmt                 Manage Flexible Resources between Primary and Secondary Controller
  rpmb                      Replay Protection Memory Block commands
  fid-support-effects-log   Submit Feature ID Support and Effects Log, Return result
  lockdown                  Submit a Lockdown command,return result
  version                   Shows the program version
  help                      Display this help

See 'nvme help <command>' for more information on a specific command

The following are all installed plugin extensions:
  intel           Intel vendor specific extensions
  amzn            Amazon vendor specific extensions
  lnvm            LightNVM specific extensions
  memblaze        Memblaze vendor specific extensions
  wdc             Western Digital vendor specific extensions
  huawei          Huawei vendor specific extensions
  netapp          NetApp vendor specific extensions
  toshiba         Toshiba NVME plugin
  micron          Micron vendor specific extensions
  seagate         Seagate vendor specific extensions
  virtium         Virtium vendor specific extensions
  shannon         Shannon vendor specific extensions
  dera            Dera vendor specific extensions
  sfx             ScaleFlux vendor specific extensions
  transcend       Transcend vendor specific extensions
  zns             Zoned Namespace Command Set
  nvidia          NVIDIA vendor specific extensions
  ymtc            Ymtc vendor specific extensions

See 'nvme <plugin> help' for more information on a plugin



obj-m += /target
obj-m += /


CURRENT_PATH:=$(shell pwd)
LINUX_KERNEL_PATH:=/lib/modules/$(shell uname -r)/build

all:
    $(MAKE) -C $(LINUX_KERNEL_PATH) M=$(CURRENT_PATH) modules
clean:
    $(MAKE) -C $(LINUX_KERNEL_PATH) M=$(CURRENT_PATH) clean
