use super::*;

#[test]
fn create_cpu() {
    let mut _cpu: Cpu = Cpu::new();
}

#[test]
fn assemble_program() {
    let program: &str = "LDA #$c0
             BRK";

    let bytes: Vec<u8> = assemble(&program);

    println!("{:?}", bytes);

    assert_eq!(bytes, [169, 192, 0]);
}

#[test]
fn load_cpu() {
    let mut cpu: Cpu = Cpu::new();

    let program: &str = "LDA #$c0
             BRK";

    let bytes: Vec<u8> = assemble(&program);

    cpu.load(0x800, &bytes);
}
