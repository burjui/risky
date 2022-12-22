//! [`Csr`] type and standard CSR definitions

use core::fmt;
use std::{
    error::Error,
    fmt::{Debug, Display},
};

use crate::util::{
    i16_fits_n_bits_unsigned, i32_fits_n_bits_unsigned, i64_fits_n_bits_unsigned,
    isize_fits_n_bits_unsigned, u16_fits_n_bits, u32_fits_n_bits, u64_fits_n_bits,
    usize_fits_n_bits,
};

/// 12-bit unsigned value representing a CSR (Control and Status Register)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Csr(pub(crate) u16);

impl Csr {
    // --- Unprivileged CSR addresses ---

    // Unprivileged Floating-Point CSRs

    /// (URW) Floating-Point Accrued Exceptions
    pub const FFLAGS: Self = Self(0x001);
    /// (URW) Floating-Point Dynamic Rounding Mode
    pub const FRM: Self = Self(0x002);
    /// (URW) Floating-Point Control and Status Register (FRM + FFLAGS)
    pub const FESR: Self = Self(0x003);

    // Unprivileged Counter/Timers

    /// (URO) Cycle counter for RDCYCLE instruction
    pub const CYCLE: Self = Self(0xC00);
    /// (URO) Timer for RDTIME instruction
    pub const TIME: Self = Self(0xC01);
    /// (URO) Instructions-retired counter for RDINSTRET instruction
    pub const INSTRET: Self = Self(0xC02);
    /// (URO) Performance-monitoring counter
    pub const HPMCOUNTER3: Self = Self(0xC03);
    /// (URO) Performance-monitoring counter
    pub const HPMCOUNTER4: Self = Self(0xC04);
    /// (URO) Performance-monitoring counter
    pub const HPMCOUNTER5: Self = Self(0xC05);
    /// (URO) Performance-monitoring counter
    pub const HPMCOUNTER6: Self = Self(0xC06);
    /// (URO) Performance-monitoring counter
    pub const HPMCOUNTER7: Self = Self(0xC07);
    /// (URO) Performance-monitoring counter
    pub const HPMCOUNTER8: Self = Self(0xC08);
    /// (URO) Performance-monitoring counter
    pub const HPMCOUNTER9: Self = Self(0xC09);
    /// (URO) Performance-monitoring counter
    pub const HPMCOUNTER10: Self = Self(0xC0A);
    /// (URO) Performance-monitoring counter
    pub const HPMCOUNTER11: Self = Self(0xC0B);
    /// (URO) Performance-monitoring counter
    pub const HPMCOUNTER12: Self = Self(0xC0C);
    /// (URO) Performance-monitoring counter
    pub const HPMCOUNTER13: Self = Self(0xC0D);
    /// (URO) Performance-monitoring counter
    pub const HPMCOUNTER14: Self = Self(0xC0E);
    /// (URO) Performance-monitoring counter
    pub const HPMCOUNTER15: Self = Self(0xC0F);
    /// (URO) Performance-monitoring counter
    pub const HPMCOUNTER16: Self = Self(0xC10);
    /// (URO) Performance-monitoring counter
    pub const HPMCOUNTER17: Self = Self(0xC11);
    /// (URO) Performance-monitoring counter
    pub const HPMCOUNTER18: Self = Self(0xC12);
    /// (URO) Performance-monitoring counter
    pub const HPMCOUNTER19: Self = Self(0xC13);
    /// (URO) Performance-monitoring counter
    pub const HPMCOUNTER20: Self = Self(0xC14);
    /// (URO) Performance-monitoring counter
    pub const HPMCOUNTER21: Self = Self(0xC15);
    /// (URO) Performance-monitoring counter
    pub const HPMCOUNTER22: Self = Self(0xC16);
    /// (URO) Performance-monitoring counter
    pub const HPMCOUNTER23: Self = Self(0xC17);
    /// (URO) Performance-monitoring counter
    pub const HPMCOUNTER24: Self = Self(0xC18);
    /// (URO) Performance-monitoring counter
    pub const HPMCOUNTER25: Self = Self(0xC19);
    /// (URO) Performance-monitoring counter
    pub const HPMCOUNTER26: Self = Self(0xC1A);
    /// (URO) Performance-monitoring counter
    pub const HPMCOUNTER27: Self = Self(0xC1B);
    /// (URO) Performance-monitoring counter
    pub const HPMCOUNTER28: Self = Self(0xC1C);
    /// (URO) Performance-monitoring counter
    pub const HPMCOUNTER29: Self = Self(0xC1D);
    /// (URO) Performance-monitoring counter
    pub const HPMCOUNTER30: Self = Self(0xC1E);
    /// (URO) Performance-monitoring counter
    pub const HPMCOUNTER31: Self = Self(0xC1F);
    /// (URO) Upper 32 bits of [`CYCLE`](Self::CYCLE), RV32 only
    pub const CYCLEH: Self = Self(0xC80);
    /// (URO) Upper 32 bits of [`TIME`](Self::TIME), RV32 only
    pub const TIMEH: Self = Self(0xC81);
    /// (URO) Upper 32 bits of [`INSTRET`](Self::INSTRET), RV32 only
    pub const INSTRETH: Self = Self(0xC82);
    /// (URO) Upper 32 bits of [`HPMCOUNTER3`](Self::HPMCOUNTER3), RV32 only
    pub const HPMCOUNTER3H: Self = Self(0xC83);
    /// (URO) Upper 32 bits of [`HPMCOUNTER4`](Self::HPMCOUNTER4), RV32 only
    pub const HPMCOUNTER4H: Self = Self(0xC84);
    /// (URO) Upper 32 bits of [`HPMCOUNTER5`](Self::HPMCOUNTER5), RV32 only
    pub const HPMCOUNTER5H: Self = Self(0xC85);
    /// (URO) Upper 32 bits of [`HPMCOUNTER6`](Self::HPMCOUNTER6), RV32 only
    pub const HPMCOUNTER6H: Self = Self(0xC86);
    /// (URO) Upper 32 bits of [`HPMCOUNTER7`](Self::HPMCOUNTER7), RV32 only
    pub const HPMCOUNTER7H: Self = Self(0xC87);
    /// (URO) Upper 32 bits of [`HPMCOUNTER8`](Self::HPMCOUNTER8), RV32 only
    pub const HPMCOUNTER8H: Self = Self(0xC88);
    /// (URO) Upper 32 bits of [`HPMCOUNTER9`](Self::HPMCOUNTER9), RV32 only
    pub const HPMCOUNTER9H: Self = Self(0xC89);
    /// (URO) Upper 32 bits of [`HPMCOUNTER10`](Self::HPMCOUNTER10), RV32 only
    pub const HPMCOUNTER10H: Self = Self(0xC8A);
    /// (URO) Upper 32 bits of [`HPMCOUNTER11`](Self::HPMCOUNTER11), RV32 only
    pub const HPMCOUNTER11H: Self = Self(0xC8B);
    /// (URO) Upper 32 bits of [`HPMCOUNTER12`](Self::HPMCOUNTER12), RV32 only
    pub const HPMCOUNTER12H: Self = Self(0xC8C);
    /// (URO) Upper 32 bits of [`HPMCOUNTER13`](Self::HPMCOUNTER13), RV32 only
    pub const HPMCOUNTER13H: Self = Self(0xC8D);
    /// (URO) Upper 32 bits of [`HPMCOUNTER14`](Self::HPMCOUNTER14), RV32 only
    pub const HPMCOUNTER14H: Self = Self(0xC8E);
    /// (URO) Upper 32 bits of [`HPMCOUNTER15`](Self::HPMCOUNTER15), RV32 only
    pub const HPMCOUNTER15H: Self = Self(0xC8F);
    /// (URO) Upper 32 bits of [`HPMCOUNTER16`](Self::HPMCOUNTER16), RV32 only
    pub const HPMCOUNTER16H: Self = Self(0xC90);
    /// (URO) Upper 32 bits of [`HPMCOUNTER17`](Self::HPMCOUNTER17), RV32 only
    pub const HPMCOUNTER17H: Self = Self(0xC91);
    /// (URO) Upper 32 bits of [`HPMCOUNTER18`](Self::HPMCOUNTER18), RV32 only
    pub const HPMCOUNTER18H: Self = Self(0xC92);
    /// (URO) Upper 32 bits of [`HPMCOUNTER19`](Self::HPMCOUNTER19), RV32 only
    pub const HPMCOUNTER19H: Self = Self(0xC93);
    /// (URO) Upper 32 bits of [`HPMCOUNTER20`](Self::HPMCOUNTER20), RV32 only
    pub const HPMCOUNTER20H: Self = Self(0xC94);
    /// (URO) Upper 32 bits of [`HPMCOUNTER21`](Self::HPMCOUNTER21), RV32 only
    pub const HPMCOUNTER21H: Self = Self(0xC95);
    /// (URO) Upper 32 bits of [`HPMCOUNTER22`](Self::HPMCOUNTER22), RV32 only
    pub const HPMCOUNTER22H: Self = Self(0xC96);
    /// (URO) Upper 32 bits of [`HPMCOUNTER23`](Self::HPMCOUNTER23), RV32 only
    pub const HPMCOUNTER23H: Self = Self(0xC97);
    /// (URO) Upper 32 bits of [`HPMCOUNTER24`](Self::HPMCOUNTER24), RV32 only
    pub const HPMCOUNTER24H: Self = Self(0xC98);
    /// (URO) Upper 32 bits of [`HPMCOUNTER25`](Self::HPMCOUNTER25), RV32 only
    pub const HPMCOUNTER25H: Self = Self(0xC99);
    /// (URO) Upper 32 bits of [`HPMCOUNTER26`](Self::HPMCOUNTER26), RV32 only
    pub const HPMCOUNTER26H: Self = Self(0xC9A);
    /// (URO) Upper 32 bits of [`HPMCOUNTER27`](Self::HPMCOUNTER27), RV32 only
    pub const HPMCOUNTER27H: Self = Self(0xC9B);
    /// (URO) Upper 32 bits of [`HPMCOUNTER28`](Self::HPMCOUNTER28), RV32 only
    pub const HPMCOUNTER28H: Self = Self(0xC9C);
    /// (URO) Upper 32 bits of [`HPMCOUNTER29`](Self::HPMCOUNTER29), RV32 only
    pub const HPMCOUNTER29H: Self = Self(0xC9D);
    /// (URO) Upper 32 bits of [`HPMCOUNTER30`](Self::HPMCOUNTER30), RV32 only
    pub const HPMCOUNTER30H: Self = Self(0xC9E);
    /// (URO) Upper 32 bits of [`HPMCOUNTER31`](Self::HPMCOUNTER31), RV32 only
    pub const HPMCOUNTER31H: Self = Self(0xC9F);

    // --- Supervisor-level CSR addresses ---

    // Supervisor Trap Setup

    /// (SRW) Supervisor status register
    pub const SSTATUS: Self = Self(0x100);
    /// (SRW) Supervisor interrupt-enable register
    pub const SIE: Self = Self(0x104);
    /// (SRW) Supervisor trap handler base address
    pub const STVEC: Self = Self(0x105);
    /// (SRW) Supervisor counter enable
    pub const SCOUNTEREN: Self = Self(0x106);

    // Supervisor Configuration

    /// (SRW) Supervisor environment configuration register
    pub const SENVCFG: Self = Self(0x10A);

    // Supervisor Trap Handling

    /// (SRW) Scratch register for supervisor trap handlers
    pub const SSCRATCH: Self = Self(0x140);
    /// (SRW) Supervisor exception program counter
    pub const SEPC: Self = Self(0x141);
    /// (SRW) Supervisor trap cause
    pub const SCAUSE: Self = Self(0x142);
    /// (SRW) Supervisor bad address or instruction
    pub const STVAL: Self = Self(0x143);
    /// (SRW) Supervisor interrupt pending
    pub const SIP: Self = Self(0x144);

    // Supervisor Protection and Translation

    /// (SRW) Supervisor address translation and protection
    pub const SATP: Self = Self(0x180);

    // Debug/Trace Registers

    /// (SRW) Supervisor-mode context register
    pub const SCONTEXT: Self = Self(0x5A8);

    // --- Hypervisor and VS CSR addresses ---

    // Hypervisor Trap Setup

    /// (HRW) Hypervisor status register
    pub const HSTATUS: Self = Self(0x600);
    /// (HRW) Hypervisor exception delegation register
    pub const HEDELEG: Self = Self(0x602);
    /// (HRW) Hypervisor interrupt delegation register
    pub const HIDELEG: Self = Self(0x603);
    /// (HRW) Hypervisor interrupt-enable register
    pub const HIE: Self = Self(0x604);
    /// (HRW) Hypervisor counter enable
    pub const HCOUNTEREN: Self = Self(0x606);
    /// (HRW) Hypervisor guest external interrupt-enable register
    pub const HGEIE: Self = Self(0x607);

    // Hypervisor Trap Handling

    /// (HRW) Hypervisor bad guest physical address
    pub const HTVAL: Self = Self(0x643);
    /// (HRW) Hypervisor interrupt pending
    pub const HIP: Self = Self(0x644);
    /// (HRW) Hypervisor virtual interrupt pending
    pub const HVIP: Self = Self(0x645);
    /// (HRW) Hypervisor trap instruction (transformed)
    pub const HTINST: Self = Self(0x64A);
    /// (HRO) Hypervisor guest external interrupt pending
    pub const HGEIP: Self = Self(0xE12);

    // Hypervisor Configuration

    /// (HRW) Hypervisor environment configuration register
    pub const HENVCFG: Self = Self(0x60A);
    /// (HRW) Additional hypervisor environment configuration register, RV32 only
    pub const HENVCFGH: Self = Self(0x61A);

    // Hypervisor Protection and Translation

    /// (HRW) Hypervisor guest address translation and protection
    pub const HGATP: Self = Self(0x680);

    // Debug/Trace Registers

    /// (HRW) Hypervisor-mode context register
    pub const HCONTEXT: Self = Self(0x6A8);

    // Hypervisor Counter/Timer Virtualization Registers

    /// (HRW) Delta for VS/VU-mode timer
    pub const HTIMEDELTA: Self = Self(0x605);
    /// (HRW) Upper 32 bits of [`HTIMEDELTA`](Self::HTIMEDELTA), HSXLEN=32 only
    pub const HTIMEDELTAH: Self = Self(0x615);

    // Virtual Supervisor Registers

    /// (HRW) Virtual supervisor status register
    pub const VSSTATUS: Self = Self(0x200);
    /// (HRW) Virtual supervisor interrupt-enable register
    pub const VSIE: Self = Self(0x204);
    /// (HRW) Virtual supervisor trap handler base address
    pub const VSTVEC: Self = Self(0x205);
    /// (HRW) Virtual supervisor scratch register
    pub const VSSCRATCH: Self = Self(0x240);
    /// (HRW) Virtual supervisor exception program counter
    pub const VSEPC: Self = Self(0x241);
    /// (HRW) Virtual supervisor trap cause
    pub const VSCAUSE: Self = Self(0x242);
    /// (HRW) Virtual supervisor bad address or instruction
    pub const VSTVAL: Self = Self(0x243);
    /// (HRW) Virtual supervisor interrupt pending
    pub const VSIP: Self = Self(0x244);
    /// (HRW) Virtual supervisor address translation and protection
    pub const VSATP: Self = Self(0x280);

    // --- Machine-level CSR addresses ---

    // Machine Information Registers

    /// (MRO) Vendor ID
    pub const MVENDORID: Self = Self(0xF11);
    /// (MRO) Architecture ID
    pub const MARCHID: Self = Self(0xF12);
    /// (MRO) Implementation ID
    pub const MIMPID: Self = Self(0xF13);
    /// (MRO) Hardware thread ID
    pub const MHARTID: Self = Self(0xF14);
    /// (MRO) Pointer to configuration data structure
    pub const MCONFIGPTR: Self = Self(0xF15);

    // Machine Trap Setup

    /// (MRW) Machine status register
    pub const MSTATUS: Self = Self(0x300);
    /// (MRW) ISA and extensions
    pub const MISA: Self = Self(0x301);
    /// (MRW) Machine exception delegation register
    pub const MEDELEG: Self = Self(0x302);
    /// (MRW) Machine interrupt delegation register
    pub const MIDELEG: Self = Self(0x303);
    /// (MRW) Machine interrupt-enable register
    pub const MIE: Self = Self(0x304);
    /// (MRW) Machine trap-handler base address
    pub const MTVEC: Self = Self(0x305);
    /// (MRW) Machine counter enable
    pub const MCOUNTEREN: Self = Self(0x306);
    /// (MRW) Additional machine status register, RV32 only
    pub const MSTATUSH: Self = Self(0x310);

    // Machine Trap Handling

    /// (MRW) Scratch register for machine trap handlers
    pub const MSCRATCH: Self = Self(0x340);
    /// (MRW) Machine exception program counter
    pub const MEPC: Self = Self(0x341);
    /// (MRW) Machine trap cause
    pub const MCAUSE: Self = Self(0x342);
    /// (MRW) Machine bad address or instruction
    pub const MTVAL: Self = Self(0x343);
    /// (MRW) Machine interrupt pending
    pub const MIP: Self = Self(0x344);
    /// (MRW) Machine trap instruction (transformed)
    pub const MTINST: Self = Self(0x34A);
    /// (MRW) Machine bad guest physical address
    pub const MTVAL2: Self = Self(0x34B);

    // Machine Configuration

    /// (MRW) Machine environment configuration register
    pub const MENVCFG: Self = Self(0x30A);
    /// (MRW) Additional machine environment configuration register, RV32 only
    pub const MENVCFGH: Self = Self(0x31A);
    /// (MRW) Machine security configuration register
    pub const MSECCFG: Self = Self(0x747);
    /// (MRW) Additional machine security configuration register, RV32 only
    pub const MSECCFGH: Self = Self(0x757);

    // Machine Memory Protection

    /// (MRW) Physical memory protection configuration
    pub const PMPCFG0: Self = Self(0x3A0);
    /// (MRW) Physical memory protection configuration, RV32 only
    pub const PMPCFG1: Self = Self(0x3A1);
    /// (MRW) Physical memory protection configuration
    pub const PMPCFG2: Self = Self(0x3A2);
    /// (MRW) Physical memory protection configuration, RV32 only
    pub const PMPCFG3: Self = Self(0x3A3);
    /// (MRW) Physical memory protection configuration
    pub const PMPCFG4: Self = Self(0x3A4);
    /// (MRW) Physical memory protection configuration, RV32 only
    pub const PMPCFG5: Self = Self(0x3A5);
    /// (MRW) Physical memory protection configuration
    pub const PMPCFG6: Self = Self(0x3A6);
    /// (MRW) Physical memory protection configuration, RV32 only
    pub const PMPCFG7: Self = Self(0x3A7);
    /// (MRW) Physical memory protection configuration
    pub const PMPCFG8: Self = Self(0x3A8);
    /// (MRW) Physical memory protection configuration, RV32 only
    pub const PMPCFG9: Self = Self(0x3A9);
    /// (MRW) Physical memory protection configuration
    pub const PMPCFG10: Self = Self(0x3AA);
    /// (MRW) Physical memory protection configuration, RV32 only
    pub const PMPCFG11: Self = Self(0x3AB);
    /// (MRW) Physical memory protection configuration
    pub const PMPCFG12: Self = Self(0x3AC);
    /// (MRW) Physical memory protection configuration, RV32 only
    pub const PMPCFG13: Self = Self(0x3AD);
    /// (MRW) Physical memory protection configuration
    pub const PMPCFG14: Self = Self(0x3AE);
    /// (MRW) Physical memory protection configuration, RV32 only
    pub const PMPCFG15: Self = Self(0x3AF);

    /// (MRW) Physical memory protection address register
    pub const PMPADDR0: Self = Self(0x3B0);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR1: Self = Self(0x3B1);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR2: Self = Self(0x3B2);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR3: Self = Self(0x3B3);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR4: Self = Self(0x3B4);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR5: Self = Self(0x3B5);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR6: Self = Self(0x3B6);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR7: Self = Self(0x3B7);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR8: Self = Self(0x3B8);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR9: Self = Self(0x3B9);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR10: Self = Self(0x3BA);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR11: Self = Self(0x3BB);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR12: Self = Self(0x3BC);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR13: Self = Self(0x3BD);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR14: Self = Self(0x3BE);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR15: Self = Self(0x3BF);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR16: Self = Self(0x3C0);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR17: Self = Self(0x3C1);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR18: Self = Self(0x3C2);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR19: Self = Self(0x3C3);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR20: Self = Self(0x3C4);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR21: Self = Self(0x3C5);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR22: Self = Self(0x3C6);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR23: Self = Self(0x3C7);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR24: Self = Self(0x3C8);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR25: Self = Self(0x3C9);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR26: Self = Self(0x3CA);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR27: Self = Self(0x3CB);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR28: Self = Self(0x3CC);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR29: Self = Self(0x3CD);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR30: Self = Self(0x3CE);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR31: Self = Self(0x3CF);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR32: Self = Self(0x3D0);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR33: Self = Self(0x3D1);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR34: Self = Self(0x3D2);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR35: Self = Self(0x3D3);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR36: Self = Self(0x3D4);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR37: Self = Self(0x3D5);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR38: Self = Self(0x3D6);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR39: Self = Self(0x3D7);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR40: Self = Self(0x3D8);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR41: Self = Self(0x3D9);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR42: Self = Self(0x3DA);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR43: Self = Self(0x3DB);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR44: Self = Self(0x3DC);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR45: Self = Self(0x3DD);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR46: Self = Self(0x3DE);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR47: Self = Self(0x3DF);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR48: Self = Self(0x3E0);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR49: Self = Self(0x3E1);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR50: Self = Self(0x3E2);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR51: Self = Self(0x3E3);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR52: Self = Self(0x3E4);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR53: Self = Self(0x3E5);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR54: Self = Self(0x3E6);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR55: Self = Self(0x3E7);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR56: Self = Self(0x3E8);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR57: Self = Self(0x3E9);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR58: Self = Self(0x3EA);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR59: Self = Self(0x3EB);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR60: Self = Self(0x3EC);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR61: Self = Self(0x3ED);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR62: Self = Self(0x3EE);
    /// (MRW) Physical memory protection address register
    pub const PMPADDR63: Self = Self(0x3EF);

    // Machine Non-Maskable Interrupt Handling

    /// (MRW) Resumable NMI scratch register
    pub const MNSCRATCH: Self = Self(0x740);
    /// (MRW) Resumable NMI program counter
    pub const MNEPC: Self = Self(0x741);
    /// (MRW) Resumable NMI cause
    pub const MNCAUSE: Self = Self(0x742);
    /// (MRW) Resumable NMI status
    pub const MNSTATUS: Self = Self(0x744);

    // Machine Counter/Timers

    /// (MRW) Machine cycle counter
    pub const MCYCLE: Self = Self(0xB00);
    /// (MRW) Machine instructions-retired counter
    pub const MINSTRET: Self = Self(0xB02);
    /// (MRW) Machine performance-monitoring counter
    pub const MHPMCOUNTER3: Self = Self(0xB03);
    /// (MRW) Machine performance-monitoring counter
    pub const MHPMCOUNTER4: Self = Self(0xB04);
    /// (MRW) Machine performance-monitoring counter
    pub const MHPMCOUNTER5: Self = Self(0xB05);
    /// (MRW) Machine performance-monitoring counter
    pub const MHPMCOUNTER6: Self = Self(0xB06);
    /// (MRW) Machine performance-monitoring counter
    pub const MHPMCOUNTER7: Self = Self(0xB07);
    /// (MRW) Machine performance-monitoring counter
    pub const MHPMCOUNTER8: Self = Self(0xB08);
    /// (MRW) Machine performance-monitoring counter
    pub const MHPMCOUNTER9: Self = Self(0xB09);
    /// (MRW) Machine performance-monitoring counter
    pub const MHPMCOUNTER10: Self = Self(0xB0A);
    /// (MRW) Machine performance-monitoring counter
    pub const MHPMCOUNTER11: Self = Self(0xB0B);
    /// (MRW) Machine performance-monitoring counter
    pub const MHPMCOUNTER12: Self = Self(0xB0C);
    /// (MRW) Machine performance-monitoring counter
    pub const MHPMCOUNTER13: Self = Self(0xB0D);
    /// (MRW) Machine performance-monitoring counter
    pub const MHPMCOUNTER14: Self = Self(0xB0E);
    /// (MRW) Machine performance-monitoring counter
    pub const MHPMCOUNTER15: Self = Self(0xB0F);
    /// (MRW) Machine performance-monitoring counter
    pub const MHPMCOUNTER16: Self = Self(0xB10);
    /// (MRW) Machine performance-monitoring counter
    pub const MHPMCOUNTER17: Self = Self(0xB11);
    /// (MRW) Machine performance-monitoring counter
    pub const MHPMCOUNTER18: Self = Self(0xB12);
    /// (MRW) Machine performance-monitoring counter
    pub const MHPMCOUNTER19: Self = Self(0xB13);
    /// (MRW) Machine performance-monitoring counter
    pub const MHPMCOUNTER20: Self = Self(0xB14);
    /// (MRW) Machine performance-monitoring counter
    pub const MHPMCOUNTER21: Self = Self(0xB15);
    /// (MRW) Machine performance-monitoring counter
    pub const MHPMCOUNTER22: Self = Self(0xB16);
    /// (MRW) Machine performance-monitoring counter
    pub const MHPMCOUNTER23: Self = Self(0xB17);
    /// (MRW) Machine performance-monitoring counter
    pub const MHPMCOUNTER24: Self = Self(0xB18);
    /// (MRW) Machine performance-monitoring counter
    pub const MHPMCOUNTER25: Self = Self(0xB19);
    /// (MRW) Machine performance-monitoring counter
    pub const MHPMCOUNTER26: Self = Self(0xB1A);
    /// (MRW) Machine performance-monitoring counter
    pub const MHPMCOUNTER27: Self = Self(0xB1B);
    /// (MRW) Machine performance-monitoring counter
    pub const MHPMCOUNTER28: Self = Self(0xB1C);
    /// (MRW) Machine performance-monitoring counter
    pub const MHPMCOUNTER29: Self = Self(0xB1D);
    /// (MRW) Machine performance-monitoring counter
    pub const MHPMCOUNTER30: Self = Self(0xB1E);
    /// (MRW) Machine performance-monitoring counter
    pub const MHPMCOUNTER31: Self = Self(0xB1F);
    /// (MRW) Upper 32 bits of mcycle, RV32 only
    pub const MCYCLEH: Self = Self(0xB80);
    /// (MRW) Upper 32 bits of minstret, RV32 only
    pub const MINSTRETH: Self = Self(0xB82);
    /// (MRW) Upper 32 bits of [`MHPMCOUNTER3`](Self::MHPMCOUNTER3), RV32 only
    pub const MHPMCOUNTER3H: Self = Self(0xB83);
    /// (MRW) Upper 32 bits of [`MHPMCOUNTER4`](Self::MHPMCOUNTER4), RV32 only
    pub const MHPMCOUNTER4H: Self = Self(0xB84);
    /// (MRW) Upper 32 bits of [`MHPMCOUNTER5`](Self::MHPMCOUNTER5), RV32 only
    pub const MHPMCOUNTER5H: Self = Self(0xB85);
    /// (MRW) Upper 32 bits of [`MHPMCOUNTER6`](Self::MHPMCOUNTER6), RV32 only
    pub const MHPMCOUNTER6H: Self = Self(0xB86);
    /// (MRW) Upper 32 bits of [`MHPMCOUNTER7`](Self::MHPMCOUNTER7), RV32 only
    pub const MHPMCOUNTER7H: Self = Self(0xB87);
    /// (MRW) Upper 32 bits of [`MHPMCOUNTER8`](Self::MHPMCOUNTER8), RV32 only
    pub const MHPMCOUNTER8H: Self = Self(0xB88);
    /// (MRW) Upper 32 bits of [`MHPMCOUNTER9`](Self::MHPMCOUNTER9), RV32 only
    pub const MHPMCOUNTER9H: Self = Self(0xB89);
    /// (MRW) Upper 32 bits of [`MHPMCOUNTER10`](Self::MHPMCOUNTER10), RV32 only
    pub const MHPMCOUNTER10H: Self = Self(0xB8A);
    /// (MRW) Upper 32 bits of [`MHPMCOUNTER11`](Self::MHPMCOUNTER11), RV32 only
    pub const MHPMCOUNTER11H: Self = Self(0xB8B);
    /// (MRW) Upper 32 bits of [`MHPMCOUNTER12`](Self::MHPMCOUNTER12), RV32 only
    pub const MHPMCOUNTER12H: Self = Self(0xB8C);
    /// (MRW) Upper 32 bits of [`MHPMCOUNTER13`](Self::MHPMCOUNTER13), RV32 only
    pub const MHPMCOUNTER13H: Self = Self(0xB8D);
    /// (MRW) Upper 32 bits of [`MHPMCOUNTER14`](Self::MHPMCOUNTER14), RV32 only
    pub const MHPMCOUNTER14H: Self = Self(0xB8E);
    /// (MRW) Upper 32 bits of [`MHPMCOUNTER15`](Self::MHPMCOUNTER15), RV32 only
    pub const MHPMCOUNTER15H: Self = Self(0xB8F);
    /// (MRW) Upper 32 bits of [`MHPMCOUNTER16`](Self::MHPMCOUNTER16), RV32 only
    pub const MHPMCOUNTER16H: Self = Self(0xB90);
    /// (MRW) Upper 32 bits of [`MHPMCOUNTER17`](Self::MHPMCOUNTER17), RV32 only
    pub const MHPMCOUNTER17H: Self = Self(0xB91);
    /// (MRW) Upper 32 bits of [`MHPMCOUNTER18`](Self::MHPMCOUNTER18), RV32 only
    pub const MHPMCOUNTER18H: Self = Self(0xB92);
    /// (MRW) Upper 32 bits of [`MHPMCOUNTER19`](Self::MHPMCOUNTER19), RV32 only
    pub const MHPMCOUNTER19H: Self = Self(0xB93);
    /// (MRW) Upper 32 bits of [`MHPMCOUNTER20`](Self::MHPMCOUNTER20), RV32 only
    pub const MHPMCOUNTER20H: Self = Self(0xB94);
    /// (MRW) Upper 32 bits of [`MHPMCOUNTER21`](Self::MHPMCOUNTER21), RV32 only
    pub const MHPMCOUNTER21H: Self = Self(0xB95);
    /// (MRW) Upper 32 bits of [`MHPMCOUNTER22`](Self::MHPMCOUNTER22), RV32 only
    pub const MHPMCOUNTER22H: Self = Self(0xB96);
    /// (MRW) Upper 32 bits of [`MHPMCOUNTER23`](Self::MHPMCOUNTER23), RV32 only
    pub const MHPMCOUNTER23H: Self = Self(0xB97);
    /// (MRW) Upper 32 bits of [`MHPMCOUNTER24`](Self::MHPMCOUNTER24), RV32 only
    pub const MHPMCOUNTER24H: Self = Self(0xB98);
    /// (MRW) Upper 32 bits of [`MHPMCOUNTER25`](Self::MHPMCOUNTER25), RV32 only
    pub const MHPMCOUNTER25H: Self = Self(0xB99);
    /// (MRW) Upper 32 bits of [`MHPMCOUNTER26`](Self::MHPMCOUNTER26), RV32 only
    pub const MHPMCOUNTER26H: Self = Self(0xB9A);
    /// (MRW) Upper 32 bits of [`MHPMCOUNTER27`](Self::MHPMCOUNTER27), RV32 only
    pub const MHPMCOUNTER27H: Self = Self(0xB9B);
    /// (MRW) Upper 32 bits of [`MHPMCOUNTER28`](Self::MHPMCOUNTER28), RV32 only
    pub const MHPMCOUNTER28H: Self = Self(0xB9C);
    /// (MRW) Upper 32 bits of [`MHPMCOUNTER29`](Self::MHPMCOUNTER29), RV32 only
    pub const MHPMCOUNTER29H: Self = Self(0xB9D);
    /// (MRW) Upper 32 bits of [`MHPMCOUNTER30`](Self::MHPMCOUNTER30), RV32 only
    pub const MHPMCOUNTER30H: Self = Self(0xB9E);
    /// (MRW) Upper 32 bits of [`MHPMCOUNTER31`](Self::MHPMCOUNTER31), RV32 only
    pub const MHPMCOUNTER31H: Self = Self(0xB9F);

    // Machine Counter Setup

    /// (MRW) Machine counter-inhibit register
    pub const MCOUNTINHIBIT: Self = Self(0x320);
    /// (MRW) Machine performance-monitoring event selector
    pub const MHPMEVENT3: Self = Self(0x323);
    /// (MRW) Machine performance-monitoring event selector
    pub const MHPMEVENT4: Self = Self(0x324);
    /// (MRW) Machine performance-monitoring event selector
    pub const MHPMEVENT31: Self = Self(0x33F);

    // Debug/Trace Registers (shared with Debug Mode)

    /// (MRW) Debug/Trace trigger register select
    pub const TSELECT: Self = Self(0x7A0);
    /// (MRW) First Debug/Trace trigger data register
    pub const TDATAL: Self = Self(0x7A1);
    /// (MRW) Second Debug/Trace trigger data register
    pub const TDATA2: Self = Self(0x7A2);
    /// (MRW) Third Debug/Trace trigger data register
    pub const TDATA3: Self = Self(0x7A3);
    /// (MRW) Machine-mode context register
    pub const MCONTEXT: Self = Self(0x7A8);

    // Debug Mode Registers

    /// (DRW) Debug control and status register
    pub const DESR: Self = Self(0x7B0);
    /// (DRW) Debug program counter
    pub const DPC: Self = Self(0x7B1);
    /// (DRW) Debug scratch register 0
    pub const DSCRATCH0: Self = Self(0x7B2);
    /// (DRW) Debug scratch register 1
    pub const DSCRATCH1: Self = Self(0x7B3);
}

impl Csr {
    const NBITS: usize = 12;

    /// Creates a `Csr` from an [u8] constant
    #[must_use]
    pub const fn from_u8<const VALUE: u8>() -> Self {
        Self(VALUE as u16)
    }

    #[doc = include_str!("../../doc/nightly_warning.html")]
    ///
    /// Creates a `Csr` from an [u16] constant
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_u16<const VALUE: u16>() -> Self
    where
        internal::Assert<{ u16_fits_n_bits(VALUE, Self::NBITS) }>: internal::Fits12BIts,
    {
        Self(VALUE)
    }

    #[doc = include_str!("../../doc/nightly_warning.html")]
    ///
    /// Creates a `Csr` from an [u32] constant
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_u32<const VALUE: u32>() -> Self
    where
        internal::Assert<{ u32_fits_n_bits(VALUE, Self::NBITS) }>: internal::Fits12BIts,
    {
        Self(VALUE as u16)
    }

    #[doc = include_str!("../../doc/nightly_warning.html")]
    ///
    /// Creates a `Csr` from an [u64] constant
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_u64<const VALUE: u64>() -> Self
    where
        internal::Assert<{ u64_fits_n_bits(VALUE, Self::NBITS) }>: internal::Fits12BIts,
    {
        Self(VALUE as u16)
    }

    #[doc = include_str!("../../doc/nightly_warning.html")]
    ///
    /// Creates a `Csr` from an [usize] constant
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_usize<const VALUE: usize>() -> Self
    where
        internal::Assert<{ usize_fits_n_bits(VALUE, Self::NBITS) }>: internal::Fits12BIts,
    {
        Self(VALUE as u16)
    }

    #[doc = include_str!("../../doc/nightly_warning.html")]
    ///
    /// Creates a `Csr` from an [i8] constant
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_i8<const VALUE: i8>() -> Self
    where
        internal::Assert<{ VALUE >= 0 }>: internal::Fits12BIts,
    {
        Self(VALUE as u16)
    }

    #[doc = include_str!("../../doc/nightly_warning.html")]
    ///
    /// Creates a `Csr` from an [i16] constant
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_i16<const VALUE: i16>() -> Self
    where
        internal::Assert<{ i16_fits_n_bits_unsigned(VALUE, Self::NBITS) }>: internal::Fits12BIts,
    {
        Self(VALUE as u16)
    }

    #[doc = include_str!("../../doc/nightly_warning.html")]
    ///
    /// Creates a `Csr` from an [i32] constant
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_i32<const VALUE: i32>() -> Self
    where
        internal::Assert<{ i32_fits_n_bits_unsigned(VALUE, Self::NBITS) }>: internal::Fits12BIts,
    {
        Self(VALUE as u16)
    }

    #[doc = include_str!("../../doc/nightly_warning.html")]
    ///
    /// Creates a `Csr` from an [i64] constant
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_i64<const VALUE: i64>() -> Self
    where
        internal::Assert<{ i64_fits_n_bits_unsigned(VALUE, Self::NBITS) }>: internal::Fits12BIts,
    {
        Self(VALUE as u16)
    }

    #[doc = include_str!("../../doc/nightly_warning.html")]
    ///
    /// Creates a `Csr` from an [isize] constant
    #[cfg(feature = "nightly")]
    #[must_use]
    pub const fn from_isize<const VALUE: isize>() -> Self
    where
        internal::Assert<{ isize_fits_n_bits_unsigned(VALUE, Self::NBITS) }>: internal::Fits12BIts,
    {
        Self(VALUE as u16)
    }

    pub(crate) const fn into_u32(self) -> u32 {
        self.0 as u32
    }
}

#[cfg(feature = "nightly")]
#[test]
fn constructors() {
    let _ = Csr::from_u8::<0xFF>();
    let _ = Csr::from_u16::<0xFFF>();
    let _ = Csr::from_u32::<0xFFF>();
    let _ = Csr::from_u64::<0xFFF>();
    let _ = Csr::from_usize::<0xFFF>();
    let _ = Csr::from_i8::<0b1111111>();
    let _ = Csr::from_i16::<0xFFF>();
    let _ = Csr::from_i32::<0xFFF>();
    let _ = Csr::from_i64::<0xFFF>();
    let _ = Csr::from_isize::<0xFFF>();
}

#[test]
fn into_u32() {
    assert_eq!(Csr(0xFF).into_u32(), 0xFF);
}

impl Display for Csr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

#[test]
fn display() -> Result<(), CsrConvError> {
    assert_eq!(Csr::try_from(0xFFF_u16)?.to_string(), "4095");
    Ok(())
}

impl From<u8> for Csr {
    fn from(value: u8) -> Self {
        Self(u16::from(value))
    }
}

impl TryFrom<u16> for Csr {
    type Error = CsrConvError;

    #[allow(clippy::cast_possible_truncation)]
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if u16_fits_n_bits(value, Self::NBITS) {
            Ok(Self(value))
        } else {
            Err(CsrConvError::U16(value))
        }
    }
}

impl TryFrom<u32> for Csr {
    type Error = CsrConvError;

    #[allow(clippy::cast_possible_truncation)]
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if u32_fits_n_bits(value, Self::NBITS) {
            Ok(Self(value as u16))
        } else {
            Err(CsrConvError::U32(value))
        }
    }
}

impl TryFrom<u64> for Csr {
    type Error = CsrConvError;

    #[allow(clippy::cast_possible_truncation)]
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if u64_fits_n_bits(value, Self::NBITS) {
            Ok(Self(value as u16))
        } else {
            Err(CsrConvError::U64(value))
        }
    }
}

impl TryFrom<usize> for Csr {
    type Error = CsrConvError;

    #[allow(clippy::cast_possible_truncation)]
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if usize_fits_n_bits(value, Self::NBITS) {
            Ok(Self(value as u16))
        } else {
            Err(CsrConvError::Usize(value))
        }
    }
}

impl TryFrom<i8> for Csr {
    type Error = CsrConvError;

    #[allow(clippy::cast_sign_loss)]
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if value >= 0 {
            Ok(Self(value as u16))
        } else {
            Err(CsrConvError::I8(value))
        }
    }
}

impl TryFrom<i16> for Csr {
    type Error = CsrConvError;

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        if i16_fits_n_bits_unsigned(value, Self::NBITS) {
            Ok(Self(value as u16))
        } else {
            Err(CsrConvError::I16(value))
        }
    }
}

impl TryFrom<i32> for Csr {
    type Error = CsrConvError;

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if i32_fits_n_bits_unsigned(value, Self::NBITS) {
            Ok(Self(value as u16))
        } else {
            Err(CsrConvError::I32(value))
        }
    }
}

impl TryFrom<i64> for Csr {
    type Error = CsrConvError;

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if i64_fits_n_bits_unsigned(value, Self::NBITS) {
            Ok(Self(value as u16))
        } else {
            Err(CsrConvError::I64(value))
        }
    }
}

impl TryFrom<isize> for Csr {
    type Error = CsrConvError;

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if isize_fits_n_bits_unsigned(value, Self::NBITS) {
            Ok(Self(value as u16))
        } else {
            Err(CsrConvError::Isize(value))
        }
    }
}

#[test]
fn conversions() -> Result<(), CsrConvError> {
    assert_eq!(Csr::from(0xFF_u8), Csr(0xFF));
    assert_eq!(Csr::try_from(0xFFF_u16)?, Csr(0xFFF));
    assert_eq!(Csr::try_from(0xFFF_u32)?, Csr(0xFFF));
    assert_eq!(Csr::try_from(0xFFF_u64)?, Csr(0xFFF));
    assert_eq!(Csr::try_from(0xFFF_usize)?, Csr(0xFFF));
    assert_eq!(Csr::try_from(0b111_1111_i8)?, Csr(0b111_1111));
    assert_eq!(Csr::try_from(0xFFF_i16)?, Csr(0xFFF));
    assert_eq!(Csr::try_from(0xFFF_i32)?, Csr(0xFFF));
    assert_eq!(Csr::try_from(0xFFF_i64)?, Csr(0xFFF));
    assert_eq!(Csr::try_from(0xFFF_isize)?, Csr(0xFFF));

    assert!(matches!(Csr::try_from(-1_i8), Err(CsrConvError::I8(-1))));
    assert!(matches!(Csr::try_from(-1_i16), Err(CsrConvError::I16(-1))));
    assert!(matches!(Csr::try_from(-1_i32), Err(CsrConvError::I32(-1))));
    assert!(matches!(Csr::try_from(-1_i64), Err(CsrConvError::I64(-1))));
    assert!(matches!(
        Csr::try_from(-1_isize),
        Err(CsrConvError::Isize(-1))
    ));

    assert!(matches!(
        Csr::try_from(0x1000_u16),
        Err(CsrConvError::U16(4096))
    ));
    assert!(matches!(
        Csr::try_from(0x1000_u32),
        Err(CsrConvError::U32(4096))
    ));
    assert!(matches!(
        Csr::try_from(0x1000_u64),
        Err(CsrConvError::U64(4096))
    ));
    assert!(matches!(
        Csr::try_from(0x1000_usize),
        Err(CsrConvError::Usize(4096))
    ));

    Ok(())
}

/// Csr conversion error
#[derive(Debug)]
pub enum CsrConvError {
    ///
    U8(u8),
    ///
    U16(u16),
    ///
    U32(u32),
    ///
    U64(u64),
    ///
    Usize(usize),
    ///
    I8(i8),
    ///
    I16(i16),
    ///
    I32(i32),
    ///
    I64(i64),
    ///
    Isize(isize),
}

impl Display for CsrConvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid {}-bit unsigned immediate: ", Csr::NBITS)?;
        match self {
            CsrConvError::U8(value) => write!(f, "{value} (0x{value:02x})"),
            CsrConvError::U16(value) => write!(f, "{value} (0x{value:04x})"),
            CsrConvError::U32(value) => write!(f, "{value} (0x{value:08x})"),
            CsrConvError::U64(value) => write!(f, "{value} (0x{value:016x})"),
            CsrConvError::Usize(value) => write!(f, "{value}"),
            CsrConvError::I8(value) => write!(f, "{value} (0x{value:02x})"),
            CsrConvError::I16(value) => write!(f, "{value} (0x{value:04x})"),
            CsrConvError::I32(value) => write!(f, "{value} (0x{value:08x})"),
            CsrConvError::I64(value) => write!(f, "{value} (0x{value:016x})"),
            CsrConvError::Isize(value) => write!(f, "{value}"),
        }
    }
}

#[test]
fn conv_error_impl_display() {
    assert_eq!(
        Csr::try_from(-1_i8).unwrap_err().to_string(),
        format!("invalid {}-bit unsigned immediate: -1 (0xff)", Csr::NBITS)
    );
    assert_eq!(
        Csr::try_from(-1_i16).unwrap_err().to_string(),
        format!("invalid {}-bit unsigned immediate: -1 (0xffff)", Csr::NBITS)
    );
    assert_eq!(
        Csr::try_from(-1_i32).unwrap_err().to_string(),
        format!(
            "invalid {}-bit unsigned immediate: -1 (0xffffffff)",
            Csr::NBITS
        )
    );
    assert_eq!(
        Csr::try_from(-1_i64).unwrap_err().to_string(),
        format!(
            "invalid {}-bit unsigned immediate: -1 (0xffffffffffffffff)",
            Csr::NBITS
        )
    );
    assert_eq!(
        Csr::try_from(-1_isize).unwrap_err().to_string(),
        format!("invalid {}-bit unsigned immediate: -1", Csr::NBITS)
    );

    assert_eq!(
        Csr::try_from(0x1000_u16).unwrap_err().to_string(),
        format!(
            "invalid {}-bit unsigned immediate: 4096 (0x1000)",
            Csr::NBITS
        )
    );
    assert_eq!(
        Csr::try_from(0x1000_u16).unwrap_err().to_string(),
        format!(
            "invalid {}-bit unsigned immediate: 4096 (0x1000)",
            Csr::NBITS
        )
    );
    assert_eq!(
        Csr::try_from(0x1000_u32).unwrap_err().to_string(),
        format!(
            "invalid {}-bit unsigned immediate: 4096 (0x00001000)",
            Csr::NBITS
        )
    );
    assert_eq!(
        Csr::try_from(0x1000_u64).unwrap_err().to_string(),
        format!(
            "invalid {}-bit unsigned immediate: 4096 (0x0000000000001000)",
            Csr::NBITS
        )
    );
    assert_eq!(
        Csr::try_from(0x1000_usize).unwrap_err().to_string(),
        format!("invalid {}-bit unsigned immediate: 4096", Csr::NBITS)
    );
}

impl Error for CsrConvError {}

#[test]
fn conv_error_impl_error() -> Result<(), Box<dyn Error>> {
    assert_eq!(Csr::try_from(0u8)?, Csr(0));
    Ok(())
}

mod internal {
    pub enum Assert<const CHECK: bool> {}
    pub trait Fits12BIts {}
    impl Fits12BIts for Assert<true> {}
}
