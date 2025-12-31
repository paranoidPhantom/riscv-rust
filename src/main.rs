use riscv_rust::Cpu;

fn main() {
    let instructions: Vec<String> = vec![
        "00000000100000000000010000010011".to_string(),
        "00000000100000000000010010010011".to_string(),
        "00000000100101000000011001100011".to_string(),
        "00000000100101000000001100110011".to_string(),
        "00000000000000000000010001100011".to_string(),
        "00000000100101000000001010110011".to_string(),
    ];
    let mut cpu = Cpu::new(instructions);

    while cpu.execute().is_ok() {
        dbg!(cpu.registers());
    }
}
