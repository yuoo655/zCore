n  -append "LOG=warn" -drive file=nvme.img,if=none,id=nvm -device nvme,serial=xxxxx,drive=nvm --trace "pci_nvme_*"
WARNING: Image format was not specified for 'nvme.img' and probing guessed raw.
         Automatically detecting the format is dangerous for raw images, write operations on block 0 will be restricted.
         Specify the 'raw' format explicitly to remove the restrictions.
pci_nvme_pci_reset PCI Function Level Reset
VNC server running on 127.0.0.1:5900

OpenSBI v1.0
   ____                    _____ ____ _____
  / __ \                  / ____|  _ \_   _|
 | |  | |_ __   ___ _ __ | (___ | |_) || |
 | |  | | '_ \ / _ \ '_ \ \___ \|  _ < | |
 | |__| | |_) |  __/ | | |____) | |_) || |_
  \____/| .__/ \___|_| |_|_____/|____/_____|
        | |
        |_|

Platform Name             : riscv-virtio,qemu
Platform Features         : medeleg
Platform HART Count       : 1
Platform IPI Device       : aclint-mswi
Platform Timer Device     : aclint-mtimer @ 10000000Hz
Platform Console Device   : uart8250
Platform HSM Device       : ---
Platform Reboot Device    : sifive_test
Platform Shutdown Device  : sifive_test
Firmware Base             : 0x80000000
Firmware Size             : 252 KB
Runtime SBI Version       : 0.3

Domain0 Name              : root
Domain0 Boot HART         : 0
Domain0 HARTs             : 0*
Domain0 Region00          : 0x0000000002000000-0x000000000200ffff (I)
Domain0 Region01          : 0x0000000080000000-0x000000008003ffff ()
Domain0 Region02          : 0x0000000000000000-0xffffffffffffffff (R,W,X)
Domain0 Next Address      : 0x0000000080200000
Domain0 Next Arg1         : 0x00000000bfe00000
Domain0 Next Mode         : S-mode
Domain0 SysReset          : yes

Boot HART ID              : 0
Boot HART Domain          : root
Boot HART ISA             : rv64imafdcsuh
Boot HART Features        : scounteren,mcounteren,mcountinhibit,time
Boot HART PMP Count       : 16
Boot HART PMP Granularity : 4
Boot HART PMP Address Bits: 54
Boot HART MHPM Count      : 16
Boot HART MIDELEG         : 0x0000000000001666
Boot HART MEDELEG         : 0x0000000000f0b509

boot page table launched, sstatus = 0x8000000200046000
kernel (physical): 0000000080200000..000000008051e300
kernel (remapped): ffffffc080200000..ffffffc08051e300
device tree:       00000000bfe00000..00000000bfe013c6

hart0 is the primary hart.

[  0.000732 INFO  0 0:0 zcore::memory] MEMORY = 0xffffffc08031e068..0xffffffc08051e068
[  0.000770 INFO  0 0:0 kernel_hal::imp::boot] Primary CPU 0 init early...
[  0.000785 INFO  0 0:0 zcore_drivers::utils::devicetree] Loading device tree blob from 0xffffffc0bfe00000
[  0.000848 INFO  0 0:0 kernel_hal::imp::arch] Load kernel cmdline from DTB: "LOG=warn"
[  0.000869 INFO  0 0:0 kernel_hal::imp::arch] Load CPU clock frequency from DTB: 10000000 Hz
[  0.089761 INFO  0 0:0 kernel_hal::imp::arch] Load memory regions from DTB: [
    0x80000000..0x100000000,
]
[  0.095063 INFO  0 0:0 zcore] Boot options: BootOptions {
    cmdline: "LOG=warn",
    log_level: "warn",
    root_proc: "/bin/busybox?sh",
}
[  0.108381 INFO  0 0:0 kernel_hal::imp::boot] Primary CPU 0 init...
[  0.118290 INFO  0 0:0 kernel_hal::imp::arch::vm] FREE PHY MEM: 80520000..bfe00000
[  0.125324 INFO  0 0:0 kernel_hal::imp::arch::vm] FREE PHY MEM: bfe02000..100000000
[  0.131058 INFO  0 0:0 kernel_hal::imp::arch::vm] initialized kernel page table @ 0x8031f000
[  0.132980 INFO  0 0:0 zcore_drivers::utils::devicetree] Loading device tree blob from 0xffffffc0bfe00000
[  0.141044 INFO  0 0:0 zcore_drivers::builder::devicetree] device-tree: register interrupts for IrqDevice("riscv-plic"): UartDevice("uart16550-mmio"), irq_num=10
[  0.143026 ERROR 0 0:0 zcore_drivers::irq::riscv_plic] plic register_handler: 10
[  0.144112 INFO  0 0:0 zcore_drivers::utils::irq_manager] IRQ register handler 10
[  0.145755 INFO  0 0:0 zcore_drivers::builder::devicetree] device-tree: register interrupts for IrqDevice("riscv-intc-cpu0"): IrqDevice("riscv-plic"), irq_num=9
[  0.151588 INFO  0 0:0 zcore_drivers::bus::pci] 
[  0.152528 INFO  0 0:0 zcore_drivers::bus::pci] --------- PCI bus:device:function ---------
[  0.153527 WARN  0 0:0 pci] probe_function
[  0.154005 WARN  0 0:0 pci] PCI probe: 0x8 0x1b36 @ Location { bus: 0, device: 0, function: 0 }
[  0.155826 WARN  0 0:0 pci] BAR[0]: None
[  0.156413 WARN  0 0:0 pci] BAR[1]: None
[  0.156801 WARN  0 0:0 pci] BAR[2]: None
[  0.157191 WARN  0 0:0 pci] BAR[3]: None
[  0.157621 WARN  0 0:0 pci] BAR[4]: None
[  0.158009 WARN  0 0:0 pci] BAR[5]: None
[  0.158789 INFO  0 0:0 zcore_drivers::bus::pci] pci: 0:0:0 1b36:0008 (6 0) irq: 0:None
[  0.159812 WARN  0 0:0 zcore_drivers::bus::pci] NoResources, failed to initialize PCI device: 1b36:0008
[  0.160594 WARN  0 0:0 pci] probe_function
[  0.161006 WARN  0 0:0 pci] PCI probe: 0x10 0x1b36 @ Location { bus: 0, device: 1, function: 0 }
[  0.162187 WARN  0 0:0 pci] BAR[0]: Some(Memory(0, 3ffe, No, Bits64))
[  0.163450 WARN  0 0:0 pci] BAR[2]: None
[  0.163872 WARN  0 0:0 pci] BAR[3]: None
[  0.164260 WARN  0 0:0 pci] BAR[4]: None
[  0.164672 WARN  0 0:0 pci] BAR[5]: None
[  0.165068 INFO  0 0:0 zcore_drivers::bus::pci] pci: 0:1:0 1b36:0010 (1 8) irq: 0:Some(INTA)
[  0.166298 WARN  0 0:0 zcore_drivers::bus::pci] BAR0 set from 0x4 to 0x40000004
[  0.167146 WARN  0 0:0 zcore_drivers::bus::pci] PCI device has cap id 17 at 0x40
[  0.167816 WARN  0 0:0 zcore_drivers::bus::pci] PCI device has cap id 16 at 0x80
[  0.168415 WARN  0 0:0 zcore_drivers::bus::pci] PCI device has cap id 1 at 0x60
[  0.169665 WARN  0 0:0 zcore_drivers::bus::pci] MSI not found, using PCI interrupt
[  0.170429 WARN  0 0:0 zcore_drivers::bus::pci] pci device enable done
pci_nvme_mmio_write addr 0x24 data 0x1f001f size 4
pci_nvme_mmio_aqattr wrote MMIO, admin queue attributes=0x1f001f
pci_nvme_mmio_write addr 0x28 data 0xbfe0a000 size 4
pci_nvme_mmio_asqaddr wrote MMIO, admin submission queue address=0xbfe0a000
pci_nvme_mmio_write addr 0x30 data 0xbfe0c000 size 4
pci_nvme_mmio_acqaddr wrote MMIO, admin completion queue address=0xbfe0c000
pci_nvme_mmio_write addr 0x14 data 0x460001 size 4
pci_nvme_mmio_cfg wrote MMIO, config controller config=0x460001
pci_nvme_setfeat_timestamp set feature timestamp = 0x0
pci_nvme_mmio_start_success setting controller enable bit succeeded
pci_nvme_mmio_read addr 0x1c size 4
pci_nvme_mmio_write addr 0x1000 data 0x1 size 4
pci_nvme_mmio_doorbell_sq sqid 0 new_tail 1
pci_nvme_admin_cmd cid 4120 sqid 0 opc 0x6 opname 'NVME_ADM_CMD_IDENTIFY'
pci_nvme_identify cid 4120 cns 0x1 ctrlid 0 csi 0x0
pci_nvme_identify_ctrl identify controller
pci_nvme_map_prp trans_len 4096 len 4096 prp1 0xbfe08000 prp2 0x0 num_prps 2
pci_nvme_map_addr addr 0xbfe08000 len 4096
pci_nvme_enqueue_req_completion cid 4120 cqid 0 dw0 0x0 dw1 0x0 status 0x0
pci_nvme_irq_pin pulsing IRQ pin
sifive_plic_set_pending: irq=33
pci_nvme_mmio_write addr 0x1004 data 0x1 size 4
pci_nvme_mmio_doorbell_cq cqid 0 new_head 1
sifive_plic_set_pending: irq=33
pci_nvme_mmio_write addr 0x1000 data 0x2 size 4
pci_nvme_mmio_doorbell_sq sqid 0 new_tail 2
pci_nvme_admin_cmd cid 4122 sqid 0 opc 0x9 opname 'NVME_ADM_CMD_SET_FEATURES'
pci_nvme_setfeat cid 4122 nsid 0x1 fid 0x7 save 0x0 cdw11 0x0
pci_nvme_enqueue_req_completion cid 4122 cqid 0 dw0 0x0 dw1 0x0 status 0x410f
pci_nvme_err_req_status cid 4122 nsid 0 status 0x410f opc 0x9
pci_nvme_irq_pin pulsing IRQ pin
sifive_plic_set_pending: irq=33
pci_nvme_mmio_write addr 0x1004 data 0x1 size 4
pci_nvme_mmio_doorbell_cq cqid 0 new_head 1
pci_nvme_mmio_write addr 0x1000 data 0x3 size 4
pci_nvme_mmio_doorbell_sq sqid 0 new_tail 3
pci_nvme_mmio_write addr 0x1004 data 0x2 size 4
pci_nvme_mmio_doorbell_cq cqid 0 new_head 2
sifive_plic_set_pending: irq=33
pci_nvme_admin_cmd cid 4123 sqid 0 opc 0x5 opname 'NVME_ADM_CMD_CREATE_CQ'
pci_nvme_create_cq create completion queue, addr=0xbfe0c000, cqid=1, vector=0, qsize=1023, qflags=3, ien=1
pci_nvme_enqueue_req_completion cid 4123 cqid 0 dw0 0x0 dw1 0x0 status 0x0
pci_nvme_irq_pin pulsing IRQ pin
sifive_plic_set_pending: irq=33
pci_nvme_mmio_write addr 0x1000 data 0x4 size 4
pci_nvme_mmio_doorbell_sq sqid 0 new_tail 4
pci_nvme_mmio_write addr 0x1004 data 0x3 size 4
pci_nvme_mmio_doorbell_cq cqid 0 new_head 3
sifive_plic_set_pending: irq=33
pci_nvme_admin_cmd cid 8216 sqid 0 opc 0x1 opname 'NVME_ADM_CMD_CREATE_SQ'
pci_nvme_create_sq create submission queue, addr=0xbfe0a000, sqid=1, cqid=1, qsize=1023, qflags=1
pci_nvme_enqueue_req_completion cid 8216 cqid 0 dw0 0x0 dw1 0x0 status 0x0
pci_nvme_irq_pin pulsing IRQ pin
sifive_plic_set_pending: irq=33
[  0.251590 INFO  0 0:0 zcore_drivers::bus::pci] ---------
[  0.252219 INFO  0 0:0 zcore_drivers::bus::pci] 
[  0.256336 INFO  0 0:0 zcore] hello world
[  0.257307 ERROR 0 0:0 zcore_drivers::irq::riscv_plic] plic register_handler: 33
[  0.257978 INFO  0 0:0 zcore_drivers::utils::irq_manager] IRQ register handler 33
sifive_plic_set_claimed: irq=33
sifive_plic_set_claimed: irq=33
sifive_plic_set_claimed: irq=33
sifive_plic_set_claimed: irq=33
sifive_plic_set_claimed: irq=33
sifive_plic_set_claimed: irq=33
sifive_plic_set_claimed: irq=33
[  0.259130 WARN  0 0:0 zcore_drivers::nvme::interface] async write block
[  0.259878 WARN  0 0:0 zcore_drivers::nvme::interface] write block
pci_nvme_mmio_write addr 0x1008 data 0x5 size 4
pci_nvme_mmio_doorbell_sq sqid 1 new_tail 5
pci_nvme_io_cmd cid 4120 nsid 0x1 sqid 1 opc 0x6 opname 'NVME_NVM_CMD_UNKNOWN'
pci_nvme_err_invalid_opc invalid opcode 0x6
pci_nvme_enqueue_req_completion cid 4120 cqid 1 dw0 0x0 dw1 0x0 status 0x4001
pci_nvme_err_req_status cid 4120 nsid 0 status 0x4001 opc 0x6
pci_nvme_io_cmd cid 4122 nsid 0x1 sqid 1 opc 0x9 opname 'NVME_NVM_CMD_DSM'
pci_nvme_dsm nr 8 attr 0x0
pci_nvme_enqueue_req_completion cid 4122 cqid 1 dw0 0x0 dw1 0x0 status 0x0
pci_nvme_io_cmd cid 4123 nsid 0x1 sqid 1 opc 0x5 opname 'NVME_NVM_CMD_COMPARE'
pci_nvme_compare cid 4123 nsid 1 slba 0x303ff0001 nlb 1
pci_nvme_err_invalid_lba_range Invalid LBA start=12951945217 len=1 limit=32768
pci_nvme_enqueue_req_completion cid 4123 cqid 1 dw0 0x0 dw1 0x0 status 0x4080
pci_nvme_err_req_status cid 4123 nsid 1 status 0x4080 opc 0x5
pci_nvme_io_cmd cid 8216 nsid 0x1 sqid 1 opc 0x1 opname 'NVME_NVM_CMD_WRITE'
pci_nvme_write cid 8216 opname 'NVME_NVM_CMD_WRITE' nsid 1 nlb 1 count 512 lba 0x1000103ff0001
pci_nvme_err_invalid_lba_range Invalid LBA start=281479338721281 len=1 limit=32768
pci_nvme_enqueue_req_completion cid 8216 cqid 1 dw0 0x0 dw1 0x0 status 0x4080
pci_nvme_err_req_status cid 8216 nsid 1 status 0x4080 opc 0x1
pci_nvme_io_cmd cid 273 nsid 0x1 sqid 1 opc 0x1 opname 'NVME_NVM_CMD_WRITE'
pci_nvme_write cid 273 opname 'NVME_NVM_CMD_WRITE' nsid 1 nlb 1 count 512 lba 0x0
pci_nvme_map_prp trans_len 512 len 512 prp1 0x8026b288 prp2 0x0 num_prps 1
pci_nvme_map_addr addr 0x8026b288 len 512
pci_nvme_irq_pin pulsing IRQ pin
[ pci_nvme_rw_cb cid 273 blk 'nvm'
pci_nvme_rw_complete_cb cid 273 blk 'nvm'
pci_nvme_enqueue_req_completion cid 273 cqid 1 dw0 0x0 dw1 0x0 status 0x0
pci_nvme_irq_pin pulsing IRQ pin
 0.260866 WARN  0 0:0 zcore_drivers::nvme::interface] send write command cid 273 tail 4
sifive_plic_set_pending: irq=33
sifive_plic_set_claimed: irq=33
[  0.263299 WARN  0 0:0 zcore_drivers::nvme::interface] nvme device irq 33
[  0.264096 WARN  0 0:0 zcore_drivers::nvme::interface] tail 4
[  0.264791 WARN  0 0:0 zcore_drivers::nvme::interface] status NvmeCompletion {
    result: 0x0,
    sq_head: 0x5,
    sq_id: 0x1,
    command_id: 0x111,
    status: 0x1,
}
pci_nvme_mmio_write addr 0x100c data 0x5 size 4
pci_nvme_mmio_doorbell_cq cqid 1 new_head 5
[  0.266493 INFO  0 0:0 zcore] wake_by_ref
[  0.266955 ERROR 0 0:0 zcore_drivers::irq::riscv_plic] riscv plic handle irq: 33
sifive_plic_set_claimed: irq=33
[  0.267859 WARN  0 0:0 zcore_drivers::nvme::interface] async write block
[  0.268330 WARN  0 0:0 zcore_drivers::nvme::interface] write block
pci_nvme_mmio_write addr 0x1008 data 0x6 size 4
pci_nvme_mmio_doorbell_sq sqid 1 new_tail 6
[pci_nvme_io_cmd cid 273 nsid 0x1 sqid 1 opc 0x1 opname 'NVME_NVM_CMD_WRITE'
pci_nvme_write cid 273 opname 'NVME_NVM_CMD_WRITE' nsid 1 nlb 1 count 512 lba 0x1
pci_nvme_map_prp trans_len 512 len 512 prp1 0x8026b488 prp2 0x0 num_prps 1
pci_nvme_map_addr addr 0x8026b488 len 512
 pci_nvme_rw_cb cid 273 blk 'nvm'
pci_nvme_rw_complete_cb cid 273 blk 'nvm'
pci_nvme_enqueue_req_completion cid 273 cqid 1 dw0 0x0 dw1 0x0 status 0x0
pci_nvme_irq_pin pulsing IRQ pin
 0.268784 WARN  0 0:0 zcore_drivers::nvme::interface] send write command cid 273 tail 5
QEMU: Terminated
y@y:~/dev/yuoo655/zCore$ qemu-system-riscv64 -smp 1 -machine virt -bios default -m 2G -no-reboot -serial mon:stdio -serial file:/tmp/serial.out -kernel target/riscv64/release/zcore.bin  -append "LOG=warn" -drive file=nvme.img,if=none,id=nvm -device nvme,serial=xxxxx,drive=nvm --trace "pci_nvme_*"
WARNING: Image format was not specified for 'nvme.img' and probing guessed raw.
         Automatically detecting the format is dangerous for raw images, write operations on block 0 will be restricted.
         Specify the 'raw' format explicitly to remove the restrictions.
pci_nvme_pci_reset PCI Function Level Reset
VNC server running on 127.0.0.1:5900

OpenSBI v1.0
   ____                    _____ ____ _____
  / __ \                  / ____|  _ \_   _|
 | |  | |_ __   ___ _ __ | (___ | |_) || |
 | |  | | '_ \ / _ \ '_ \ \___ \|  _ < | |
 | |__| | |_) |  __/ | | |____) | |_) || |_
  \____/| .__/ \___|_| |_|_____/|____/_____|
        | |
        |_|

Platform Name             : riscv-virtio,qemu
Platform Features         : medeleg
Platform HART Count       : 1
Platform IPI Device       : aclint-mswi
Platform Timer Device     : aclint-mtimer @ 10000000Hz
Platform Console Device   : uart8250
Platform HSM Device       : ---
Platform Reboot Device    : sifive_test
Platform Shutdown Device  : sifive_test
Firmware Base             : 0x80000000
Firmware Size             : 252 KB
Runtime SBI Version       : 0.3

Domain0 Name              : root
Domain0 Boot HART         : 0
Domain0 HARTs             : 0*
Domain0 Region00          : 0x0000000002000000-0x000000000200ffff (I)
Domain0 Region01          : 0x0000000080000000-0x000000008003ffff ()
Domain0 Region02          : 0x0000000000000000-0xffffffffffffffff (R,W,X)
Domain0 Next Address      : 0x0000000080200000
Domain0 Next Arg1         : 0x00000000bfe00000
Domain0 Next Mode         : S-mode
Domain0 SysReset          : yes

Boot HART ID              : 0
Boot HART Domain          : root
Boot HART ISA             : rv64imafdcsuh
Boot HART Features        : scounteren,mcounteren,mcountinhibit,time
Boot HART PMP Count       : 16
Boot HART PMP Granularity : 4
Boot HART PMP Address Bits: 54
Boot HART MHPM Count      : 16
Boot HART MIDELEG         : 0x0000000000001666
Boot HART MEDELEG         : 0x0000000000f0b509

boot page table launched, sstatus = 0x8000000200046000
kernel (physical): 0000000080200000..000000008051e300
kernel (remapped): ffffffc080200000..ffffffc08051e300
device tree:       00000000bfe00000..00000000bfe013c6

hart0 is the primary hart.

[  0.000634 INFO  0 0:0 zcore::memory] MEMORY = 0xffffffc08031e068..0xffffffc08051e068
[  0.000665 INFO  0 0:0 kernel_hal::imp::boot] Primary CPU 0 init early...
[  0.000678 INFO  0 0:0 zcore_drivers::utils::devicetree] Loading device tree blob from 0xffffffc0bfe00000
[  0.000737 INFO  0 0:0 kernel_hal::imp::arch] Load kernel cmdline from DTB: "LOG=warn"
[  0.000765 INFO  0 0:0 kernel_hal::imp::arch] Load CPU clock frequency from DTB: 10000000 Hz
[  0.078897 INFO  0 0:0 kernel_hal::imp::arch] Load memory regions from DTB: [
    0x80000000..0x100000000,
]
[  0.083295 INFO  0 0:0 zcore] Boot options: BootOptions {
    cmdline: "LOG=warn",
    log_level: "warn",
    root_proc: "/bin/busybox?sh",
}
[  0.095279 INFO  0 0:0 kernel_hal::imp::boot] Primary CPU 0 init...
[  0.102262 INFO  0 0:0 kernel_hal::imp::arch::vm] FREE PHY MEM: 80520000..bfe00000
[  0.108254 INFO  0 0:0 kernel_hal::imp::arch::vm] FREE PHY MEM: bfe02000..100000000
[  0.119128 INFO  0 0:0 kernel_hal::imp::arch::vm] initialized kernel page table @ 0x8031f000
[  0.120460 INFO  0 0:0 zcore_drivers::utils::devicetree] Loading device tree blob from 0xffffffc0bfe00000
[  0.127009 INFO  0 0:0 zcore_drivers::builder::devicetree] device-tree: register interrupts for IrqDevice("riscv-plic"): UartDevice("uart16550-mmio"), irq_num=10
[  0.128835 ERROR 0 0:0 zcore_drivers::irq::riscv_plic] plic register_handler: 10
[  0.129802 INFO  0 0:0 zcore_drivers::utils::irq_manager] IRQ register handler 10
[  0.131210 INFO  0 0:0 zcore_drivers::builder::devicetree] device-tree: register interrupts for IrqDevice("riscv-intc-cpu0"): IrqDevice("riscv-plic"), irq_num=9
[  0.136019 INFO  0 0:0 zcore_drivers::bus::pci] 
[  0.136750 INFO  0 0:0 zcore_drivers::bus::pci] --------- PCI bus:device:function ---------
[  0.137581 WARN  0 0:0 pci] probe_function
[  0.138007 WARN  0 0:0 pci] PCI probe: 0x8 0x1b36 @ Location { bus: 0, device: 0, function: 0 }
[  0.139458 WARN  0 0:0 pci] BAR[0]: None
[  0.139955 WARN  0 0:0 pci] BAR[1]: None
[  0.140309 WARN  0 0:0 pci] BAR[2]: None
[  0.140643 WARN  0 0:0 pci] BAR[3]: None
[  0.140977 WARN  0 0:0 pci] BAR[4]: None
[  0.141315 WARN  0 0:0 pci] BAR[5]: None
[  0.141972 INFO  0 0:0 zcore_drivers::bus::pci] pci: 0:0:0 1b36:0008 (6 0) irq: 0:None
[  0.142793 WARN  0 0:0 zcore_drivers::bus::pci] NoResources, failed to initialize PCI device: 1b36:0008
[  0.143456 WARN  0 0:0 pci] probe_function
[  0.143795 WARN  0 0:0 pci] PCI probe: 0x10 0x1b36 @ Location { bus: 0, device: 1, function: 0 }
[  0.144766 WARN  0 0:0 pci] BAR[0]: Some(Memory(0, 3ffe, No, Bits64))
[  0.145758 WARN  0 0:0 pci] BAR[2]: None
[  0.146101 WARN  0 0:0 pci] BAR[3]: None
[  0.146430 WARN  0 0:0 pci] BAR[4]: None
[  0.146759 WARN  0 0:0 pci] BAR[5]: None
[  0.147104 INFO  0 0:0 zcore_drivers::bus::pci] pci: 0:1:0 1b36:0010 (1 8) irq: 0:Some(INTA)
[  0.148053 WARN  0 0:0 zcore_drivers::bus::pci] BAR0 set from 0x4 to 0x40000004
[  0.148772 WARN  0 0:0 zcore_drivers::bus::pci] PCI device has cap id 17 at 0x40
[  0.149335 WARN  0 0:0 zcore_drivers::bus::pci] PCI device has cap id 16 at 0x80
[  0.149817 WARN  0 0:0 zcore_drivers::bus::pci] PCI device has cap id 1 at 0x60
[  0.150778 WARN  0 0:0 zcore_drivers::bus::pci] MSI not found, using PCI interrupt
[  0.151345 WARN  0 0:0 zcore_drivers::bus::pci] pci device enable done
pci_nvme_mmio_write addr 0x24 data 0x1f001f size 4
pci_nvme_mmio_aqattr wrote MMIO, admin queue attributes=0x1f001f
pci_nvme_mmio_write addr 0x28 data 0xbfe0a000 size 4
pci_nvme_mmio_asqaddr wrote MMIO, admin submission queue address=0xbfe0a000
pci_nvme_mmio_write addr 0x30 data 0xbfe0c000 size 4
pci_nvme_mmio_acqaddr wrote MMIO, admin completion queue address=0xbfe0c000
pci_nvme_mmio_write addr 0x14 data 0x460001 size 4
pci_nvme_mmio_cfg wrote MMIO, config controller config=0x460001
pci_nvme_setfeat_timestamp set feature timestamp = 0x0
pci_nvme_mmio_start_success setting controller enable bit succeeded
pci_nvme_mmio_read addr 0x1c size 4
pci_nvme_mmio_write addr 0x1000 data 0x1 size 4
pci_nvme_mmio_doorbell_sq sqid 0 new_tail 1
pci_nvme_admin_cmd cid 4120 sqid 0 opc 0x6 opname 'NVME_ADM_CMD_IDENTIFY'
pci_nvme_identify cid 4120 cns 0x1 ctrlid 0 csi 0x0
pci_nvme_identify_ctrl identify controller
pci_nvme_map_prp trans_len 4096 len 4096 prp1 0xbfe08000 prp2 0x0 num_prps 2
pci_nvme_map_addr addr 0xbfe08000 len 4096
pci_nvme_enqueue_req_completion cid 4120 cqid 0 dw0 0x0 dw1 0x0 status 0x0
pci_nvme_irq_pin pulsing IRQ pin
sifive_plic_set_pending: irq=33
pci_nvme_mmio_write addr 0x1004 data 0x1 size 4
pci_nvme_mmio_doorbell_cq cqid 0 new_head 1
sifive_plic_set_pending: irq=33
pci_nvme_mmio_write addr 0x1000 data 0x2 size 4
pci_nvme_mmio_doorbell_sq sqid 0 new_tail 2
pci_nvme_admin_cmd cid 4122 sqid 0 opc 0x9 opname 'NVME_ADM_CMD_SET_FEATURES'
pci_nvme_setfeat cid 4122 nsid 0x1 fid 0x7 save 0x0 cdw11 0x0
pci_nvme_enqueue_req_completion cid 4122 cqid 0 dw0 0x0 dw1 0x0 status 0x410f
pci_nvme_err_req_status cid 4122 nsid 0 status 0x410f opc 0x9
pci_nvme_mmio_write addr 0x1004 data 0x1 size 4
pci_nvme_mmio_doorbell_cq cqid 0 new_head 1
pci_nvme_irq_pin pulsing IRQ pin
sifive_plic_set_pending: irq=33
pci_nvme_mmio_write addr 0x1000 data 0x3 size 4
pci_nvme_mmio_doorbell_sq sqid 0 new_tail 3
pci_nvme_mmio_write addr 0x1004 data 0x2 size 4
pci_nvme_mmio_doorbell_cq cqid 0 new_head 2
pci_nvme_admin_cmd cid 4123 sqid 0 opc 0x5 opname 'NVME_ADM_CMD_CREATE_CQ'
pci_nvme_create_cq create completion queue, addr=0xbfe0c000, cqid=1, vector=0, qsize=1023, qflags=3, ien=1
pci_nvme_enqueue_req_completion cid 4123 cqid 0 dw0 0x0 dw1 0x0 status 0x0
pci_nvme_mmio_write addr 0x1000 data 0x4 size 4
pci_nvme_mmio_doorbell_sq sqid 0 new_tail 4
pci_nvme_irq_pin pulsing IRQ pin
pci_nvme_admin_cmd cid 8216 sqid 0 opc 0x1 opname 'NVME_ADM_CMD_CREATE_SQ'
pci_nvme_create_sq create submission queue, addr=0xbfe0a000, sqid=1, cqid=1, qsize=1023, qflags=1
pci_nvme_enqueue_req_completion cid 8216 cqid 0 dw0 0x0 dw1 0x0 status 0x0
pci_nvme_irq_pin pulsing IRQ pin
pci_nvme_mmio_write addr 0x1004 data 0x3 size 4
pci_nvme_mmio_doorbell_cq cqid 0 new_head 3
[  0.237537 INFO  0 0:0 zcore_drivers::bus::pci] ---------
[  0.238047 INFO  0 0:0 zcore_drivers::bus::pci] 
[  0.239379 INFO  0 0:0 zcore] hello world
[  0.240193 ERROR 0 0:0 zcore_drivers::irq::riscv_plic] plic register_handler: 33
[  0.240693 INFO  0 0:0 zcore_drivers::utils::irq_manager] IRQ register handler 33
sifive_plic_set_claimed: irq=33
sifive_plic_set_claimed: irq=33
sifive_plic_set_claimed: irq=33
sifive_plic_set_claimed: irq=33
sifive_plic_set_claimed: irq=33
sifive_plic_set_claimed: irq=33
sifive_plic_set_claimed: irq=33
[  0.241583 WARN  0 0:0 zcore_drivers::nvme::interface] async write block
[  0.242173 WARN  0 0:0 zcore_drivers::nvme::interface] write block
pci_nvme_mmio_write addr 0x1008 data 0x5 size 4
pci_nvme_mmio_doorbell_sq sqid 1 new_tail 5
pci_nvme_io_cmd cid 4120 nsid 0x1 sqid 1 opc 0x6 opname 'NVME_NVM_CMD_UNKNOWN'
pci_nvme_err_invalid_opc invalid opcode 0x6
pci_nvme_enqueue_req_completion cid 4120 cqid 1 dw0 0x0 dw1 0x0 status 0x4001
pci_nvme_err_req_status cid 4120 nsid 0 status 0x4001 opc 0x6
pci_nvme_io_cmd cid 4122 nsid 0x1 sqid 1 opc 0x9 opname 'NVME_NVM_CMD_DSM'
pci_nvme_dsm nr 8 attr 0x0
pci_nvme_enqueue_req_completion cid 4122 cqid 1 dw0 0x0 dw1 0x0 status 0x0
pci_nvme_io_cmd cid 4123 nsid 0x1 sqid 1 opc 0x5 opname 'NVME_NVM_CMD_COMPARE'
pci_nvme_compare cid 4123 nsid 1 slba 0x303ff0001 nlb 1
pci_nvme_err_invalid_lba_range Invalid LBA start=12951945217 len=1 limit=32768
pci_nvme_enqueue_req_completion cid 4123 cqid 1 dw0 0x0 dw1 0x0 status 0x4080
pci_nvme_err_req_status cid 4123 nsid 1 status 0x4080 opc 0x5
pci_nvme_io_cmd cid 8216 nsid 0x1 sqid 1 opc 0x1 opname 'NVME_NVM_CMD_WRITE'
pci_nvme_write cid 8216 opname 'NVME_NVM_CMD_WRITE' nsid 1 nlb 1 count 512 lba 0x1000103ff0001
pci_nvme_err_invalid_lba_range Invalid LBA start=281479338721281 len=1 limit=32768
pci_nvme_enqueue_req_completion cid 8216 cqid 1 dw0 0x0 dw1 0x0 status 0x4080
pci_nvme_err_req_status cid 8216 nsid 1 status 0x4080 opc 0x1
pci_nvme_io_cmd cid 273 nsid 0x1 sqid 1 opc 0x1 opname 'NVME_NVM_CMD_WRITE'
pci_nvme_write cid 273 opname 'NVME_NVM_CMD_WRITE' nsid 1 nlb 1 count 512 lba 0x0
pci_nvme_map_prp trans_len 512 len 512 prp1 0x8026b288 prp2 0x0 num_prps 1
pci_nvme_map_addr addr 0x8026b288 len 512
pci_nvme_irq_pin pulsing IRQ pin
pci_nvme_rw_cb cid 273 blk 'nvm'
pci_nvme_rw_complete_cb cid 273 blk 'nvm'
pci_nvme_enqueue_req_completion cid 273 cqid 1 dw0 0x0 dw1 0x0 status 0x0
pci_nvme_irq_pin pulsing IRQ pin
[  0.243172 WARN  0 0:0 zcore_drivers::nvme::interface] send write command cid 273 tail 4
sifive_plic_set_pending: irq=33
sifive_plic_set_claimed: irq=33
[  0.245096 WARN  0 0:0 zcore_drivers::nvme::interface] nvme device irq 33
[  0.245688 WARN  0 0:0 zcore_drivers::nvme::interface] tail 4
[  0.246180 WARN  0 0:0 zcore_drivers::nvme::interface] status NvmeCompletion {
    result: 0x0,
    sq_head: 0x5,
    sq_id: 0x1,
    command_id: 0x111,
    status: 0x1,
}
pci_nvme_mmio_write addr 0x100c data 0x5 size 4
pci_nvme_mmio_doorbell_cq cqid 1 new_head 5
sifive_plic_set_pending: irq=33
[  0.247625 INFO  0 0:0 zcore] wake_by_ref
[  0.248071 ERROR 0 0:0 zcore_drivers::irq::riscv_plic] riscv plic handle irq: 33
sifive_plic_set_claimed: irq=33
[  0.248980 WARN  0 0:0 zcore_drivers::nvme::interface] async write block
[  0.249412 WARN  0 0:0 zcore_drivers::nvme::interface] write block
pci_nvme_mmio_write addr 0x1008 data 0x6 size 4
pci_nvme_mmio_doorbell_sq sqid 1 new_tail 6
ci_nvme_io_cmd cid 273 nsid 0x1 sqid 1 opc 0x1 opname 'NVME_NVM_CMD_WRITE'
pci_nvme_write cid 273 opname 'NVME_NVM_CMD_WRITE' nsid 1 nlb 1 count 512 lba 0x1
pci_nvme_map_prp trans_len 512 len 512 prp1 0x8026b488 prp2 0x0 num_prps 1
pci_nvme_map_addr addr 0x8026b488 len 512
m[  0.249858 pci_nvme_rw_cb cid 273 blk 'nvm'
pci_nvme_rw_complete_cb cid 273 blk 'nvm'
pci_nvme_enqueue_req_completion cid 273 cqid 1 dw0 0x0 dw1 0x0 status 0x0
pci_nvme_irq_pin pulsing IRQ pin
sifive_plic_set_pending: irq=33
WARN  0 0:0 zcore_drivers::nvme::interface] send write command cid 273 tail 5
sifive_plic_set_pending: irq=33
sifive_plic_set_claimed: irq=33
[  0.250796 WARN  0 0:0 zcore_drivers::nvme::interface] nvme device irq 33
[  0.251259 WARN  0 0:0 zcore_drivers::nvme::interface] tail 5
[  0.251669 WARN  0 0:0 zcore_drivers::nvme::interface] status NvmeCompletion {
    result: 0x0,
    sq_head: 0x6,
    sq_id: 0x1,
    command_id: 0x111,
    status: 0x1,
}
pci_nvme_mmio_write addr 0x100c data 0x6 size 4
pci_nvme_mmio_doorbell_cq cqid 1 new_head 6
sifive_plic_set_pending: irq=33
[  0.252580 INFO  0 0:0 zcore] wake_by_ref
[  0.252916 ERROR 0 0:0 zcore_drivers::irq::riscv_plic] riscv plic handle irq: 33
sifive_plic_set_claimed: irq=33
