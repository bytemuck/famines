use famines::{Cartridge, CPU, Memory, OPERATIONS, AddressMode};

fn main() {
    let bytes: Vec<u8> = std::fs::read("nestest.nes").unwrap();
    let cartridge = Cartridge::new(&bytes).unwrap();

    let mut cpu = CPU::new(cartridge);
    cpu.reset();
    cpu.pc = 0xC000;

    cpu.run_callback(move |cpu| {
        println!("{}", trace(cpu));
    });
}

pub fn trace(cpu: &CPU) -> String {
    let code = cpu.bus.read_byte(cpu.pc);
    let op = &OPERATIONS[code as usize];

    let begin = cpu.pc;
    let mut hex_dump = vec![code];

    let (mem_addr, stored_value) = match op.mode {
        AddressMode::Immediate | AddressMode::None | AddressMode::Implied => (0, 0),
        _ => {
            let addr = cpu.get_absolute_address(&op.mode, begin + 1);
            (addr, cpu.bus.read_byte(addr))
        }
    };

    let tmp = match op.length {
        1 => match op.code {
            0x0a | 0x4a | 0x2a | 0x6a => "A ".to_string(),
            _ => String::from(""),
        },
        2 => {
            let address: u8 = cpu.bus.read_byte(begin + 1);
            hex_dump.push(address);

            match op.mode {
                AddressMode::Immediate => format!("#${:02x}", address),
                AddressMode::ZeroPage => format!("${:02x} = {:02x}", mem_addr, stored_value),
                AddressMode::ZeroPageX => format!(
                    "${:02x},X @ {:02x} = {:02x}",
                    address, mem_addr, stored_value
                ),
                AddressMode::ZeroPageY => format!(
                    "${:02x},Y @ {:02x} = {:02x}",
                    address, mem_addr, stored_value
                ),
                AddressMode::IndirectX => format!(
                    "(${:02x},X) @ {:02x} = {:04x} = {:02x}",
                    address,
                    (address.wrapping_add(cpu.x)),
                    mem_addr,
                    stored_value
                ),
                AddressMode::IndirectY => format!(
                    "(${:02x}),Y = {:04x} @ {:04x} = {:02x}",
                    address,
                    (mem_addr.wrapping_sub(cpu.y as u16)),
                    mem_addr,
                    stored_value
                ),
                AddressMode::None | AddressMode::Implied => {
                    // assuming local jumps: BNE, BVS, etc....
                    let address: usize =
                        (begin as usize + 2).wrapping_add((address as i8) as usize);
                    format!("${:04x}", address)
                }

                _ => panic!(
                    "unexpected addressing mode {:?} has ops-len 2. code {:02x}",
                    op.mode, op.code
                ),
            }
        }
        3 => {
            let address_lo = cpu.bus.read_byte(begin + 1);
            let address_hi = cpu.bus.read_byte(begin + 2);
            hex_dump.push(address_lo);
            hex_dump.push(address_hi);

            let address = cpu.bus.read_word(begin + 1);

            match op.mode {
                AddressMode::None | AddressMode::Implied  => {
                    if op.code == 0x6c {
                        let jmp_addr = if address & 0x00FF == 0x00FF {
                            let lo = cpu.bus.read_byte(address);
                            let hi = cpu.bus.read_byte(address & 0xFF00);
                            (hi as u16) << 8 | (lo as u16)
                        } else {
                            cpu.bus.read_word(address)
                        };

                        format!("(${:04x}) = {:04x}", address, jmp_addr)
                    } else {
                        format!("${:04x}", address)
                    }
                }
                AddressMode::Absolute => format!("${:04x} = {:02x}", mem_addr, stored_value),
                AddressMode::AbsoluteX => format!(
                    "${:04x},X @ {:04x} = {:02x}",
                    address, mem_addr, stored_value
                ),
                AddressMode::AbsoluteY => format!(
                    "${:04x},Y @ {:04x} = {:02x}",
                    address, mem_addr, stored_value
                ),
                _ => panic!(
                    "unexpected addressing mode {:?} has ops-len 3. code {:02x}",
                    op.mode, op.code
                ),
            }
        }
        _ => String::from(""),
    };

    let hex_str = hex_dump
        .iter()
        .map(|z| format!("{:02x}", z))
        .collect::<Vec<String>>()
        .join(" ");
    let asm_str = format!("{:04x}  {:8} {: >4} {}", begin, hex_str, op.name, tmp)
        .trim()
        .to_string();

    format!(
        "{:47} A:{:02x} X:{:02x} Y:{:02x} P:{:02x} SP:{:02x}",
        asm_str, cpu.a, cpu.x, cpu.y, cpu.status, cpu.sp
    )
    .to_ascii_uppercase()
}