use std::error::Error;

use risky::{
    decoding::decode,
    instruction::{Instruction, R},
    registers::{X29, X30, X31},
};

#[test]
fn len() {
    assert_eq!(
        Instruction::Add(R {
            rd: X29,
            rs1: X30,
            rs2: X31,
        })
        .encode()
        .len(),
        4
    );
}

#[test]
fn write() -> Result<(), Box<dyn Error>> {
    let mut buf = Vec::new();
    let expected = Instruction::Add(R {
        rd: X29,
        rs1: X30,
        rs2: X31,
    });
    expected.write(&mut buf)?;

    let raw_instruction = u32::from_le_bytes(buf.try_into().unwrap());
    let instruction = decode(raw_instruction)?;
    assert_eq!(instruction, expected);

    Ok(())
}
