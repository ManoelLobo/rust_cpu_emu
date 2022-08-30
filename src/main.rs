struct CPU {
    registers: [u8; 16],
    position_in_memory: usize,
    memory: [u8; 0x1000],
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        let b1 = self.memory[p] as u16;
        let b2 = self.memory[p + 1] as u16;

        b1 << 8 | b2
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.position_in_memory += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = (opcode & 0x000F) as u8;

            match (c, x, y, d) {
                (0, 0, 0, 0) => {
                    return;
                }
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("need to implement opcode {:04x}", opcode),
            }
        }
    }
}

fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 15;
    cpu.registers[3] = 20;

    let mem = &mut cpu.memory;
    mem[0] = 0x80; // adds to register 0 ...
    mem[1] = 0x14; // ... from register 1
    mem[2] = 0x80; // adds to register 0 ...
    mem[3] = 0x24; // ... from register 2
    mem[4] = 0x80; // adds to register 0 ...
    mem[5] = 0x34; // ... from register 3

    cpu.run();

    assert_eq!(cpu.registers[0], 50);

    println!("5+10+15+20={}", cpu.registers[0]);
}
