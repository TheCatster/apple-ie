use super::*;
use anyhow::Result;

#[test]
fn create_cpu() {
    let mut _cpu: Cpu = Cpu::new();
}

#[test]
fn assemble_program() -> Result<()> {
    let program: &str = "LDA #$c0
             BRK";

    let bytes: Vec<u8> = assemble(program)?;

    println!("{:?}", bytes);

    assert_eq!(bytes, [169, 192, 0]);
    Ok(())
}

#[test]
fn load_cpu() -> Result<()> {
    let mut cpu: Cpu = Cpu::new();

    let program: &str = "LDA #$c0
             BRK";

    let bytes: Vec<u8> = assemble(program)?;

    cpu.load(0x800, &bytes);
    Ok(())
}
