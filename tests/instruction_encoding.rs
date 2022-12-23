use risky::{
    common::{bimm::BImm, csr::Csr, fence_mask::FenceMask, imm12::Imm12, jimm::JImm, uimm5::Uimm5},
    instruction::{CsrImm, CsrReg, IShift, Instruction, B, I, J, R, S, U},
    m_ext::*,
    registers::{Register, X29, X30, X31},
    rv32i::*,
    zicsr_ext::*,
};

#[test]
fn encoding() {
    test_u(Instruction::Lui, lui);
    test_u(Instruction::Auipc, auipc);
    test_j(Instruction::Jal, jal);
    test_i(Instruction::Jalr, jalr);
    test_b(Instruction::Beq, beq);
    test_b(Instruction::Bne, bne);
    test_b(Instruction::Blt, blt);
    test_b(Instruction::Bltu, bltu);
    test_b(Instruction::Bge, bge);
    test_b(Instruction::Bgeu, bgeu);
    test_i(Instruction::Lb, lb);
    test_i(Instruction::Lbu, lbu);
    test_i(Instruction::Lh, lh);
    test_i(Instruction::Lhu, lhu);
    test_i(Instruction::Lw, lw);
    test_s(Instruction::Sb, sb);
    test_s(Instruction::Sh, sh);
    test_s(Instruction::Sw, sw);
    test_i(Instruction::Addi, addi);
    test_i(Instruction::Slti, slti);
    test_i(Instruction::Sltiu, sltiu);
    test_i(Instruction::Xori, xori);
    test_i(Instruction::Ori, ori);
    test_i(Instruction::Andi, andi);
    test_shift(Instruction::Slli, slli);
    test_shift(Instruction::Srli, srli);
    test_shift(Instruction::Srai, srai);
    test_r(Instruction::Add, add);
    test_r(Instruction::Sub, sub);
    test_r(Instruction::Sll, sll);
    test_r(Instruction::Srl, srl);
    test_r(Instruction::Sra, sra);
    test_r(Instruction::Slt, slt);
    test_r(Instruction::Sltu, sltu);
    test_r(Instruction::Xor, xor);
    test_r(Instruction::Or, or);
    test_r(Instruction::And, and);
    test_fence();
    assert_eq!(Instruction::FenceTso.encode(), fence_tso());
    assert_eq!(Instruction::Ecall.encode(), ecall());
    assert_eq!(Instruction::Ebreak.encode(), ebreak());
    test_r(Instruction::Mul, mul);
    test_r(Instruction::Mulh, mulh);
    test_r(Instruction::Mulhsu, mulhsu);
    test_r(Instruction::Mulhu, mulhu);
    test_r(Instruction::Div, div);
    test_r(Instruction::Divu, divu);
    test_r(Instruction::Rem, rem);
    test_r(Instruction::Remu, remu);
    test_c_reg(Instruction::Csrrw, csrrw);
    test_c_reg(Instruction::Csrrs, csrrs);
    test_c_reg(Instruction::Csrrc, csrrc);
    test_c_imm(Instruction::Csrrwi, csrrwi);
    test_c_imm(Instruction::Csrrsi, csrrsi);
    test_c_imm(Instruction::Csrrci, csrrci);
}

fn test_u(variant: impl Fn(U) -> Instruction, encode: impl Fn(Register, i32) -> u32) {
    let mut u = U {
        rd: X31,
        imm: i32::MIN,
    };
    assert_eq!(variant(u).encode(), encode(u.rd, u.imm));

    u.imm = i32::MAX;
    assert_eq!(variant(u).encode(), encode(u.rd, u.imm));
}

fn test_j(variant: impl Fn(J) -> Instruction, encode: impl Fn(Register, JImm) -> u32) {
    let mut j = J {
        rd: X31,
        imm: JImm::MIN,
    };
    assert_eq!(variant(j).encode(), encode(j.rd, j.imm));

    j.imm = JImm::MAX;
    assert_eq!(variant(j).encode(), encode(j.rd, j.imm));
}

fn test_i(variant: impl Fn(I) -> Instruction, encode: impl Fn(Register, Register, Imm12) -> u32) {
    let mut i = I {
        rd: X30,
        rs1: X31,
        imm: Imm12::MIN,
    };
    assert_eq!(variant(i).encode(), encode(i.rd, i.rs1, i.imm));

    i.imm = Imm12::MAX;
    assert_eq!(variant(i).encode(), encode(i.rd, i.rs1, i.imm));
}

fn test_b(variant: impl Fn(B) -> Instruction, encode: impl Fn(BImm, Register, Register) -> u32) {
    let mut b = B {
        imm: BImm::MIN,
        rs1: X30,
        rs2: X31,
    };
    assert_eq!(variant(b).encode(), encode(b.imm, b.rs1, b.rs2));

    b.imm = BImm::MAX;
    assert_eq!(variant(b).encode(), encode(b.imm, b.rs1, b.rs2));
}

fn test_s(variant: impl Fn(S) -> Instruction, encode: impl Fn(Register, Imm12, Register) -> u32) {
    let mut s = S {
        rs1: X30,
        imm: Imm12::MIN,
        rs2: X31,
    };
    assert_eq!(variant(s).encode(), encode(s.rs1, s.imm, s.rs2));

    s.imm = Imm12::MAX;
    assert_eq!(variant(s).encode(), encode(s.rs1, s.imm, s.rs2));
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
    assert_eq!(variant(i).encode(), encode(i.rd, i.rs1, i.shamt));

    i.shamt = Uimm5::MAX;
    assert_eq!(variant(i).encode(), encode(i.rd, i.rs1, i.shamt));
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
    assert_eq!(variant(r).encode(), encode(r.rd, r.rs1, r.rs2));
}

fn test_fence() {
    let pred = FenceMask::try_from("").unwrap();
    let succ = FenceMask::try_from("wroi").unwrap();
    assert_eq!(
        Instruction::Fence { pred, succ }.encode(),
        fence(pred, succ)
    );
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
    assert_eq!(variant(c).encode(), encode(c.rd, c.rs1, c.csr));
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
    assert_eq!(variant(c).encode(), encode(c.rd, c.rs1, c.csr));
}
