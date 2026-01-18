use std::fs;

use riscv_rust::{Cpu, CpuError};

fn simulate_from_file(path: &str) -> Cpu {
    let instructions: Vec<String> = fs::read_to_string(path).expect("read success").split_whitespace().map(&str::to_owned).collect();
    let mut cpu = Cpu::new(instructions);
    loop {
        if let Err(cpu_error) = cpu.execute() {
            if cpu_error != CpuError::PCOutOfBounds {
                println!("runtime error: {}", cpu_error);
            }
            break;
        }
    }

    cpu
}

#[test]
fn arith() {
    let cpu = simulate_from_file("./tests/programs_samples/arith.dat");
    let regs = cpu.registers();
    assert_eq!(regs[8], 7);
    assert_eq!(regs[9], 8);
    assert_eq!(regs[18], 15);
    assert_eq!(regs[19], 1);
    assert_eq!(regs[20], 0);
    assert_eq!(regs[21], 15);
    assert_eq!(regs[22], 0);
}

#[test]
fn memory() {
    let cpu = simulate_from_file("./tests/programs_samples/memory.dat");
    let regs = cpu.registers();
    assert_eq!(regs[5], 2047);
    assert_eq!(regs[6], 2047);
    assert_eq!(regs[29], 511);
    let mem = cpu.memory();
    assert_eq!(mem[0], 2047);
    assert_eq!(mem[32], 511);
}

#[test]
fn beq() {
    let cpu = simulate_from_file("./tests/programs_samples/beq.dat");
    let regs = cpu.registers();
    assert_eq!(regs[5], 16);
    assert_eq!(regs[8], 8);
    assert_eq!(regs[9], 8);
}

#[test]
fn goto() {
    let cpu = simulate_from_file("./tests/programs_samples/goto.dat");
    let regs = cpu.registers();
    assert_eq!(regs[8], 4);
    assert_eq!(regs[9], 16);
    assert_eq!(regs[18], 8);
}

#[test]
fn bne1() {
    let cpu = simulate_from_file("./tests/programs_samples/bne1.dat");
    let regs = cpu.registers();
    assert_eq!(regs[5], 1);
    assert_eq!(regs[8], 9);
    assert_eq!(regs[9], 8);
}

#[test]
fn bne2() {
    let cpu = simulate_from_file("./tests/programs_samples/bne2.dat");
    let regs = cpu.registers();
    assert_eq!(regs[6], 16);
    assert_eq!(regs[8], 8);
    assert_eq!(regs[9], 8);
}
