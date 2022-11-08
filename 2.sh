    root_proc: "/bin/busybox?sh",
}
[  1.913402 DEBUG 0 0:0 zcore::memory] init_frame_allocator regions: [1000..a0000, 100000..800000, 808000..810000, 1400000..7afcf000, 7eb1b000..7eca2000, 7f211000..7f213000, 7fa1b000..7fa1d000, 7fe00000..7feed000, 100000000..140000000, 140016000..280000000]
[  1.914509 INFO  0 0:0 zcore::memory] Frame allocator: add range 0x1000..0xa0000
[  1.914769 INFO  0 0:0 zcore::memory] Frame allocator: add range 0x100000..0x800000
[  1.915025 INFO  0 0:0 zcore::memory] Frame allocator: add range 0x808000..0x810000
[  1.915868 INFO  0 0:0 zcore::memory] Frame allocator: add range 0x1400000..0x7afcf000
[  1.916113 INFO  0 0:0 zcore::memory] Frame allocator: add range 0x7eb1b000..0x7eca2000
[  1.916351 INFO  0 0:0 zcore::memory] Frame allocator: add range 0x7f211000..0x7f213000
[  1.916590 INFO  0 0:0 zcore::memory] Frame allocator: add range 0x7fa1b000..0x7fa1d000
[  1.916844 INFO  0 0:0 zcore::memory] Frame allocator: add range 0x7fe00000..0x7feed000
[  1.917389 INFO  0 0:0 zcore::memory] Frame allocator: add range 0x100000000..0x140000000
[  1.919611 INFO  0 0:0 zcore::memory] Frame allocator: add range 0x140016000..0x280000000
                                                                  0x800000000

[  1.939148 WARN  0 0:0 pci] BAR[5]: None
[  1.939318 INFO  0 0:0 zcore_drivers::bus::pci] pci: 0:2:0 1b36:0010 (1 8) irq: 11:Some(INTA)
[  1.939798 INFO  0 0:0 zcore_drivers::bus::pci] addr 0x800000000
[  1.940149 WARN  0 0:0 zcore_drivers::bus::pci] BAR0 set from 0x4 to 0x4
[  1.940492 WARN  0 0:0 zcore_drivers::bus::pci] MSI not found, using PCI interrupt
[  1.940754 WARN  0 0:0 zcore_drivers::bus::pci] pci device enable done
[  1.941021 INFO  0 0:0 zcore_drivers::bus::pci] vaddr 0xffff800800000000