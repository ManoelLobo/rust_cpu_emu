struct CPU {
    current_operation: u16,
    registers: [u8; 2],
}

fn main() {
    let mut cpu = CPU {
        current_operation: 0,
        registers: [0; 2],
    };

    // 8 - operation with 2 registers; 0 - register; 1 - register; 4 - addition
    cpu.current_operation = 0x8014;
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
}
