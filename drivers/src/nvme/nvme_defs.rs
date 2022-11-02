
#[repr(C)]
#[derive(Debug, Clone, Copy)]
//1+1+2+4+4*2+8+8+8+4*6 64B
pub struct NvmeCommonCommand {
	pub opcode: u8,
	pub flags: u8,
    pub command_id: u16,
    pub nsid: u32,
    pub cdw2: [u32;2],
    pub metadata: u64,
    pub prp1: u64,
    pub prp2: u64,
    pub cdw10: u32,
    pub cdw11: u32,
    pub cdw12: u32,
    pub cdw13: u32,
    pub cdw14: u32,
    pub cdw15: u32,
}

impl NvmeCommonCommand {
    pub fn new() -> Self{
        Self{
            opcode: 0,
            flags: 0,
            command_id: 0,
            nsid: 0,
            cdw2: [0;2],
            metadata: 0,
            prp1: 0,
            prp2: 0,
            cdw10: 0,
            cdw11: 0,
            cdw12: 0,
            cdw13: 0,
            cdw14: 0,
            cdw15: 0,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum NvmeCommand {
    NvmeRWCommand,
    NvmeCreateSq,
	NvmeCreateCq,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
// 4+4+2+2+2+2 16B
pub struct NvmeCompletion{
    pub result: u64,
    // pub rsvd: u32,
    pub sq_head: u16,
    pub sq_id: u16,
    pub command_id: u16,
    pub status: u16,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct NvmeCreateCq{
    pub opcode: u8,
    pub flags: u8,
    pub command_id: u16,
    pub rsvd1: [u32;5],
    pub prp1: u64,
    pub rsvd8: u64,
    pub cqid: u16,
    pub qsize: u16,
    pub cq_flags: u16,
    pub irq_vector: u16,
    pub rsvd12: [u32;4],
}

impl NvmeCreateCq{
    pub fn new_create_cq_command() -> Self{
        let flags = NVME_QUEUE_PHYS_CONTIG | NVME_CQ_IRQ_ENABLED;
        NvmeCreateCq{
            opcode: 0x05,
            flags: 0,
            command_id: 2,
            rsvd1: [0 as u32; 5],
            prp1: 0,
            rsvd8: 0,
            cqid: 1,
            qsize: 31,
            cq_flags: 0x3ff,
            irq_vector: 0,
            rsvd12: [0 as u32; 4],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct NvmeCreateSq{
    pub opcode: u8,
    pub flags: u8,
    pub command_id: u16,
    pub rsvd1: [u32;5],
    pub prp1: u64,
    pub rsvd8: u64,
    pub sqid: u16,
    pub qsize: u16,
    pub sq_flags: u16,
    pub cqid: u16,
    pub rsvd12: [u32;4],
}

impl NvmeCreateSq{
    pub fn new_create_sq_command() -> Self{
        let flags = NVME_QUEUE_PHYS_CONTIG | NVME_SQ_PRIO_MEDIUM;
        NvmeCreateSq{
            opcode: 0x01,
            flags: 0,
            command_id: 3,
            rsvd1: [0 as u32; 5],
            prp1: 0,
            rsvd8: 0,
            sqid: 1,
            qsize: 32,
            sq_flags: 0x3ff,
            cqid: 0,
            rsvd12: [0 as u32; 4],
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct NvmeRWCommand {
    pub opcode: u8,
    pub flags: u8,
    pub command_id: u16,
    pub nsid: u32,
    pub rsvd2: u64,
    pub metadata: u64,
    pub prp1: u64,
    pub prp2: u64,
    pub slba: u64,
    pub length: u16,
    pub control: u16,
    pub dsmgmt: u32,
    pub reftag: u32,
    pub apptag: u16,
    pub appmask: u16,
}

impl NvmeRWCommand{
    pub fn new_write_command() -> Self{
        Self{
            opcode: 0x01,
            flags: 0,
            command_id: 0,
            nsid: 0,
            rsvd2: 0,
            metadata: 0,
            prp1: 0,
            prp2: 0,
            slba: 0,
            length: 0,
            control: 0,
            dsmgmt: 0,
            reftag: 0,
            apptag: 0,
            appmask: 0,
        }
    }
    pub fn new_read_command() -> Self{
        Self{
            opcode: 0x02,
            flags: 0,
            command_id: 0,
            nsid: 0,
            rsvd2: 0,
            metadata: 0,
            prp1: 0,
            prp2: 0,
            slba: 0,
            length: 0,
            control: 0,
            dsmgmt: 0,
            reftag: 0,
            apptag: 0,
            appmask: 0,
        }
    }
}

pub struct NvmeFeatures{
    pub opcode: u8,
    pub flags: u8,
    pub command_id: u16,
    pub nsid: u32,
    pub rsvd2: [u64;2],
    pub prp1: u64,
    pub prp2: u64,
    pub fid: u32,
    pub dword11: u32,
    pub rsvd12: [u32;4],
}

impl NvmeFeatures{
    pub fn new_admin_set_features() -> Self{
        Self{
            opcode: 0x09,
            flags: 0,
            command_id: 0,
            nsid: 0,
            rsvd2: [0;2],
            prp1: 0,
            prp2: 0,
            fid: 0,
            dword11: 0,
            rsvd12: [0;4],
        }
    }

    pub fn nvme_set_queue_count(count: u32) -> Self{

        Self{
            opcode: 0x09,
            flags: 0,
            command_id: 3,
            nsid: 0,
            rsvd2: [0;2],
            prp1: 0,
            prp2: 0,
            fid: 0x07,
            dword11: count,
            rsvd12: [0;4],
        }
    }
}
// 16 bytes
pub struct NvmeCompleteQueue{
    pub byte8_1: u64,
    pub byte8_2: u64,
}
 
// 1+1+2+4+8+8+8+8+1+1+2+1+1+1+1+4+4+4+4 = 64 bytes
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NvmeIdentify{
    pub opcode: u8,
    pub flags: u8,
    pub command_id: u16,
    pub nsid: u32,
    pub rsvd2: [u64;2],
    pub prp1:u64,
    pub prp2:u64,
    pub cns: u8,
    pub rsvd3: u8,
    pub ctrlid: u16,
    pub rsvd11: [u8;3],
    pub csi: u8,
    pub rsvd12: [u32;4],
}

impl NvmeIdentify{
    pub fn new() -> Self{
        Self{
            opcode: 0x06,
            flags: 0,
            command_id: 0x1,
            nsid: 1,
            rsvd2: [0;2],
            prp1: 0,
            prp2: 0,
            cns: 1,
            rsvd3: 0,
            ctrlid: 0,
            rsvd11: [0;3],
            csi: 0,
            rsvd12: [0;4],
        }
    }

    pub fn nvme_init_non_mdts_limits() -> Self{
        Self{
            opcode: 0x06,
            flags: 0,
            command_id: 2,
            nsid: 0,
            rsvd2: [0;2],
            prp1: 0,
            prp2: 0,
            cns: 0,
            rsvd3: 0,
            ctrlid: 0,
            rsvd11: [0;3],
            csi: 0,
            rsvd12: [0;4],
        }
    }
}

pub struct NvmeFeatAutoPst{
    pub entry: [u64;32],
}

impl NvmeFeatAutoPst {

    pub fn new() -> Self{
        Self{
            entry: [0;32],
        }
    }
}

//NvmeRegister
pub const NVME_REG_CAP:usize	= 0x0000;	/* Controller Capabilities */
pub const NVME_REG_VS:usize	    = 0x0008;	/* Version */
pub const NVME_REG_INTMS:usize	= 0x000c;	/* Interrupt Mask Set */
pub const NVME_REG_INTMC:usize	= 0x0010;	/* Interrupt Mask Clear */
pub const NVME_REG_CC:usize	    = 0x0014;	/* Controller Configuration */
pub const NVME_REG_CSTS:usize	= 0x001c;	/* Controller Status */
pub const NVME_REG_NSSR:usize	= 0x0020;	/* NVM Subsystem Reset */
pub const NVME_REG_AQA:usize    = 0x0024;	/* Admin Queue Attributes */
pub const NVME_REG_ASQ:usize    = 0x0028;	/* Admin SQ Base Address */
pub const NVME_REG_ACQ:usize    = 0x0030;	/* Admin CQ Base Address */
pub const NVME_REG_CMBLOC:usize	= 0x0038;	/* Controller Memory Buffer Location */
pub const NVME_REG_CMBSZ:usize	= 0x003c;	/* Controller Memory Buffer Size */
pub const NVME_REG_BPINFO:usize	= 0x0040;	/* Boot Partition Information */
pub const NVME_REG_BPRSEL:usize	= 0x0044;	/* Boot Partition Read Select */
pub const NVME_REG_BPMBL:usize	= 0x0048;	/* Boot Partition Memory Buffer
         				 * Location
         				 */
pub const NVME_REG_CMBMSC:usize = 0x0050;	/* Controller Memory Buffer Memory
         				 * Space Control
         				 */
pub const NVME_REG_CRTO:usize	= 0x0068;	/* Controller Ready Timeouts */
pub const NVME_REG_PMRCAP:usize	= 0x0e00;	/* Persistent Memory Capabilities */
pub const NVME_REG_PMRCTL:usize	= 0x0e04;	/* Persistent Memory Region Control */
pub const NVME_REG_PMRSTS:usize	= 0x0e08;	/* Persistent Memory Region Status */
pub const NVME_REG_PMREBS:usize	= 0x0e0c;	/* Persistent Memory Region Elasticity
         				 * Buffer Size
         				 */
pub const NVME_REG_PMRSWTP:usize = 0x0e10;	/* Persistent Memory Region Sustained
         				 * Write Throughput
         				 */
pub const NVME_REG_DBS:usize	= 0x1000;	/* SQ 0 Tail Doorbell */

// NVME CONST
pub const NVME_CC_ENABLE: u32 = 1 << 0;
pub const NVME_CC_CSS_NVM: u32 = 0 << 4;
pub const NVME_CC_MPS_SHIFT: u32 = 7;
pub const NVME_CC_ARB_RR: u32 = 0 << 11;
pub const NVME_CC_ARB_WRRU: u32 = 1 << 11;
pub const NVME_CC_ARB_VS: u32 = 7 << 11;
pub const NVME_CC_SHN_NONE: u32 = 0 << 14;
pub const NVME_CC_SHN_NORMAL: u32 = 1 << 14;
pub const NVME_CC_SHN_ABRUPT: u32 = 2 << 14;
pub const NVME_CC_IOSQES: u32 = 6 << 16;
pub const NVME_CC_IOCQES: u32 = 4 << 20;
pub const NVME_CSTS_RDY: u32 = 1 << 0;
pub const NVME_CSTS_CFS: u32 = 1 << 1;
pub const NVME_CSTS_SHST_NORMAL: u32 = 0 << 2;
pub const NVME_CSTS_SHST_OCCUR: u32 = 1 << 2;
pub const NVME_CSTS_SHST_CMPLT: u32 = 2 << 2;

pub const NVME_QUEUE_PHYS_CONTIG: u16 = 1 << 0;
pub const NVME_CQ_IRQ_ENABLED: u16 = 1 << 1;
pub const NVME_SQ_PRIO_URGENT: u16 = 0 << 1;
pub const NVME_SQ_PRIO_HIGH: u16 = 1 << 1;
pub const NVME_SQ_PRIO_MEDIUM: u16 = 2 << 1;
pub const NVME_SQ_PRIO_LOW: u16 = 3 << 1;

pub const NVME_FEAT_ARBITRATION: u32 = 0x01;
pub const NVME_FEAT_POWER_MGMT: u32 = 0x02;
pub const NVME_FEAT_LBA_RANGE: u32 = 0x03;
pub const NVME_FEAT_TEMP_THRESH: u32 = 0x04;
pub const NVME_FEAT_ERR_RECOVERY: u32 = 0x05;
pub const NVME_FEAT_VOLATILE_WC: u32 = 0x06;
pub const NVME_FEAT_NUM_QUEUES: u32 = 0x07;
pub const NVME_FEAT_IRQ_COALESCE: u32 = 0x08;
pub const NVME_FEAT_IRQ_CONFIG: u32 = 0x09;
pub const NVME_FEAT_WRITE_ATOMIC: u32 = 0x0a;
pub const NVME_FEAT_ASYNC_EVENT: u32 = 0x0b;
pub const NVME_FEAT_SW_PROGRESS: u32 = 0x0c;