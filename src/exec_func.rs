use crate::*;

pub(crate) fn adc(result: AddrFuncResult, cpu: &mut Processor) {
    match result {
        AddrFuncResult::Immediate(v) => {
            let sign = !(((cpu.registers.a ^ v) & FLAG_NEGATIVE) > 0);
            let mut sum = cpu.registers.a as u16;
            sum += v as u16;
            sum += cpu.registers.c as u16;

            cpu.registers.a = (sum & 0xFF) as u8;

            cpu.registers.z = cpu.registers.a == 0;
            cpu.registers.n = cpu.registers.a & FLAG_NEGATIVE > 0;
            cpu.registers.c = sum > 0xFF;
            cpu.registers.v = sign && ((cpu.registers.a ^ v) & FLAG_NEGATIVE) > 0;
        }
        AddrFuncResult::Address(addr) => {
            let v = cpu.read_byte(addr);
            let sign = !(((cpu.registers.a ^ v) & FLAG_NEGATIVE) > 0);
            let mut sum = cpu.registers.a as u16;
            sum += v as u16;
            sum += cpu.registers.c as u16;

            cpu.registers.a = (sum & 0xFF) as u8;

            cpu.registers.z = cpu.registers.a == 0;
            cpu.registers.n = cpu.registers.a & FLAG_NEGATIVE > 0;
            cpu.registers.c = sum > 0xFF;
            cpu.registers.v = sign && ((cpu.registers.a ^ v) & FLAG_NEGATIVE) > 0;
        }
        _ => {}
    }
}

pub(crate) fn and(result: AddrFuncResult, cpu: &mut Processor) {
    match result {
        AddrFuncResult::Immediate(v) => {
            cpu.registers.a &= v;

            cpu.registers.z = cpu.registers.a == 0;
            cpu.registers.n = cpu.registers.a & FLAG_NEGATIVE > 0;
        }
        AddrFuncResult::Address(addr) => {
            let v = cpu.read_byte(addr);
            cpu.registers.a &= v;

            cpu.registers.z = cpu.registers.a == 0;
            cpu.registers.n = cpu.registers.a & FLAG_NEGATIVE > 0;
        }
        _ => {}
    }
}

pub(crate) fn asl(result: AddrFuncResult, cpu: &mut Processor) {
    match result {
        AddrFuncResult::Implied => {
            let mut before = cpu.registers.a;
            cpu.registers.c = before & FLAG_NEGATIVE > 0;

            before <<= 1;
            before &= 0xFF;
            cpu.cycles += 1;

            cpu.registers.n = before & FLAG_NEGATIVE > 0;
            cpu.registers.z = before == 0;

            cpu.registers.a = before;
        }
        AddrFuncResult::Address(addr) => {
            let mut value = cpu.read_byte(addr);
            cpu.registers.c = value & FLAG_NEGATIVE > 0;

            value <<= 1;
            value &= 0xFF;
            cpu.cycles += 1;

            cpu.write_byte(value, addr);

            cpu.registers.n = value & FLAG_NEGATIVE > 0;
            cpu.registers.z = value == 0;
        }
        _ => {}
    }
}

pub(crate) fn bcc(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Relative(addr) = result {
        cpu.branch_if(!cpu.registers.c, addr);
    }
}

pub(crate) fn bcs(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Relative(addr) = result {
        cpu.branch_if(cpu.registers.c, addr);
    }
}

pub(crate) fn beq(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Relative(addr) = result {
        cpu.branch_if(cpu.registers.z, addr);
    }
}

pub(crate) fn bit(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Address(addr) = result {
        let result = cpu.read_byte(addr);

        let anded = cpu.registers.a & result;
        cpu.registers.z = anded == 0;

        cpu.registers.n = result & FLAG_NEGATIVE > 0;
        cpu.registers.v = result & FLAG_OVERFLOW > 0
    }
}

pub(crate) fn bmi(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Relative(addr) = result {
        cpu.branch_if(cpu.registers.n, addr);
    }
}

pub(crate) fn bne(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Relative(addr) = result {
        cpu.branch_if(!cpu.registers.z, addr);
    }
}

pub(crate) fn bpl(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Relative(addr) = result {
        cpu.branch_if(!cpu.registers.n, addr);
    }
}

/* still not sure how to use break and interrupts, I feel like I need to read a little bit. */
pub(crate) fn brk(_: AddrFuncResult, cpu: &mut Processor) {
    cpu.push_pc_plus_one_to_stack();
    cpu.push_status_to_stack();
    cpu.registers.pc = cpu.read_word(BREAK_AND_INTERRUPT_REQUEST_HANDLER_BOTTOM);
    cpu.registers.i = true;
    cpu.registers.b = true;
    cpu.registers.unused = true;
}

pub(crate) fn bvc(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Relative(addr) = result {
        cpu.branch_if(!cpu.registers.v, addr);
    }
}

pub(crate) fn bvs(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Relative(addr) = result {
        cpu.branch_if(cpu.registers.v, addr);
    }
}

pub(crate) fn clc(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Implied = result {
        cpu.registers.c = false;
        cpu.cycles += 1;
    }
}

pub(crate) fn cld(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Implied = result {
        cpu.registers.d = false;
        cpu.cycles += 1;
    }
}

pub(crate) fn cli(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Implied = result {
        cpu.registers.i = false;
        cpu.cycles += 1;
    }
}

pub(crate) fn clv(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Implied = result {
        cpu.registers.v = false;
        cpu.cycles += 1;
    }
}

pub(crate) fn cmp(result: AddrFuncResult, cpu: &mut Processor) {
    match result {
        AddrFuncResult::Immediate(v) => {
            let temp = u8::wrapping_sub(cpu.registers.a, v);
            cpu.registers.n = (temp & FLAG_NEGATIVE) > 0;
            cpu.registers.z = cpu.registers.a == v;
            cpu.registers.c = cpu.registers.a >= v;
        }
        AddrFuncResult::Address(addr) => {
            let v = cpu.read_byte(addr);
            let temp = u8::wrapping_sub(cpu.registers.a, v);
            cpu.registers.n = (temp & FLAG_NEGATIVE) > 0;
            cpu.registers.z = cpu.registers.a == v;
            cpu.registers.c = cpu.registers.a >= v;
        }
        _ => {}
    }
}

pub(crate) fn cpx(result: AddrFuncResult, cpu: &mut Processor) {
    match result {
        AddrFuncResult::Immediate(v) => {
            let temp = cpu.registers.x.wrapping_sub(v);
            cpu.registers.n = temp & FLAG_NEGATIVE > 0;
            cpu.registers.z = cpu.registers.x == v;
            cpu.registers.c = cpu.registers.x >= v;
        }
        AddrFuncResult::Address(addr) => {
            let v = cpu.read_byte(addr);
            let temp = cpu.registers.x.wrapping_sub(v);
            cpu.registers.n = temp & FLAG_NEGATIVE > 0;
            cpu.registers.z = cpu.registers.x == v;
            cpu.registers.c = cpu.registers.x >= v;
        }
        _ => {}
    }
}

pub(crate) fn cpy(result: AddrFuncResult, cpu: &mut Processor) {
    match result {
        AddrFuncResult::Immediate(v) => {
            let temp = cpu.registers.y.wrapping_sub(v);
            cpu.registers.n = temp & FLAG_NEGATIVE > 0;
            cpu.registers.z = cpu.registers.y == v;
            cpu.registers.c = cpu.registers.y >= v;
        }
        AddrFuncResult::Address(addr) => {
            let v = cpu.read_byte(addr);
            let temp = cpu.registers.y.wrapping_sub(v);
            cpu.registers.n = temp & FLAG_NEGATIVE > 0;
            cpu.registers.z = cpu.registers.y == v;
            cpu.registers.c = cpu.registers.y >= v;
        }
        _ => {}
    }
}

pub(crate) fn dec(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Address(addr) = result {
        let mut result = cpu.read_byte(addr);
        result = result.wrapping_sub(0x01);
        cpu.cycles += 1;

        cpu.write_byte(result, addr);

        cpu.registers.n = result & FLAG_NEGATIVE > 0;
        cpu.registers.z = result == 0;
    }
}

pub(crate) fn dex(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Implied = result {
        cpu.registers.x = cpu.registers.x.wrapping_sub(0x01);
        cpu.cycles += 1;

        cpu.registers.n = cpu.registers.x & FLAG_NEGATIVE > 0;
        cpu.registers.z = cpu.registers.x == 0;
    }
}

pub(crate) fn dey(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Implied = result {
        cpu.registers.y = cpu.registers.y.wrapping_sub(0x01);
        cpu.cycles += 1;

        cpu.registers.n = cpu.registers.y & FLAG_NEGATIVE > 0;
        cpu.registers.z = cpu.registers.y == 0;
    }
}

pub(crate) fn eor(result: AddrFuncResult, cpu: &mut Processor) {
    match result {
        AddrFuncResult::Immediate(v) => {
            cpu.registers.a ^= v;

            cpu.registers.z = cpu.registers.a == 0;
            cpu.registers.n = cpu.registers.a & FLAG_NEGATIVE > 0;
        }
        AddrFuncResult::Address(addr) => {
            cpu.registers.a ^= cpu.read_byte(addr);

            cpu.registers.z = cpu.registers.a == 0;
            cpu.registers.n = cpu.registers.a & FLAG_NEGATIVE > 0;
        }
        _ => {}
    }
}

pub(crate) fn inc(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Address(addr) = result {
        let mut result = cpu.read_byte(addr);
        result = result.wrapping_add(0x01);
        cpu.cycles += 1;

        cpu.write_byte(result, addr);

        cpu.registers.n = result & FLAG_NEGATIVE > 0;
        cpu.registers.z = result == 0;
    }
}

pub(crate) fn inx(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Implied = result {
        cpu.registers.x = cpu.registers.x.wrapping_add(0x01);
        cpu.cycles += 1;

        cpu.registers.n = cpu.registers.x & FLAG_NEGATIVE > 0;
        cpu.registers.z = cpu.registers.x == 0;
    }
}

pub(crate) fn iny(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Implied = result {
        cpu.registers.y = cpu.registers.y.wrapping_add(0x01);
        cpu.cycles += 1;

        cpu.registers.n = cpu.registers.y & FLAG_NEGATIVE > 0;
        cpu.registers.z = cpu.registers.y == 0;
    }
}

pub(crate) fn jmp(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Address(addr) = result {
        cpu.registers.pc = addr;
    }
}

pub(crate) fn jsr(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Address(addr) = result {
        cpu.push_pc_minus_one_to_stack();

        cpu.registers.pc = addr;
        cpu.cycles += 1;
    }
}

pub(crate) fn lda(result: AddrFuncResult, cpu: &mut Processor) {
    match result {
        AddrFuncResult::Immediate(v) => {
            cpu.registers.a = v;

            cpu.registers.z = cpu.registers.a == 0;
            cpu.registers.n = cpu.registers.a & FLAG_NEGATIVE > 0;
        }
        AddrFuncResult::Address(addr) => {
            let v = cpu.read_byte(addr);
            cpu.registers.a = v;

            cpu.registers.z = cpu.registers.a == 0;
            cpu.registers.n = cpu.registers.a & FLAG_NEGATIVE > 0;
        }
        _ => {}
    }
}

pub(crate) fn ldx(result: AddrFuncResult, cpu: &mut Processor) {
    match result {
        AddrFuncResult::Immediate(v) => {
            cpu.registers.x = v;

            cpu.registers.z = cpu.registers.x == 0;
            cpu.registers.n = cpu.registers.x & FLAG_NEGATIVE > 0;
        }
        AddrFuncResult::Address(addr) => {
            let v = cpu.read_byte(addr);
            cpu.registers.x = v;

            cpu.registers.z = cpu.registers.x == 0;
            cpu.registers.n = cpu.registers.x & FLAG_NEGATIVE > 0;
        }
        _ => {}
    }
}

pub(crate) fn ldy(result: AddrFuncResult, cpu: &mut Processor) {
    match result {
        AddrFuncResult::Immediate(v) => {
            cpu.registers.y = v;

            cpu.registers.z = cpu.registers.y == 0;
            cpu.registers.n = cpu.registers.y & FLAG_NEGATIVE > 0;
        }
        AddrFuncResult::Address(addr) => {
            let v = cpu.read_byte(addr);
            cpu.registers.y = v;

            cpu.registers.z = cpu.registers.y == 0;
            cpu.registers.n = cpu.registers.y & FLAG_NEGATIVE > 0;
        }
        _ => {}
    }
}

pub(crate) fn lsr(result: AddrFuncResult, cpu: &mut Processor) {
    match result {
        AddrFuncResult::Implied => {
            cpu.registers.c = cpu.registers.a & FLAG_CARRY > 0;
            cpu.registers.a >>= 1;
            cpu.cycles += 1;

            cpu.registers.n = cpu.registers.a & FLAG_NEGATIVE > 0;
            cpu.registers.z = cpu.registers.a == 0;
        }
        AddrFuncResult::Address(addr) => {
            let mut value = cpu.read_byte(addr);
            cpu.registers.c = value & FLAG_CARRY > 0;
            value >>= 1;
            cpu.cycles += 1;

            cpu.write_byte(value, addr);

            cpu.registers.n = value & FLAG_NEGATIVE > 0;
            cpu.registers.z = value == 0;
        }
        _ => {}
    }
}

pub(crate) fn nop(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Implied = result {
        /* do nothing, no operations! */
        cpu.cycles += 1;
    }
}

pub(crate) fn ora(result: AddrFuncResult, cpu: &mut Processor) {
    match result {
        AddrFuncResult::Immediate(v) => {
            cpu.registers.a |= v;

            cpu.registers.n = cpu.registers.a & FLAG_NEGATIVE > 0;
            cpu.registers.z = cpu.registers.a == 0;
        }
        AddrFuncResult::Address(addr) => {
            cpu.registers.a |= cpu.read_byte(addr);

            cpu.registers.n = cpu.registers.a & FLAG_NEGATIVE > 0;
            cpu.registers.z = cpu.registers.a == 0;
        }
        _ => {}
    }
}

pub(crate) fn pha(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Implied = result {
        cpu.push_byte_onto_stack(cpu.registers.a);
    }
}

pub(crate) fn php(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Implied = result {
        cpu.push_status_to_stack();
    }
}

// pla uses 4 cycles, [fetch, pop, and two cycles which can't really be simulated so we just do +2]
pub(crate) fn pla(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Implied = result {
        cpu.registers.a = cpu.stack_pop_byte();
        cpu.registers.z = cpu.registers.a == 0;
        cpu.registers.n = cpu.registers.a & FLAG_NEGATIVE > 0;
        cpu.cycles += 2;
    }
}

// plp uses 4 cycles, [fetch, pop, and two cycles which can't really be simulated so we just do +2]
pub(crate) fn plp(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Implied = result {
        cpu.stack_pop_status();
        cpu.cycles += 2;
    }
}

// plp uses [2, 5, 6, 6, 7] cycles, [fetch + addressing, and one cycle which can't really be simulated so we just do +1]
pub(crate) fn rol(result: AddrFuncResult, cpu: &mut Processor) {
    match result {
        AddrFuncResult::Implied => {
            let mut m = cpu.registers.a;

            let new_bit = if cpu.registers.c { FLAG_CARRY } else { 0 };

            cpu.registers.c = m & FLAG_NEGATIVE > 0;

            m <<= 1;
            m |= new_bit;

            cpu.registers.a = m;
            cpu.cycles += 1;

            cpu.registers.z = cpu.registers.a == 0;
            cpu.registers.n = cpu.registers.a & FLAG_NEGATIVE > 0;
        }
        AddrFuncResult::Address(addr) => {
            let mut m = cpu.read_byte(addr);

            let new_bit = if cpu.registers.c { FLAG_CARRY } else { 0 };

            cpu.registers.c = m & FLAG_NEGATIVE > 0;

            m <<= 1;
            m |= new_bit;

            cpu.write_byte(m, addr);
            cpu.cycles += 1;
            cpu.registers.n = m & FLAG_NEGATIVE > 0;
            cpu.registers.z = m == 0;
        }
        _ => {}
    }
}

// plp uses [2, 5, 6, 6, 7] cycles, [fetch + addressing, and one cycle which can't really be simulated so we just do +1]
pub(crate) fn ror(result: AddrFuncResult, cpu: &mut Processor) {
    match result {
        AddrFuncResult::Implied => {
            let old_bit0 = (cpu.registers.a & FLAG_CARRY) > 0;
            cpu.registers.a >>= 1;
            if cpu.registers.c {
                cpu.registers.a |= FLAG_NEGATIVE;
            }
            cpu.cycles += 1;

            cpu.registers.c = old_bit0;
            cpu.registers.z = cpu.registers.a == 0;
            cpu.registers.n = cpu.registers.a & FLAG_NEGATIVE > 0;
        }
        AddrFuncResult::Address(addr) => {
            let mut m = cpu.read_byte(addr);
            let old_bit0 = (m & FLAG_CARRY) > 0;
            m >>= 1;

            if cpu.registers.c {
                m |= FLAG_NEGATIVE;
            }
            cpu.cycles += 1;

            cpu.write_byte(m, addr);

            cpu.registers.c = old_bit0;
            cpu.registers.z = m == 0;
            cpu.registers.n = m & FLAG_NEGATIVE > 0;
        }
        _ => {}
    }
}

pub(crate) fn rti(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Implied = result {
        let pop = cpu.stack_pop_byte();
        cpu.registers.from_byte(pop);
        cpu.registers.b = false;
        cpu.registers.unused = false;
        cpu.registers.pc = cpu.stack_pop_word();
        cpu.cycles += 2;
    }
}

pub(crate) fn rts(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Implied = result {
        cpu.registers.pc = cpu.stack_pop_word() + 0x0001;
        cpu.cycles += 3;
    }
}

pub(crate) fn sbc(result: AddrFuncResult, cpu: &mut Processor) {
    match result {
        AddrFuncResult::Immediate(v) => {
            adc(AddrFuncResult::Immediate(!v), cpu);
        }
        AddrFuncResult::Address(addr) => {
            let m = cpu.read_byte(addr);
            adc(AddrFuncResult::Immediate(!m), cpu);
        }
        _ => {}
    }
}

pub(crate) fn sec(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Implied = result {
        cpu.registers.c = true;
        cpu.cycles += 1;
    }
}

pub(crate) fn sed(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Implied = result {
        cpu.registers.d = true;
        cpu.cycles += 1;
    }
}

pub(crate) fn sei(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Implied = result {
        cpu.registers.i = true;
        cpu.cycles += 1;
    }
}

pub(crate) fn sta(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Address(addr) = result {
        cpu.write_byte(cpu.registers.a, addr);
    }
}

pub(crate) fn stx(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Address(addr) = result {
        cpu.write_byte(cpu.registers.x, addr);
    }
}

pub(crate) fn sty(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Address(addr) = result {
        cpu.write_byte(cpu.registers.y, addr);
    }
}

pub(crate) fn tax(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Implied = result {
        cpu.registers.x = cpu.registers.a;
        cpu.registers.z = cpu.registers.x == 0;
        cpu.registers.n = cpu.registers.x & FLAG_NEGATIVE > 0;
        cpu.cycles += 1;
    }
}

pub(crate) fn tay(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Implied = result {
        cpu.registers.y = cpu.registers.a;
        cpu.registers.z = cpu.registers.y == 0;
        cpu.registers.n = cpu.registers.y & FLAG_NEGATIVE > 0;
        cpu.cycles += 1;
    }
}

pub(crate) fn tsx(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Implied = result {
        cpu.registers.x = cpu.registers.sp;
        cpu.registers.z = cpu.registers.x == 0;
        cpu.registers.n = cpu.registers.x & FLAG_NEGATIVE > 0;
        cpu.cycles += 1;
    }
}

pub(crate) fn txa(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Implied = result {
        cpu.registers.a = cpu.registers.x;
        cpu.registers.z = cpu.registers.a == 0;
        cpu.registers.n = cpu.registers.a & FLAG_NEGATIVE > 0;
        cpu.cycles += 1;
    }
}

pub(crate) fn txs(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Implied = result {
        cpu.registers.sp = cpu.registers.x;
        cpu.cycles += 1;
    }
}

pub(crate) fn tya(result: AddrFuncResult, cpu: &mut Processor) {
    if let AddrFuncResult::Implied = result {
        cpu.registers.a = cpu.registers.y;
        cpu.registers.z = cpu.registers.a == 0;
        cpu.registers.n = cpu.registers.a & FLAG_NEGATIVE > 0;
        cpu.cycles += 1;
    }
}
