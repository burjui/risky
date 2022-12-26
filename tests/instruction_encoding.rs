use risky::{
    common::{bimm::BImm, csr::Csr, fence_mask::FenceMask, imm12::Imm12, jimm::JImm, uimm5::Uimm5},
    instruction::{
        CsrImm, CsrReg, EncodedInstruction::Standard, IShift, Instruction, B, I, J, R, S, U,
    },
    m_ext::*,
    registers::{Register, X29, X30, X31},
    rv32i::*,
    zicsr_ext::*,
};

#[test]
fn _lui() {
    test_u(Instruction::Lui, lui);
}

#[test]
fn _auipc() {
    test_u(Instruction::Auipc, auipc);
}

#[test]
fn _jal() {
    test_j(Instruction::Jal, jal);
}

#[test]
fn _jalr() {
    test_i(Instruction::Jalr, jalr);
}

#[test]
fn _beq() {
    test_b(Instruction::Beq, beq);
}

#[test]
fn _bne() {
    test_b(Instruction::Bne, bne);
}

#[test]
fn _blt() {
    test_b(Instruction::Blt, blt);
}

#[test]
fn _bltu() {
    test_b(Instruction::Bltu, bltu);
}

#[test]
fn _bge() {
    test_b(Instruction::Bge, bge);
}

#[test]
fn _bgeu() {
    test_b(Instruction::Bgeu, bgeu);
}

#[test]
fn _lb() {
    test_i(Instruction::Lb, lb);
}

#[test]
fn _lbu() {
    test_i(Instruction::Lbu, lbu);
}

#[test]
fn _lh() {
    test_i(Instruction::Lh, lh);
}

#[test]
fn _lhu() {
    test_i(Instruction::Lhu, lhu);
}

#[test]
fn _lw() {
    test_i(Instruction::Lw, lw);
}

#[test]
fn _sb() {
    test_s(Instruction::Sb, sb);
}

#[test]
fn _sh() {
    test_s(Instruction::Sh, sh);
}

#[test]
fn _sw() {
    test_s(Instruction::Sw, sw);
}

#[test]
fn _addi() {
    test_i(Instruction::Addi, addi);
}

#[test]
fn _slti() {
    test_i(Instruction::Slti, slti);
}

#[test]
fn _sltiu() {
    test_i(Instruction::Sltiu, sltiu);
}

#[test]
fn _xori() {
    test_i(Instruction::Xori, xori);
}

#[test]
fn _ori() {
    test_i(Instruction::Ori, ori);
}

#[test]
fn _andi() {
    test_i(Instruction::Andi, andi);
}

#[test]
fn _slli() {
    test_shift(Instruction::Slli, slli);
}

#[test]
fn _srli() {
    test_shift(Instruction::Srli, srli);
}

#[test]
fn _srai() {
    test_shift(Instruction::Srai, srai);
}

#[test]
fn _add() {
    test_r(Instruction::Add, add);
}

#[test]
fn _sub() {
    test_r(Instruction::Sub, sub);
}

#[test]
fn _sll() {
    test_r(Instruction::Sll, sll);
}

#[test]
fn _srl() {
    test_r(Instruction::Srl, srl);
}

#[test]
fn _sra() {
    test_r(Instruction::Sra, sra);
}

#[test]
fn _slt() {
    test_r(Instruction::Slt, slt);
}

#[test]
fn _sltu() {
    test_r(Instruction::Sltu, sltu);
}

#[test]
fn _xor() {
    test_r(Instruction::Xor, xor);
}

#[test]
fn _or() {
    test_r(Instruction::Or, or);
}

#[test]
fn _fence() {
    let pred = FenceMask::try_from("").unwrap();
    let succ = FenceMask::try_from("wroi").unwrap();
    assert_eq!(
        Instruction::Fence { pred, succ }.encode(),
        Standard(fence(pred, succ))
    );
}

#[test]
fn _and() {
    test_r(Instruction::And, and);
}

#[test]
fn _fence_tso() {
    assert_eq!(Instruction::FenceTso.encode(), Standard(fence_tso()));
}

#[test]
fn _ecall() {
    assert_eq!(Instruction::Ecall.encode(), Standard(ecall()));
}

#[test]
fn _ebreak() {
    assert_eq!(Instruction::Ebreak.encode(), Standard(ebreak()));
}

#[test]
fn _mul() {
    test_r(Instruction::Mul, mul);
}

#[test]
fn _mulh() {
    test_r(Instruction::Mulh, mulh);
}

#[test]
fn _mulhsu() {
    test_r(Instruction::Mulhsu, mulhsu);
}

#[test]
fn _mulhu() {
    test_r(Instruction::Mulhu, mulhu);
}

#[test]
fn _div() {
    test_r(Instruction::Div, div);
}

#[test]
fn _divu() {
    test_r(Instruction::Divu, divu);
}

#[test]
fn _rem() {
    test_r(Instruction::Rem, rem);
}

#[test]
fn _remu() {
    test_r(Instruction::Remu, remu);
}

#[test]
fn _csrrw() {
    test_c_reg(Instruction::Csrrw, csrrw);
}

#[test]
fn _csrrs() {
    test_c_reg(Instruction::Csrrs, csrrs);
}

#[test]
fn _csrrc() {
    test_c_reg(Instruction::Csrrc, csrrc);
}

#[test]
fn _csrrwi() {
    test_c_imm(Instruction::Csrrwi, csrrwi);
}

#[test]
fn _csrrsi() {
    test_c_imm(Instruction::Csrrsi, csrrsi);
}

#[test]
fn _csrrci() {
    test_c_imm(Instruction::Csrrci, csrrci);
}

fn test_u(variant: impl Fn(U) -> Instruction, encode: impl Fn(Register, i32) -> u32) {
    let mut u = U {
        rd: X31,
        imm: i32::MIN,
    };
    assert_eq!(variant(u).encode(), Standard(encode(u.rd, u.imm)));

    u.imm = i32::MAX;
    assert_eq!(variant(u).encode(), Standard(encode(u.rd, u.imm)));
}

fn test_j(variant: impl Fn(J) -> Instruction, encode: impl Fn(Register, JImm) -> u32) {
    let mut j = J {
        rd: X31,
        imm: JImm::MIN,
    };
    assert_eq!(variant(j).encode(), Standard(encode(j.rd, j.imm)));

    j.imm = JImm::MAX;
    assert_eq!(variant(j).encode(), Standard(encode(j.rd, j.imm)));
}

fn test_i(variant: impl Fn(I) -> Instruction, encode: impl Fn(Register, Register, Imm12) -> u32) {
    let mut i = I {
        rd: X30,
        rs1: X31,
        imm: Imm12::MIN,
    };
    assert_eq!(variant(i).encode(), Standard(encode(i.rd, i.rs1, i.imm)));

    i.imm = Imm12::MAX;
    assert_eq!(variant(i).encode(), Standard(encode(i.rd, i.rs1, i.imm)));
}

fn test_b(variant: impl Fn(B) -> Instruction, encode: impl Fn(BImm, Register, Register) -> u32) {
    let mut b = B {
        imm: BImm::MIN,
        rs1: X30,
        rs2: X31,
    };
    assert_eq!(variant(b).encode(), Standard(encode(b.imm, b.rs1, b.rs2)));

    b.imm = BImm::MAX;
    assert_eq!(variant(b).encode(), Standard(encode(b.imm, b.rs1, b.rs2)));
}

fn test_s(variant: impl Fn(S) -> Instruction, encode: impl Fn(Register, Imm12, Register) -> u32) {
    let mut s = S {
        rs1: X30,
        imm: Imm12::MIN,
        rs2: X31,
    };
    assert_eq!(variant(s).encode(), Standard(encode(s.rs1, s.imm, s.rs2)));

    s.imm = Imm12::MAX;
    assert_eq!(variant(s).encode(), Standard(encode(s.rs1, s.imm, s.rs2)));
}
fn test_shift(
    variant: impl Fn(IShift) -> Instruction,
    encode: impl Fn(Register, Register, Uimm5) -> u32,
) {
    let mut i = IShift {
        rd: X30,
        rs1: X31,
        shamt: Uimm5::MIN,
    };
    assert_eq!(variant(i).encode(), Standard(encode(i.rd, i.rs1, i.shamt)));

    i.shamt = Uimm5::MAX;
    assert_eq!(variant(i).encode(), Standard(encode(i.rd, i.rs1, i.shamt)));
}

fn test_r(
    variant: impl FnOnce(R) -> Instruction,
    encode: impl FnOnce(Register, Register, Register) -> u32,
) {
    let r = R {
        rd: X29,
        rs1: X30,
        rs2: X31,
    };
    assert_eq!(variant(r).encode(), Standard(encode(r.rd, r.rs1, r.rs2)));
}

fn test_c_reg(
    variant: impl FnOnce(CsrReg) -> Instruction,
    encode: impl FnOnce(Register, Register, Csr) -> u32,
) {
    let c = CsrReg {
        rd: X30,
        rs1: X31,
        csr: Csr::FFLAGS,
    };
    assert_eq!(variant(c).encode(), Standard(encode(c.rd, c.rs1, c.csr)));
}

fn test_c_imm(
    variant: impl FnOnce(CsrImm) -> Instruction,
    encode: impl FnOnce(Register, Uimm5, Csr) -> u32,
) {
    let c = CsrImm {
        rd: X30,
        rs1: Uimm5::MAX,
        csr: Csr::FFLAGS,
    };
    assert_eq!(variant(c).encode(), Standard(encode(c.rd, c.rs1, c.csr)));
}
