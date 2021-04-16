use crate::*;

pub(crate) fn adc(result: AddrFuncResult, cpu: &mut Processor) {
    match result {
        AddrFuncResult::Immediate(v) => {
            let before = (v + cpu.registers.a + cpu.registers.get_carry() as Byte) as Word;

            cpu.registers.a = (before & 0xFF) as Byte;

            cpu.registers
                .set_negative(before as Byte & FLAG_NEGATIVE > 0);
            cpu.registers.set_overflow_a(before as Byte, v);
            cpu.registers.set_zero(before == 0);
            cpu.registers.set_carry(before > 0xFF);
        }
        AddrFuncResult::Address(addr) => {
            let v = cpu.read_byte(addr);
            let before = v as Word + cpu.registers.a as Word + cpu.registers.get_carry() as Word;
            cpu.registers.set_carry(before > 0xFF);

            cpu.registers.a = (before & 0xFF) as Byte;

            cpu.registers
                .set_negative(before as Byte & FLAG_NEGATIVE > 0);
            cpu.registers.set_overflow_a(before as Byte, v);
            cpu.registers.set_zero(before == 0);
        }
        _ => {}
    }
}

pub(crate) fn and(result: AddrFuncResult, cpu: &mut Processor) {
    match result {
        AddrFuncResult::Immediate(v) => {
            cpu.registers.a &= v;

            cpu.registers.set_negative_a();
            cpu.registers.set_zero_a();
        }
        AddrFuncResult::Address(addr) => {
            let v = cpu.read_byte(addr);
            cpu.registers.a &= v;

            cpu.registers.set_negative_a();
            cpu.registers.set_zero_a();
        }
        _ => {}
    }
}

pub(crate) fn asl(result: AddrFuncResult, cpu: &mut Processor) {
    match result {
        AddrFuncResult::Implied => {
            let mut before = cpu.registers.a;
            cpu.registers.set_carry(before & FLAG_NEGATIVE > 0);

            before <<= 1;
            before &= 0xFF;
            cpu.cycles += 1;

            cpu.registers.set_negative(before & FLAG_NEGATIVE > 0);
            cpu.registers.set_zero(before == 0);

            cpu.registers.a = before;
        }
        AddrFuncResult::Address(addr) => {
            let mut value = cpu.read_byte(addr);
            cpu.registers.set_carry(value & FLAG_NEGATIVE > 0);

            value <<= 1;
            value &= 0xFF;
            cpu.cycles += 1;

            cpu.write_byte(value, addr);

            cpu.registers.set_negative(value & FLAG_NEGATIVE > 0);
            cpu.registers.set_zero(value == 0);
        }
        _ => {}
    }
}

pub(crate) fn bcc(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Relative(addr) = result {
        cpu.branch_if(!cpu.registers.get_carry(), addr);
    }
}

pub(crate) fn bcs(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Relative(addr) = result {
        cpu.branch_if(cpu.registers.get_carry(), addr);
    }
}

pub(crate) fn beq(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Relative(addr) = result {
        cpu.branch_if(cpu.registers.get_zero(), addr);
    }
}

pub(crate) fn bit(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Address(addr) = result {
        let result = cpu.read_byte(addr);

        let anded = cpu.registers.a & result;
        cpu.registers.set_zero(anded == 0);

        cpu.registers.set_negative(result & FLAG_NEGATIVE > 0);
        cpu.registers.set_overflow(result & FLAG_OVERFLOW > 0)
    }
}

pub(crate) fn bmi(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Relative(addr) = result {
        cpu.branch_if(cpu.registers.get_negative(), addr);
    }
}

pub(crate) fn bne(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Relative(addr) = result {
        cpu.branch_if(!cpu.registers.get_zero(), addr);
    }
}

pub(crate) fn bpl(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Relative(addr) = result {
        cpu.branch_if(!cpu.registers.get_negative(), addr);
    }
}

/* still not sure how to use break and interrupts, I feel like I need to read a little bit. */
pub(crate) fn brk(_: AddrFuncResult, cpu: &mut Processor) {
    cpu.push_pc_plus_one_to_stack();
    cpu.push_status_to_stack();
    let interrupt_vector = Address(BREAK_AND_INTERRUPT_REQUEST_HANDLER_BOTTOM);
    cpu.registers.pc = Address(cpu.read_word(interrupt_vector));
    cpu.registers.set_interrupt(true);
    //cpu.registers.set_break(true); // idk why, but every reference i looked at set either both, interrupt or break idk what to do lmao
}

pub(crate) fn bvc(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Relative(addr) = result {
        cpu.branch_if(!cpu.registers.get_overflow(), addr);
    }
}

pub(crate) fn bvs(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Relative(addr) = result {
        cpu.branch_if(cpu.registers.get_overflow(), addr);
    }
}

pub(crate) fn clc(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Implied = result {
        cpu.registers.set_carry(false);
        cpu.cycles += 1;
    }
}

pub(crate) fn cld(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Implied = result {
        cpu.registers.set_decimal(false);
        cpu.cycles += 1;
    }
}

pub(crate) fn cli(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Implied = result {
        cpu.registers.set_interrupt(false);
        cpu.cycles += 1;
    }
}

pub(crate) fn clv(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Implied = result {
        cpu.registers.set_overflow(false);
        cpu.cycles += 1;
    }
}

pub(crate) fn cmp(result: AddrFuncResult, cpu: &mut Processor) {
    match result {
        AddrFuncResult::Immediate(v) => {
            let temp = cpu.registers.a - v;
            cpu.registers.set_negative(temp & FLAG_NEGATIVE > 0);
            cpu.registers.set_zero(cpu.registers.a == v);
            cpu.registers.set_carry(cpu.registers.a >= v);
        }
        AddrFuncResult::Address(addr) => {
            let v = cpu.read_byte(addr);
            let temp = cpu.registers.a - v;
            cpu.registers.set_negative(temp & FLAG_NEGATIVE > 0);
            cpu.registers.set_zero(cpu.registers.a == v);
            cpu.registers.set_carry(cpu.registers.a >= v);
        }
        _ => {}
    }
}

pub(crate) fn cpx(result: AddrFuncResult, cpu: &mut Processor) {
    match result {
        AddrFuncResult::Immediate(v) => {
            let temp = cpu.registers.x - v;
            cpu.registers.set_negative(temp & FLAG_NEGATIVE > 0);
            cpu.registers.set_zero(cpu.registers.x == v);
            cpu.registers.set_carry(cpu.registers.x >= v);
        }
        AddrFuncResult::Address(addr) => {
            let v = cpu.read_byte(addr);
            let temp = cpu.registers.x - v;
            cpu.registers.set_negative(temp & FLAG_NEGATIVE > 0);
            cpu.registers.set_zero(cpu.registers.x == v);
            cpu.registers.set_carry(cpu.registers.x >= v);
        }
        _ => {}
    }
}

pub(crate) fn cpy(result: AddrFuncResult, cpu: &mut Processor) {
    match result {
        AddrFuncResult::Immediate(v) => {
            let temp = cpu.registers.y - v;
            cpu.registers.set_negative(temp & FLAG_NEGATIVE > 0);
            cpu.registers.set_zero(cpu.registers.y == v);
            cpu.registers.set_carry(cpu.registers.y >= v);
        }
        AddrFuncResult::Address(addr) => {
            let v = cpu.read_byte(addr);
            let temp = cpu.registers.y - v;
            cpu.registers.set_negative(temp & FLAG_NEGATIVE > 0);
            cpu.registers.set_zero(cpu.registers.y == v);
            cpu.registers.set_carry(cpu.registers.y >= v);
        }
        _ => {}
    }
}

pub(crate) fn dec(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Address(addr) = result {
        let mut result = cpu.read_byte(addr);
        result -= 0x01;
        cpu.cycles += 1;

        cpu.write_byte(result, addr);

        cpu.registers.set_negative(result & FLAG_NEGATIVE > 0);
        cpu.registers.set_zero(result == 0);
    }
}

pub(crate) fn dex(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Implied = result {
        cpu.registers.x -= 0x01;
        cpu.cycles += 1;

        cpu.registers
            .set_negative(dbg!(cpu.registers.x) & FLAG_NEGATIVE > 0);
        cpu.registers.set_zero(cpu.registers.x == 0);
    }
}

pub(crate) fn dey(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Implied = result {
        cpu.registers.y -= 0x01;
        cpu.cycles += 1;

        cpu.registers
            .set_negative(cpu.registers.y & FLAG_NEGATIVE > 0);
        cpu.registers.set_zero(cpu.registers.y == 0);
    }
}

pub(crate) fn eor(result: AddrFuncResult, cpu: &mut Processor) {
    match result {
        AddrFuncResult::Immediate(v) => {
            cpu.registers.a ^= v;

            cpu.registers
                .set_negative(cpu.registers.a & FLAG_NEGATIVE > 0);
            cpu.registers.set_zero(cpu.registers.a == 0);
        }
        AddrFuncResult::Address(addr) => {
            cpu.registers.a ^= cpu.read_byte(addr);
            dbg!(cpu.cycles);
            cpu.registers
                .set_negative(cpu.registers.a & FLAG_NEGATIVE > 0);
            cpu.registers.set_zero(cpu.registers.a == 0);
        }
        _ => {}
    }
}

pub(crate) fn inc(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Address(addr) = result {
        let mut result = cpu.read_byte(addr);
        result += 0x01;
        cpu.cycles += 1;

        cpu.write_byte(result, addr);

        cpu.registers.set_negative(result & FLAG_NEGATIVE > 0);
        cpu.registers.set_zero(result == 0);
    }
}

pub(crate) fn inx(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Implied = result {
        cpu.registers.x += 0x01;
        cpu.cycles += 1;

        cpu.registers
            .set_negative(cpu.registers.x & FLAG_NEGATIVE > 0);
        cpu.registers.set_zero(cpu.registers.x == 0);
    }
}

pub(crate) fn iny(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Implied = result {
        cpu.registers.y += 0x01;
        cpu.cycles += 1;

        cpu.registers
            .set_negative(cpu.registers.y & FLAG_NEGATIVE > 0);
        cpu.registers.set_zero(cpu.registers.y == 0);
    }
}

pub(crate) fn jsr(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Address(addr) = result {
        cpu.write_word(
            (cpu.registers.pc + RelativeAddress(-1)).to_word(),
            Address(cpu.registers.sp.into()),
        );
        cpu.registers.sp -= 2;
        cpu.registers.pc = addr;
        cpu.cycles += 1;
    }
}

pub(crate) fn lda(result: AddrFuncResult, cpu: &mut Processor) {
    match result {
        AddrFuncResult::Immediate(v) => {
            cpu.registers.a = v;

            cpu.registers.set_negative_a();
            cpu.registers.set_zero_a();
        }
        AddrFuncResult::Address(addr) => {
            let v = cpu.read_byte(addr);
            cpu.registers.a = v;

            cpu.registers.set_negative_a();
            cpu.registers.set_zero_a();
        }
        _ => {}
    }
}

pub(crate) fn ldx(result: AddrFuncResult, cpu: &mut Processor) {
    match result {
        AddrFuncResult::Immediate(v) => {
            cpu.registers.x = v;

            cpu.registers.set_negative_x();
            cpu.registers.set_zero_x();
        }
        AddrFuncResult::Address(addr) => {
            let v = cpu.read_byte(addr);
            cpu.registers.x = v;

            cpu.registers.set_negative_x();
            cpu.registers.set_zero_x();
        }
        _ => {}
    }
}

pub(crate) fn ldy(result: AddrFuncResult, cpu: &mut Processor) {
    match result {
        AddrFuncResult::Immediate(v) => {
            cpu.registers.y = v;

            cpu.registers.set_negative_y();
            cpu.registers.set_zero_y();
        }
        AddrFuncResult::Address(addr) => {
            let v = cpu.read_byte(addr);
            cpu.registers.y = v;

            cpu.registers.set_negative_y();
            cpu.registers.set_zero_y();
        }
        _ => {}
    }
}
