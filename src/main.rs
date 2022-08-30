struct CPU {
    registers: [u8; 16],
    position_in_memory: usize,
    memory: [u8; 0x1000],
    stack: [u16; 16],
    stack_pointer: usize,
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

    fn call(&mut self, address: u16) {
        let pointer = self.stack_pointer;
        let stack = &mut self.stack;

        if pointer > stack.len() {
            panic!("Stack overflow");
        }

        stack[pointer] = self.position_in_memory as u16;
        self.stack_pointer += 1;
        self.position_in_memory = address as usize;
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow");
        }

        self.stack_pointer -= 1;
        let call_address = self.stack[self.stack_pointer];
        self.position_in_memory = call_address as usize;
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.position_in_memory += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = (opcode & 0x000F) as u8;

            let nnn = opcode & 0x0FFF;

            match (c, x, y, d) {
                (0, 0, 0, 0) => {
                    return;
                }
                (0, 0, 0xE, 0xE) => self.ret(),
                (0x2, _, _, _) => self.call(nnn),
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
        stack: [0; 16],
        stack_pointer: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    let mem = &mut cpu.memory;
    mem[0x000] = 0x21; // (1) Call function at 0x100 (add 10 to 5) ...
    mem[0x001] = 0x00; // ... which will follow to 0x102 (add 10 more)

    mem[0x002] = 0x21; // (5) Will repeat the previous func call, which will add
    mem[0x003] = 0x00; // 10 + 10 to register 0 again

    mem[0x004] = 0x00; // (9) Quit the CPU run
    mem[0x005] = 0x00;

    mem[0x100] = 0x80; // (2) (6) Add 10 to register 0
    mem[0x101] = 0x14;

    mem[0x102] = 0x80; // (3) (7) Add 10 to register 0, in sequence
    mem[0x103] = 0x14;

    mem[0x104] = 0x00; // (4) (8) Return the function call
    mem[0x105] = 0xEE;

    cpu.run();

    assert_eq!(cpu.registers[0], 45);

    println!("5 +(10 + 10) +(10 +10)={}", cpu.registers[0]);
}
