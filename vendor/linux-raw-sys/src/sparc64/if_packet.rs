/* automatically generated by rust-bindgen 0.66.1 */

pub type __s8 = crate::ctypes::c_schar;
pub type __u8 = crate::ctypes::c_uchar;
pub type __s16 = crate::ctypes::c_short;
pub type __u16 = crate::ctypes::c_ushort;
pub type __s32 = crate::ctypes::c_int;
pub type __u32 = crate::ctypes::c_uint;
pub type __s64 = crate::ctypes::c_longlong;
pub type __u64 = crate::ctypes::c_ulonglong;
pub type __kernel_key_t = crate::ctypes::c_int;
pub type __kernel_mqd_t = crate::ctypes::c_int;
pub type __kernel_old_uid_t = crate::ctypes::c_ushort;
pub type __kernel_old_gid_t = crate::ctypes::c_ushort;
pub type __kernel_suseconds_t = crate::ctypes::c_int;
pub type __kernel_long_t = crate::ctypes::c_long;
pub type __kernel_ulong_t = crate::ctypes::c_ulong;
pub type __kernel_ino_t = __kernel_ulong_t;
pub type __kernel_mode_t = crate::ctypes::c_uint;
pub type __kernel_pid_t = crate::ctypes::c_int;
pub type __kernel_ipc_pid_t = crate::ctypes::c_int;
pub type __kernel_uid_t = crate::ctypes::c_uint;
pub type __kernel_gid_t = crate::ctypes::c_uint;
pub type __kernel_daddr_t = crate::ctypes::c_int;
pub type __kernel_uid32_t = crate::ctypes::c_uint;
pub type __kernel_gid32_t = crate::ctypes::c_uint;
pub type __kernel_old_dev_t = crate::ctypes::c_uint;
pub type __kernel_size_t = __kernel_ulong_t;
pub type __kernel_ssize_t = __kernel_long_t;
pub type __kernel_ptrdiff_t = __kernel_long_t;
pub type __kernel_off_t = __kernel_long_t;
pub type __kernel_loff_t = crate::ctypes::c_longlong;
pub type __kernel_old_time_t = __kernel_long_t;
pub type __kernel_time_t = __kernel_long_t;
pub type __kernel_time64_t = crate::ctypes::c_longlong;
pub type __kernel_clock_t = __kernel_long_t;
pub type __kernel_timer_t = crate::ctypes::c_int;
pub type __kernel_clockid_t = crate::ctypes::c_int;
pub type __kernel_caddr_t = *mut crate::ctypes::c_char;
pub type __kernel_uid16_t = crate::ctypes::c_ushort;
pub type __kernel_gid16_t = crate::ctypes::c_ushort;
pub type __le16 = __u16;
pub type __be16 = __u16;
pub type __le32 = __u32;
pub type __be32 = __u32;
pub type __le64 = __u64;
pub type __be64 = __u64;
pub type __sum16 = __u16;
pub type __wsum = __u32;
pub type __poll_t = crate::ctypes::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __kernel_old_timeval {
pub tv_sec: __kernel_long_t,
pub tv_usec: __kernel_suseconds_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sockaddr_pkt {
pub spkt_family: crate::ctypes::c_ushort,
pub spkt_device: [crate::ctypes::c_uchar; 14usize],
pub spkt_protocol: __be16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sockaddr_ll {
pub sll_family: crate::ctypes::c_ushort,
pub sll_protocol: __be16,
pub sll_ifindex: crate::ctypes::c_int,
pub sll_hatype: crate::ctypes::c_ushort,
pub sll_pkttype: crate::ctypes::c_uchar,
pub sll_halen: crate::ctypes::c_uchar,
pub sll_addr: [crate::ctypes::c_uchar; 8usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tpacket_stats {
pub tp_packets: crate::ctypes::c_uint,
pub tp_drops: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tpacket_stats_v3 {
pub tp_packets: crate::ctypes::c_uint,
pub tp_drops: crate::ctypes::c_uint,
pub tp_freeze_q_cnt: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tpacket_rollover_stats {
pub tp_all: __u64,
pub tp_huge: __u64,
pub tp_failed: __u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tpacket_auxdata {
pub tp_status: __u32,
pub tp_len: __u32,
pub tp_snaplen: __u32,
pub tp_mac: __u16,
pub tp_net: __u16,
pub tp_vlan_tci: __u16,
pub tp_vlan_tpid: __u16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tpacket_hdr {
pub tp_status: crate::ctypes::c_ulong,
pub tp_len: crate::ctypes::c_uint,
pub tp_snaplen: crate::ctypes::c_uint,
pub tp_mac: crate::ctypes::c_ushort,
pub tp_net: crate::ctypes::c_ushort,
pub tp_sec: crate::ctypes::c_uint,
pub tp_usec: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tpacket2_hdr {
pub tp_status: __u32,
pub tp_len: __u32,
pub tp_snaplen: __u32,
pub tp_mac: __u16,
pub tp_net: __u16,
pub tp_sec: __u32,
pub tp_nsec: __u32,
pub tp_vlan_tci: __u16,
pub tp_vlan_tpid: __u16,
pub tp_padding: [__u8; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tpacket_hdr_variant1 {
pub tp_rxhash: __u32,
pub tp_vlan_tci: __u32,
pub tp_vlan_tpid: __u16,
pub tp_padding: __u16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpacket3_hdr {
pub tp_next_offset: __u32,
pub tp_sec: __u32,
pub tp_nsec: __u32,
pub tp_snaplen: __u32,
pub tp_len: __u32,
pub tp_status: __u32,
pub tp_mac: __u16,
pub tp_net: __u16,
pub __bindgen_anon_1: tpacket3_hdr__bindgen_ty_1,
pub tp_padding: [__u8; 8usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpacket_bd_ts {
pub ts_sec: crate::ctypes::c_uint,
pub __bindgen_anon_1: tpacket_bd_ts__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpacket_hdr_v1 {
pub block_status: __u32,
pub num_pkts: __u32,
pub offset_to_first_pkt: __u32,
pub blk_len: __u32,
pub seq_num: __u64,
pub ts_first_pkt: tpacket_bd_ts,
pub ts_last_pkt: tpacket_bd_ts,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpacket_block_desc {
pub version: __u32,
pub offset_to_priv: __u32,
pub hdr: tpacket_bd_header_u,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tpacket_req {
pub tp_block_size: crate::ctypes::c_uint,
pub tp_block_nr: crate::ctypes::c_uint,
pub tp_frame_size: crate::ctypes::c_uint,
pub tp_frame_nr: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tpacket_req3 {
pub tp_block_size: crate::ctypes::c_uint,
pub tp_block_nr: crate::ctypes::c_uint,
pub tp_frame_size: crate::ctypes::c_uint,
pub tp_frame_nr: crate::ctypes::c_uint,
pub tp_retire_blk_tov: crate::ctypes::c_uint,
pub tp_sizeof_priv: crate::ctypes::c_uint,
pub tp_feature_req_word: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct packet_mreq {
pub mr_ifindex: crate::ctypes::c_int,
pub mr_type: crate::ctypes::c_ushort,
pub mr_alen: crate::ctypes::c_ushort,
pub mr_address: [crate::ctypes::c_uchar; 8usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fanout_args {
pub type_flags: __u16,
pub id: __u16,
pub max_num_members: __u32,
}
pub const __BIG_ENDIAN: u32 = 4321;
pub const ASI_NULL1: u32 = 0;
pub const ASI_NULL2: u32 = 1;
pub const ASI_CONTROL: u32 = 2;
pub const ASI_SEGMAP: u32 = 3;
pub const ASI_PTE: u32 = 4;
pub const ASI_HWFLUSHSEG: u32 = 5;
pub const ASI_HWFLUSHPAGE: u32 = 6;
pub const ASI_REGMAP: u32 = 6;
pub const ASI_HWFLUSHCONTEXT: u32 = 7;
pub const ASI_USERTXT: u32 = 8;
pub const ASI_KERNELTXT: u32 = 9;
pub const ASI_USERDATA: u32 = 10;
pub const ASI_KERNELDATA: u32 = 11;
pub const ASI_FLUSHSEG: u32 = 12;
pub const ASI_FLUSHPG: u32 = 13;
pub const ASI_FLUSHCTX: u32 = 14;
pub const ASI_M_RES00: u32 = 0;
pub const ASI_M_UNA01: u32 = 1;
pub const ASI_M_MXCC: u32 = 2;
pub const ASI_M_FLUSH_PROBE: u32 = 3;
pub const ASI_M_MMUREGS: u32 = 4;
pub const ASI_M_TLBDIAG: u32 = 5;
pub const ASI_M_DIAGS: u32 = 6;
pub const ASI_M_IODIAG: u32 = 7;
pub const ASI_M_USERTXT: u32 = 8;
pub const ASI_M_KERNELTXT: u32 = 9;
pub const ASI_M_USERDATA: u32 = 10;
pub const ASI_M_KERNELDATA: u32 = 11;
pub const ASI_M_TXTC_TAG: u32 = 12;
pub const ASI_M_TXTC_DATA: u32 = 13;
pub const ASI_M_DATAC_TAG: u32 = 14;
pub const ASI_M_DATAC_DATA: u32 = 15;
pub const ASI_M_FLUSH_PAGE: u32 = 16;
pub const ASI_M_FLUSH_SEG: u32 = 17;
pub const ASI_M_FLUSH_REGION: u32 = 18;
pub const ASI_M_FLUSH_CTX: u32 = 19;
pub const ASI_M_FLUSH_USER: u32 = 20;
pub const ASI_M_BCOPY: u32 = 23;
pub const ASI_M_IFLUSH_PAGE: u32 = 24;
pub const ASI_M_IFLUSH_SEG: u32 = 25;
pub const ASI_M_IFLUSH_REGION: u32 = 26;
pub const ASI_M_IFLUSH_CTX: u32 = 27;
pub const ASI_M_IFLUSH_USER: u32 = 28;
pub const ASI_M_BFILL: u32 = 31;
pub const ASI_M_BYPASS: u32 = 32;
pub const ASI_M_FBMEM: u32 = 41;
pub const ASI_M_VMEUS: u32 = 42;
pub const ASI_M_VMEPS: u32 = 43;
pub const ASI_M_VMEUT: u32 = 44;
pub const ASI_M_VMEPT: u32 = 45;
pub const ASI_M_SBUS: u32 = 46;
pub const ASI_M_CTL: u32 = 47;
pub const ASI_M_FLUSH_IWHOLE: u32 = 49;
pub const ASI_M_IC_FLCLEAR: u32 = 54;
pub const ASI_M_DC_FLCLEAR: u32 = 55;
pub const ASI_M_DCDR: u32 = 57;
pub const ASI_M_VIKING_TMP1: u32 = 64;
pub const ASI_M_ACTION: u32 = 76;
pub const ASI_LEON_NOCACHE: u32 = 1;
pub const ASI_LEON_DCACHE_MISS: u32 = 1;
pub const ASI_LEON_CACHEREGS: u32 = 2;
pub const ASI_LEON_IFLUSH: u32 = 16;
pub const ASI_LEON_DFLUSH: u32 = 17;
pub const ASI_LEON_MMUFLUSH: u32 = 24;
pub const ASI_LEON_MMUREGS: u32 = 25;
pub const ASI_LEON_BYPASS: u32 = 28;
pub const ASI_LEON_FLUSH_PAGE: u32 = 16;
pub const ASI_N: u32 = 4;
pub const ASI_NL: u32 = 12;
pub const ASI_AIUP: u32 = 16;
pub const ASI_AIUS: u32 = 17;
pub const ASI_AIUPL: u32 = 24;
pub const ASI_AIUSL: u32 = 25;
pub const ASI_P: u32 = 128;
pub const ASI_S: u32 = 129;
pub const ASI_PNF: u32 = 130;
pub const ASI_SNF: u32 = 131;
pub const ASI_PL: u32 = 136;
pub const ASI_SL: u32 = 137;
pub const ASI_PNFL: u32 = 138;
pub const ASI_SNFL: u32 = 139;
pub const ASI_MCD_PRIV_PRIMARY: u32 = 2;
pub const ASI_MCD_REAL: u32 = 5;
pub const ASI_PHYS_USE_EC: u32 = 20;
pub const ASI_PHYS_BYPASS_EC_E: u32 = 21;
pub const ASI_BLK_AIUP_4V: u32 = 22;
pub const ASI_BLK_AIUS_4V: u32 = 23;
pub const ASI_PHYS_USE_EC_L: u32 = 28;
pub const ASI_PHYS_BYPASS_EC_E_L: u32 = 29;
pub const ASI_BLK_AIUP_L_4V: u32 = 30;
pub const ASI_BLK_AIUS_L_4V: u32 = 31;
pub const ASI_SCRATCHPAD: u32 = 32;
pub const ASI_MMU: u32 = 33;
pub const ASI_BLK_INIT_QUAD_LDD_AIUS: u32 = 35;
pub const ASI_NUCLEUS_QUAD_LDD: u32 = 36;
pub const ASI_QUEUE: u32 = 37;
pub const ASI_QUAD_LDD_PHYS_4V: u32 = 38;
pub const ASI_NUCLEUS_QUAD_LDD_L: u32 = 44;
pub const ASI_QUAD_LDD_PHYS_L_4V: u32 = 46;
pub const ASI_PCACHE_DATA_STATUS: u32 = 48;
pub const ASI_PCACHE_DATA: u32 = 49;
pub const ASI_PCACHE_TAG: u32 = 50;
pub const ASI_PCACHE_SNOOP_TAG: u32 = 51;
pub const ASI_QUAD_LDD_PHYS: u32 = 52;
pub const ASI_WCACHE_VALID_BITS: u32 = 56;
pub const ASI_WCACHE_DATA: u32 = 57;
pub const ASI_WCACHE_TAG: u32 = 58;
pub const ASI_WCACHE_SNOOP_TAG: u32 = 59;
pub const ASI_QUAD_LDD_PHYS_L: u32 = 60;
pub const ASI_SRAM_FAST_INIT: u32 = 64;
pub const ASI_CORE_AVAILABLE: u32 = 65;
pub const ASI_CORE_ENABLE_STAT: u32 = 65;
pub const ASI_CORE_ENABLE: u32 = 65;
pub const ASI_XIR_STEERING: u32 = 65;
pub const ASI_CORE_RUNNING_RW: u32 = 65;
pub const ASI_CORE_RUNNING_W1S: u32 = 65;
pub const ASI_CORE_RUNNING_W1C: u32 = 65;
pub const ASI_CORE_RUNNING_STAT: u32 = 65;
pub const ASI_CMT_ERROR_STEERING: u32 = 65;
pub const ASI_DCACHE_INVALIDATE: u32 = 66;
pub const ASI_DCACHE_UTAG: u32 = 67;
pub const ASI_DCACHE_SNOOP_TAG: u32 = 68;
pub const ASI_LSU_CONTROL: u32 = 69;
pub const ASI_DCU_CONTROL_REG: u32 = 69;
pub const ASI_DCACHE_DATA: u32 = 70;
pub const ASI_DCACHE_TAG: u32 = 71;
pub const ASI_INTR_DISPATCH_STAT: u32 = 72;
pub const ASI_INTR_RECEIVE: u32 = 73;
pub const ASI_UPA_CONFIG: u32 = 74;
pub const ASI_JBUS_CONFIG: u32 = 74;
pub const ASI_SAFARI_CONFIG: u32 = 74;
pub const ASI_SAFARI_ADDRESS: u32 = 74;
pub const ASI_ESTATE_ERROR_EN: u32 = 75;
pub const ASI_AFSR: u32 = 76;
pub const ASI_AFAR: u32 = 77;
pub const ASI_EC_TAG_DATA: u32 = 78;
pub const ASI_IMMU: u32 = 80;
pub const ASI_IMMU_TSB_8KB_PTR: u32 = 81;
pub const ASI_IMMU_TSB_64KB_PTR: u32 = 82;
pub const ASI_ITLB_DATA_IN: u32 = 84;
pub const ASI_ITLB_DATA_ACCESS: u32 = 85;
pub const ASI_ITLB_TAG_READ: u32 = 86;
pub const ASI_IMMU_DEMAP: u32 = 87;
pub const ASI_DMMU: u32 = 88;
pub const ASI_DMMU_TSB_8KB_PTR: u32 = 89;
pub const ASI_DMMU_TSB_64KB_PTR: u32 = 90;
pub const ASI_DMMU_TSB_DIRECT_PTR: u32 = 91;
pub const ASI_DTLB_DATA_IN: u32 = 92;
pub const ASI_DTLB_DATA_ACCESS: u32 = 93;
pub const ASI_DTLB_TAG_READ: u32 = 94;
pub const ASI_DMMU_DEMAP: u32 = 95;
pub const ASI_IIU_INST_TRAP: u32 = 96;
pub const ASI_INTR_ID: u32 = 99;
pub const ASI_CORE_ID: u32 = 99;
pub const ASI_CESR_ID: u32 = 99;
pub const ASI_IC_INSTR: u32 = 102;
pub const ASI_IC_TAG: u32 = 103;
pub const ASI_IC_STAG: u32 = 104;
pub const ASI_IC_PRE_DECODE: u32 = 110;
pub const ASI_IC_NEXT_FIELD: u32 = 111;
pub const ASI_BRPRED_ARRAY: u32 = 111;
pub const ASI_BLK_AIUP: u32 = 112;
pub const ASI_BLK_AIUS: u32 = 113;
pub const ASI_MCU_CTRL_REG: u32 = 114;
pub const ASI_EC_DATA: u32 = 116;
pub const ASI_EC_CTRL: u32 = 117;
pub const ASI_EC_W: u32 = 118;
pub const ASI_UDB_ERROR_W: u32 = 119;
pub const ASI_UDB_CONTROL_W: u32 = 119;
pub const ASI_INTR_W: u32 = 119;
pub const ASI_INTR_DATAN_W: u32 = 119;
pub const ASI_INTR_DISPATCH_W: u32 = 119;
pub const ASI_BLK_AIUPL: u32 = 120;
pub const ASI_BLK_AIUSL: u32 = 121;
pub const ASI_EC_R: u32 = 126;
pub const ASI_UDBH_ERROR_R: u32 = 127;
pub const ASI_UDBL_ERROR_R: u32 = 127;
pub const ASI_UDBH_CONTROL_R: u32 = 127;
pub const ASI_UDBL_CONTROL_R: u32 = 127;
pub const ASI_INTR_R: u32 = 127;
pub const ASI_INTR_DATAN_R: u32 = 127;
pub const ASI_MCD_PRIMARY: u32 = 144;
pub const ASI_MCD_ST_BLKINIT_PRIMARY: u32 = 146;
pub const ASI_PIC: u32 = 176;
pub const ASI_PST8_P: u32 = 192;
pub const ASI_PST8_S: u32 = 193;
pub const ASI_PST16_P: u32 = 194;
pub const ASI_PST16_S: u32 = 195;
pub const ASI_PST32_P: u32 = 196;
pub const ASI_PST32_S: u32 = 197;
pub const ASI_PST8_PL: u32 = 200;
pub const ASI_PST8_SL: u32 = 201;
pub const ASI_PST16_PL: u32 = 202;
pub const ASI_PST16_SL: u32 = 203;
pub const ASI_PST32_PL: u32 = 204;
pub const ASI_PST32_SL: u32 = 205;
pub const ASI_FL8_P: u32 = 208;
pub const ASI_FL8_S: u32 = 209;
pub const ASI_FL16_P: u32 = 210;
pub const ASI_FL16_S: u32 = 211;
pub const ASI_FL8_PL: u32 = 216;
pub const ASI_FL8_SL: u32 = 217;
pub const ASI_FL16_PL: u32 = 218;
pub const ASI_FL16_SL: u32 = 219;
pub const ASI_BLK_COMMIT_P: u32 = 224;
pub const ASI_BLK_COMMIT_S: u32 = 225;
pub const ASI_BLK_INIT_QUAD_LDD_P: u32 = 226;
pub const ASI_BLK_INIT_QUAD_LDD_S: u32 = 227;
pub const ASI_BLK_P: u32 = 240;
pub const ASI_BLK_S: u32 = 241;
pub const ASI_ST_BLKINIT_MRU_P: u32 = 242;
pub const ASI_ST_BLKINIT_MRU_S: u32 = 243;
pub const ASI_BLK_PL: u32 = 248;
pub const ASI_BLK_SL: u32 = 249;
pub const ASI_ST_BLKINIT_MRU_PL: u32 = 250;
pub const ASI_ST_BLKINIT_MRU_SL: u32 = 251;
pub const PACKET_HOST: u32 = 0;
pub const PACKET_BROADCAST: u32 = 1;
pub const PACKET_MULTICAST: u32 = 2;
pub const PACKET_OTHERHOST: u32 = 3;
pub const PACKET_OUTGOING: u32 = 4;
pub const PACKET_LOOPBACK: u32 = 5;
pub const PACKET_USER: u32 = 6;
pub const PACKET_KERNEL: u32 = 7;
pub const PACKET_FASTROUTE: u32 = 6;
pub const PACKET_ADD_MEMBERSHIP: u32 = 1;
pub const PACKET_DROP_MEMBERSHIP: u32 = 2;
pub const PACKET_RECV_OUTPUT: u32 = 3;
pub const PACKET_RX_RING: u32 = 5;
pub const PACKET_STATISTICS: u32 = 6;
pub const PACKET_COPY_THRESH: u32 = 7;
pub const PACKET_AUXDATA: u32 = 8;
pub const PACKET_ORIGDEV: u32 = 9;
pub const PACKET_VERSION: u32 = 10;
pub const PACKET_HDRLEN: u32 = 11;
pub const PACKET_RESERVE: u32 = 12;
pub const PACKET_TX_RING: u32 = 13;
pub const PACKET_LOSS: u32 = 14;
pub const PACKET_VNET_HDR: u32 = 15;
pub const PACKET_TX_TIMESTAMP: u32 = 16;
pub const PACKET_TIMESTAMP: u32 = 17;
pub const PACKET_FANOUT: u32 = 18;
pub const PACKET_TX_HAS_OFF: u32 = 19;
pub const PACKET_QDISC_BYPASS: u32 = 20;
pub const PACKET_ROLLOVER_STATS: u32 = 21;
pub const PACKET_FANOUT_DATA: u32 = 22;
pub const PACKET_IGNORE_OUTGOING: u32 = 23;
pub const PACKET_FANOUT_HASH: u32 = 0;
pub const PACKET_FANOUT_LB: u32 = 1;
pub const PACKET_FANOUT_CPU: u32 = 2;
pub const PACKET_FANOUT_ROLLOVER: u32 = 3;
pub const PACKET_FANOUT_RND: u32 = 4;
pub const PACKET_FANOUT_QM: u32 = 5;
pub const PACKET_FANOUT_CBPF: u32 = 6;
pub const PACKET_FANOUT_EBPF: u32 = 7;
pub const PACKET_FANOUT_FLAG_ROLLOVER: u32 = 4096;
pub const PACKET_FANOUT_FLAG_UNIQUEID: u32 = 8192;
pub const PACKET_FANOUT_FLAG_IGNORE_OUTGOING: u32 = 16384;
pub const PACKET_FANOUT_FLAG_DEFRAG: u32 = 32768;
pub const TP_STATUS_KERNEL: u32 = 0;
pub const TP_STATUS_USER: u32 = 1;
pub const TP_STATUS_COPY: u32 = 2;
pub const TP_STATUS_LOSING: u32 = 4;
pub const TP_STATUS_CSUMNOTREADY: u32 = 8;
pub const TP_STATUS_VLAN_VALID: u32 = 16;
pub const TP_STATUS_BLK_TMO: u32 = 32;
pub const TP_STATUS_VLAN_TPID_VALID: u32 = 64;
pub const TP_STATUS_CSUM_VALID: u32 = 128;
pub const TP_STATUS_GSO_TCP: u32 = 256;
pub const TP_STATUS_AVAILABLE: u32 = 0;
pub const TP_STATUS_SEND_REQUEST: u32 = 1;
pub const TP_STATUS_SENDING: u32 = 2;
pub const TP_STATUS_WRONG_FORMAT: u32 = 4;
pub const TP_STATUS_TS_SOFTWARE: u32 = 536870912;
pub const TP_STATUS_TS_SYS_HARDWARE: u32 = 1073741824;
pub const TP_STATUS_TS_RAW_HARDWARE: u32 = 2147483648;
pub const TP_FT_REQ_FILL_RXHASH: u32 = 1;
pub const TPACKET_ALIGNMENT: u32 = 16;
pub const PACKET_MR_MULTICAST: u32 = 0;
pub const PACKET_MR_PROMISC: u32 = 1;
pub const PACKET_MR_ALLMULTI: u32 = 2;
pub const PACKET_MR_UNICAST: u32 = 3;
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum tpacket_versions {
TPACKET_V1 = 0,
TPACKET_V2 = 1,
TPACKET_V3 = 2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union tpacket_stats_u {
pub stats1: tpacket_stats,
pub stats3: tpacket_stats_v3,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union tpacket3_hdr__bindgen_ty_1 {
pub hv1: tpacket_hdr_variant1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union tpacket_bd_ts__bindgen_ty_1 {
pub ts_usec: crate::ctypes::c_uint,
pub ts_nsec: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union tpacket_bd_header_u {
pub bh1: tpacket_hdr_v1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union tpacket_req_u {
pub req: tpacket_req,
pub req3: tpacket_req3,
}