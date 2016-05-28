#![allow(non_camel_case_types)]
#![allow(dead_code)]

extern crate libc;

use libc::size_t;
use std::os::raw::{c_void, c_int, c_uint, c_char};

pub mod placeholders {
    include!(concat!(env!("OUT_DIR"), "/placeholders.rs"));
}

// automatically generated by rust-bindgen

pub type csh = size_t;

/// Architecture type
pub type cs_arch = c_int;
/// ARM architecture (including Thumb, Thumb-2)
pub const CS_ARCH_ARM: cs_arch = 0;
/// ARM-64, also called AArch64
pub const CS_ARCH_ARM64: cs_arch = 1;
/// Mips architecture
pub const CS_ARCH_MIPS: cs_arch = 2;
/// X86 architecture (including x86 & x86-64)
pub const CS_ARCH_X86: cs_arch = 3;
/// PowerPC architecture
pub const CS_ARCH_PPC: cs_arch = 4;
/// Sparc architecture
pub const CS_ARCH_SPARC: cs_arch = 5;
/// SystemZ architecture
pub const CS_ARCH_SYSZ: cs_arch = 6;
/// XCore architecture
pub const CS_ARCH_XCORE: cs_arch = 7;
pub const CS_ARCH_MAX: cs_arch = 8;
/// All architectures - for cs_support()
pub const CS_ARCH_ALL: cs_arch = 0xFFFF;

/// Mode type (architecture variant, not all combination are possible)
pub type cs_mode = c_int;

/// Little-endian mode (default mode)
pub const CS_MODE_LITTLE_ENDIAN: cs_mode = 0;
/// 32-bit ARM
pub const CS_MODE_ARM: cs_mode = 0;
/// 16-bit mode X86
pub const CS_MODE_16: cs_mode = 1 << 1;
/// 32-bit mode X86
pub const CS_MODE_32: cs_mode = 1 << 2;
/// 64-bit mode X86
pub const CS_MODE_64: cs_mode = 1 << 3;
/// ARM's Thumb mode, including Thumb-2
pub const CS_MODE_THUMB: cs_mode = 1 << 4;
/// ARM's Cortex-M series
pub const CS_MODE_MCLASS: cs_mode = 1 << 5;
/// ARMv8 A32 encodings for ARM
pub const CS_MODE_V8: cs_mode = 1 << 6;
/// MicroMips mode (MIPS)
pub const CS_MODE_MICRO: cs_mode = 1 << 4;
/// Mips III ISA
pub const CS_MODE_MIPS3: cs_mode = 1 << 5;
/// Mips32r6 ISA
pub const CS_MODE_MIPS32R6: cs_mode = 1 << 6;
/// General Purpose Registers are 64-bit wide (MIPS)
pub const CS_MODE_MIPSGP64: cs_mode = 1 << 7;
/// SparcV9 mode (Sparc)
pub const CS_MODE_V9: cs_mode = 1 << 31;
/// big-endian mode
pub const CS_MODE_BIG_ENDIAN: cs_mode = 1 << 31;
/// Mips32 ISA (Mips)
pub const CS_MODE_MIPS32: cs_mode = CS_MODE_32;
/// Mips64 ISA (Mips)
pub const CS_MODE_MIPS64: cs_mode = CS_MODE_64;

pub type cs_malloc_t = Option<extern "C" fn(size: size_t) -> *mut c_void>;
pub type cs_calloc_t = Option<extern "C" fn(nmemb: size_t, size: size_t) -> *mut c_void>;
pub type cs_realloc_t = Option<unsafe extern "C" fn(ptr: *mut c_void, size: size_t) -> *mut c_void>;
pub type cs_free_t = Option<unsafe extern "C" fn(ptr: *mut c_void)>;

pub type cs_vsnprintf_t = Option<unsafe extern "C" fn()>;
// pub type cs_vsnprintf_t = Option<unsafe extern "C" fn(str: *mut c_char,
//                                                       size: size_t,
//                                                       format: *const c_char,
//                                                       ap: va_list)
//                                                       -> c_int>;

#[repr(C)]
pub struct cs_opt_mem {
    pub malloc: cs_malloc_t,
    pub calloc: cs_calloc_t,
    pub realloc: cs_realloc_t,
    pub free: cs_free_t,
    pub vsnprintf: cs_vsnprintf_t,
}
impl ::std::default::Default for cs_opt_mem {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum cs_opt_type {
    CS_OPT_SYNTAX = 1,
    CS_OPT_DETAIL = 2,
    CS_OPT_MODE = 3,
    CS_OPT_MEM = 4,
    CS_OPT_SKIPDATA = 5,
    CS_OPT_SKIPDATA_SETUP = 6,
}

pub type cs_opt_value = u32;

pub const CS_OPT_OFF: cs_opt_value = 0;
pub const CS_OPT_ON: cs_opt_value = 3;

pub const CS_OPT_SYNTAX_DEFAULT: cs_opt_value = 0;
pub const CS_OPT_SYNTAX_INTEL: cs_opt_value = 1;
pub const CS_OPT_SYNTAX_ATT: cs_opt_value = 2;
pub const CS_OPT_SYNTAX_NOREGNAME: cs_opt_value = 3;

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum cs_op_type {
    CS_OP_INVALID = 0,
    CS_OP_REG = 1,
    CS_OP_IMM = 2,
    CS_OP_MEM = 3,
    CS_OP_FP = 4,
}

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum cs_group_type {
    CS_GRP_INVALID = 0,
    CS_GRP_JUMP = 1,
    CS_GRP_CALL = 2,
    CS_GRP_RET = 3,
    CS_GRP_INT = 4,
    CS_GRP_IRET = 5,
}

pub type cs_skipdata_cb_t = Option<unsafe extern "C" fn(code: *const u8,
                                                        code_size: size_t,
                                                        offset: size_t,
                                                        user_data: *mut c_void)
                                                        -> size_t>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs_opt_skipdata {
    pub mnemonic: *const c_char,
    pub callback: cs_skipdata_cb_t,
    pub user_data: *mut c_void,
}
impl ::std::default::Default for cs_opt_skipdata {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

pub mod arm;

pub mod arm64;

pub mod mips;

pub mod ppc;

pub mod sparc;

pub mod sysz;

pub mod x86;

pub mod xcore;

#[repr(C)]
pub struct cs_detail {
    pub regs_read: [u8; 12usize],
    pub regs_read_count: u8,
    pub regs_write: [u8; 20usize],
    pub regs_write_count: u8,
    pub groups: [u8; 8usize],
    pub groups_count: u8,
    data: placeholders::detail_data,
}

impl cs_detail {
    pub unsafe fn x86(&self) -> &x86::cs_x86 {
        ::std::mem::transmute(&self.data)
    }
    pub unsafe fn arm64(&self) -> &arm64::cs_arm64 {
        ::std::mem::transmute(&self.data)
    }
    pub unsafe fn arm(&self) -> &arm::cs_arm {
        ::std::mem::transmute(&self.data)
    }
    pub unsafe fn mips(&self) -> &mips::cs_mips {
        ::std::mem::transmute(&self.data)
    }
    pub unsafe fn ppc(&self) -> &ppc::cs_ppc {
        ::std::mem::transmute(&self.data)
    }
    pub unsafe fn sparc(&self) -> &sparc::cs_sparc {
        ::std::mem::transmute(&self.data)
    }
    pub unsafe fn sysz(&self) -> &sysz::cs_sysz {
        ::std::mem::transmute(&self.data)
    }
    pub unsafe fn xcore(&self) -> &xcore::cs_xcore {
        ::std::mem::transmute(&self.data)
    }
}

#[repr(C)]
pub struct cs_insn {
    pub id: c_uint,
    pub address: u64,
    pub size: u16,
    pub bytes: [u8; 16usize],
    pub mnemonic: [c_char; 32usize],
    pub op_str: [c_char; 160usize],
    pub detail: *mut cs_detail,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(u32)]
pub enum cs_err {
    CS_ERR_OK = 0,
    CS_ERR_MEM = 1,
    CS_ERR_ARCH = 2,
    CS_ERR_HANDLE = 3,
    CS_ERR_CSH = 4,
    CS_ERR_MODE = 5,
    CS_ERR_OPTION = 6,
    CS_ERR_DETAIL = 7,
    CS_ERR_MEMSETUP = 8,
    CS_ERR_VERSION = 9,
    CS_ERR_DIET = 10,
    CS_ERR_SKIPDATA = 11,
    CS_ERR_X86_ATT = 12,
    CS_ERR_X86_INTEL = 13,
}

#[link(name = "capstone", kind = "dylib")]
extern "C" {
    pub fn cs_version(major: *mut c_int, minor: *mut c_int) -> c_uint;
    pub fn cs_support(query: c_int) -> u8;
    pub fn cs_open(arch: cs_arch, mode: cs_mode, handle: *mut csh) -> cs_err;
    pub fn cs_close(handle: *mut csh) -> cs_err;
    pub fn cs_option(handle: csh, _type: cs_opt_type, value: size_t) -> cs_err;
    pub fn cs_errno(handle: csh) -> cs_err;
    pub fn cs_strerror(code: cs_err) -> *const c_char;
    pub fn cs_disasm(handle: csh,
                     code: *const u8,
                     code_size: size_t,
                     address: u64,
                     count: size_t,
                     insn: *mut *mut cs_insn)
                     -> size_t;
    pub fn cs_free(insn: *mut cs_insn, count: size_t);
    pub fn cs_malloc(handle: csh) -> *mut cs_insn;
    pub fn cs_disasm_iter(handle: csh,
                          code: *mut *const u8,
                          size: *mut size_t,
                          address: *mut u64,
                          insn: *mut cs_insn)
                          -> u8;
    pub fn cs_reg_name(handle: csh, reg_id: c_uint) -> *const c_char;
    pub fn cs_insn_name(handle: csh, insn_id: c_uint) -> *const c_char;
    pub fn cs_group_name(handle: csh, group_id: c_uint) -> *const c_char;
    pub fn cs_insn_group(handle: csh, insn: *const cs_insn, group_id: c_uint) -> u8;
    pub fn cs_reg_read(handle: csh, insn: *const cs_insn, reg_id: c_uint) -> u8;
    pub fn cs_reg_write(handle: csh, insn: *const cs_insn, reg_id: c_uint) -> u8;
    pub fn cs_op_count(handle: csh, insn: *const cs_insn, op_type: c_uint) -> c_int;
    pub fn cs_op_index(handle: csh,
                       insn: *const cs_insn,
                       op_type: c_uint,
                       position: c_uint)
                       -> c_int;
}
