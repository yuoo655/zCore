use super::{phys_to_virt, PAGE_SIZE};
use core::ptr::{read_volatile, write_volatile};

pub const IO_BASE: u64 = 0x6008_0000;
pub const IO_BUS_ADDR: u64 = 0x6008_0000;
pub const IO_SIZE: u64 = 0x10000;

pub const DBI_BASE: u64 = 0xe0000_0000;
pub const ATU_BASE: u64 = DBI_BASE + DEFAULT_DBI_ATU_OFFSET;
//IO base - IO size: pci->io.phys_start - pci->io.size
pub const CFG_BASE: u64 = IO_BASE - IO_SIZE;
// IO size
pub const CFG_SIZE: u64 = IO_SIZE;

const DEFAULT_DBI_ATU_OFFSET: u64 = (0x3 << 20);
pub const FIRST_BUSNO: u32 = 0;

// iATU Unroll-specific register definitions
// From 4.80 core version the address translation will be made by unroll.
// The registers are offset from atu_base
const PCIE_ATU_UNR_REGION_CTRL1: u32 =     0x00;
const PCIE_ATU_UNR_REGION_CTRL2: u32 = 0x04;
const PCIE_ATU_UNR_LOWER_BASE: u32 = 0x08;
const PCIE_ATU_UNR_UPPER_BASE: u32 = 0x0c;
const PCIE_ATU_UNR_LIMIT: u32 = 0x10;
const PCIE_ATU_UNR_LOWER_TARGET: u32 = 0x14;
const PCIE_ATU_UNR_UPPER_TARGET: u32 = 0x18;

const PCIE_ATU_REGION_INDEX1: u32 = (0x1 << 0);
const PCIE_ATU_REGION_INDEX0: u32 = (0x0 << 0);
const PCIE_ATU_TYPE_MEM: u32 = (0x0 << 0);
const PCIE_ATU_TYPE_IO: u32 = (0x2 << 0);
const PCIE_ATU_TYPE_CFG0: u32 = (0x4 << 0);
const PCIE_ATU_TYPE_CFG1: u32 = (0x5 << 0);
const PCIE_ATU_ENABLE: u32 = (0x1 << 31);
const PCIE_ATU_BAR_MODE_ENABLE: u32 = (0x1 << 30);
/*
const PCIE_ATU_BUS(x)                 (((x) & 0xff) << 24);
const PCIE_ATU_DEV(x)                 (((x) & 0x1f) << 19);
const PCIE_ATU_FUNC(x)                (((x) & 0x7) << 16);

const PCIE_GET_ATU_OUTB_UNR_REG_OFFSET(region)        ((region) << 9)
*/ 
 /* Parameters for the waiting for iATU enabled routine */
const LINK_WAIT_MAX_IATU_RETRIES: u32 = 5;
const LINK_WAIT_IATU            : u32 = 10000;


// Please see the Linux header include/uapi/linux/pci.h for more details.
fn pci_bus(d: u32) -> u32 {
    (d >> 16) & 0xff
}
fn pci_dev(d: u32) -> u32 {
    (d >> 11) & 0x1f
}    
fn pci_func(d: u32) -> u32 {
    (d >> 8) & 0x7
}
fn pci_devfn(d: u32, f: u32) -> u32 {
    d << 11 | f << 8
}
fn pci_mast_bus(bdf: u32) -> u32 {
    bdf & 0xffff
}
fn pci_add_bus(bus: u32, devfn: u32) -> u32 {
    (bus << 16) | devfn
}

fn pci_bdf(b: u32, d: u32, f: u32) -> u32 {
    b << 16 | pci_devfn(d, f)
}
// Convert from Linux bdf format
fn pci_to_bdf(val: u32) -> u32 {
    val << 8
}

fn pcie_dw_addr_valid(d: u32, first_busno: u32) -> bool {
    if (pci_bus(d) == first_busno) || (pci_bus(d) == first_busno + 1) {
        if pci_dev(d) > 0 {
            return false;
        }
    }
    true
}

fn upper_32_bits(n: u64) -> u32 {
    ((n >> 16) >> 16) as u32
}

fn lower_32_bits(n: u64) -> u32 {
    (n & 0xffffffff) as u32
}

fn udelay(n: u32) {
    // tmp func
    let mut i = 0;
    loop {
        i += 1;
        if i > n { break; }
    }
}

pub enum PCI_SIZE {
    PCI8,
    PCI16,
    PCI32,
}

fn pci_get_ff(size: PCI_SIZE) -> u32 {
    match size {
        PCI8  => 0xff,
        PCI16 => 0xffff,
        PCI32 => 0xffffffff,
    }
}

fn pci_conv_32_to_size(value: u64, offset: u32, size: PCI_SIZE) -> u64 {
    match size {
        PCI8  => (value >> ((offset & 3) * 8)) & 0xff,
        PCI16 => (value >> ((offset & 2) * 8)) & 0xffff,
        PCI32 => value,
    }
}

fn pci_conv_size_to_32(old: u64, value: u64, offset: u32, size: PCI_SIZE) -> u64 {
    let mut off_mask = 0;
    let mut val_mask = 0;
    match size {
        PCI8 => {
            off_mask = 3;
            val_mask = 0xff;
        },
        PCI16 => {
            off_mask = 2;
            val_mask = 0xffff;
        },
        PCI32 => {
            return value;
        },
    }
    let shift = (offset & off_mask) * 8;
    let ldata = (value & val_mask) << shift;
    let mask = val_mask << shift;

    (old & !mask) | ldata
}

fn dw_pcie_writel_ob_unroll(index: u32, reg: u32, val: u32) {
    // PCIE_GET_ATU_OUTB_UNR_REG_OFFSET
    let offset = index << 9;
    writev((ATU_BASE + (offset + reg) as u64) as *mut u32, val);
}

fn dw_pcie_readl_ob_unroll(index: u32, reg: u32) -> u32 {
    let offset = index << 9;
    readv((ATU_BASE + (offset + reg) as u64) as *const u32)
}

// pcie_dw_prog_outbound_atu_unroll() - Configure ATU for outbound accesses
// @pcie: Pointer to the PCI controller state
// @index: ATU region index
// @type: ATU accsess type
// @cpu_addr: the physical address for the translation entry
// @pci_addr: the pcie bus address for the translation entry
// @size: the size of the translation entry
// Return: 0 is successful and -1 is failure
fn pcie_dw_prog_outbound_atu_unroll(index: u32, atu_type: u32, cpu_addr: u64, pci_addr: u64, size: u64) -> Result<i32, &'static str> {
    dw_pcie_writel_ob_unroll(index, PCIE_ATU_UNR_LOWER_BASE, lower_32_bits(cpu_addr));
    dw_pcie_writel_ob_unroll(index, PCIE_ATU_UNR_UPPER_BASE, upper_32_bits(cpu_addr));
    dw_pcie_writel_ob_unroll(index, PCIE_ATU_UNR_LIMIT, lower_32_bits(cpu_addr + size - 1));

    dw_pcie_writel_ob_unroll(index, PCIE_ATU_UNR_LOWER_TARGET, lower_32_bits(pci_addr));
    dw_pcie_writel_ob_unroll(index, PCIE_ATU_UNR_UPPER_TARGET, upper_32_bits(pci_addr));

    dw_pcie_writel_ob_unroll(index, PCIE_ATU_UNR_REGION_CTRL1, atu_type);
    dw_pcie_writel_ob_unroll(index, PCIE_ATU_UNR_REGION_CTRL2, PCIE_ATU_ENABLE);

    // Make sure ATU enable takes effect before any subsequent config and I/O accesses.

    for _ in 0..LINK_WAIT_MAX_IATU_RETRIES {
        let val = dw_pcie_readl_ob_unroll(index, PCIE_ATU_UNR_REGION_CTRL2);
        debug!("dw_pcie_readl_ob_unroll PCIE_ATU_UNR_REGION_CTRL2: {:#x}", val);
        if (val & PCIE_ATU_ENABLE) != 0 {
            return Ok(0);
        }

        udelay(LINK_WAIT_IATU);
    }

    error!("outbound iATU is not being enabled");

    return Err("outbound iATU is not being enabled"); 
}

pub fn set_cfg_address(bdf: u32, offset: u32) -> Option<u64> {
    let bus = pci_bus(bdf) - FIRST_BUSNO;
    let mut va_address: u64 = 0;
    if bus == 0 {
        va_address = DBI_BASE;
    } else {
        let atu_type = if bus == 1 {
        // For local bus whose primary bus number is root bridge,
        // change TLP Type field to 4.
            PCIE_ATU_TYPE_CFG0
        } else {
            // Otherwise, change TLP Type field to 5.
            PCIE_ATU_TYPE_CFG1
        };

        // PCI_ADD_BUS, PCI_MASK_BUS
        let bdf = (bus << 16) | (bdf & 0xffff);
        let ret = pcie_dw_prog_outbound_atu_unroll(PCIE_ATU_REGION_INDEX1, atu_type, CFG_BASE, (bdf as u64) << 8, CFG_SIZE);

        match ret {
            Ok(v) => {
                va_address = CFG_BASE;
            },
            Err(e) => return None,
        }
    }

    va_address += (offset as u64) & (!0x3);
    Some(va_address)
}

pub fn pcie_dw_read_config(bdf: u32, offset: u32, valuep: &mut u64, size: PCI_SIZE) -> Result<i32, &str> {
    info!("PCIE CFG  read: bdf={:2}:{:2}:{:2}, offset={:#x}", pci_bus(bdf), pci_dev(bdf), pci_func(bdf), offset);
    if !pcie_dw_addr_valid(bdf, FIRST_BUSNO) {
        debug!("bdf: {:#x} to read - out of range", bdf);
        *valuep = pci_get_ff(size) as u64;
        return Ok(0);
    }

    if let Some(va_address) = set_cfg_address(bdf, offset) {
        let value = readv(va_address as *const u64);

        *valuep = pci_conv_32_to_size(value, offset, size);
        debug!("Read @ {:#x}, value: {:#x}, converted to value: {:#x}", va_address, value, valuep);
    } else {
        error!("Set config address failed !");
    }

    pcie_dw_prog_outbound_atu_unroll(PCIE_ATU_REGION_INDEX1, PCIE_ATU_TYPE_IO, IO_BASE, IO_BUS_ADDR, IO_SIZE)
}

pub fn pcie_dw_write_config(bdf: u32, offset: u32, value: u64, size: PCI_SIZE) -> Result<i32, &'static str> {
    info!("PCIE CFG write: bdf={:2}:{:2}:{:2}, offset={:#x}, value={:#x}", pci_bus(bdf), pci_dev(bdf), pci_func(bdf), offset, value);
    if !pcie_dw_addr_valid(bdf, FIRST_BUSNO) {
        debug!("bdf: {:#x} to write - out of range", bdf);
        return Ok(0);
    }

    if let Some(va_address) = set_cfg_address(bdf, offset) {
        let old = readv(va_address as *const u64);
        let value = pci_conv_size_to_32(old, value, offset, size);
        
        writev(va_address as *mut u64, value);
        debug!("Write @ {:#x}, old: {:#x}, converted to value: {:#x}", va_address, old, value);
    } else {
        error!("Set config address failed !");
    }

    pcie_dw_prog_outbound_atu_unroll(PCIE_ATU_REGION_INDEX1, PCIE_ATU_TYPE_IO, IO_BASE, IO_BUS_ADDR, IO_SIZE)
}
            
fn readv<T>(src: *const T) -> T {
    debug!("read_volatile: {:#x}", phys_to_virt(src as usize));
    unsafe { core::ptr::read_volatile(phys_to_virt(src as usize) as *const T) }
}

fn writev<T>(dst: *mut T, value: T) {
    debug!("write_volatile: {:#x}", phys_to_virt(dst as usize));
    unsafe {
        core::ptr::write_volatile(phys_to_virt(dst as usize) as *mut T, value);
    }
}
