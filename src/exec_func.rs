use crate::*;

pub(crate) fn adc(result: AddrFuncResult, cpu: &mut Processor) {
    match result {
        AddrFuncResult::Immediate(v) => {
            let before = (v + cpu.registers.a + cpu.registers.get_carry() as Byte) as Word;

            cpu.registers.a = (before & 0xFF) as Byte;

            cpu.registers.set_negative(before as Byte);
            cpu.registers.set_overflow_a(v, before as Byte);
            cpu.registers.set_zero(before as Byte);
            cpu.registers.set_carry(before > 0xFF);
        }
        AddrFuncResult::Address(addr) => {
            let v = cpu.read_byte(addr);
            let before = v as Word + cpu.registers.a as Word + cpu.registers.get_carry() as Word;
            cpu.registers.set_carry(before > 0xFF);

            cpu.registers.a = (before & 0xFF) as Byte;

            cpu.registers.set_negative(before as Byte);
            cpu.registers.set_overflow_a(v, before as Byte);
            cpu.registers.set_zero(before as Byte);
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

            cpu.registers.set_negative(before);
            cpu.registers.set_zero(before);

            cpu.registers.a = before;
        }
        AddrFuncResult::Address(addr) => {
            let mut value = cpu.read_byte(addr);
            cpu.registers.set_carry(value & FLAG_NEGATIVE > 0);

            value <<= 1;
            value &= 0xFF;
            cpu.cycles += 1;

            cpu.registers.set_negative(value);
            cpu.registers.set_zero(value);

            cpu.write_byte(value, addr);
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
        cpu.registers.set_zero(anded);

        cpu.registers.set_negative(result);
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

pub(crate) fn inc(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Address(addr) = result {
        let mut result = cpu.read_byte(addr);
        result += 0x01;
        cpu.cycles += 1;

        cpu.write_byte(result, addr);

        cpu.registers.set_negative(result);
        cpu.registers.set_zero(result);
    }
}

pub(crate) fn jsr(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Address(addr) = result {
        cpu.write_word(
            (cpu.registers.pc + AddressDiff(-1)).to_word(),
            Address(cpu.registers.sp.into()),
        );
        cpu.registers.sp -= 2;
        cpu.registers.pc = addr;
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
