use crate::*;

pub(crate) fn implied(_: &mut Processor) -> AddrFuncResult {
    AddrFuncResult::Implied
}

pub(crate) fn immediate(cpu: &mut Processor) -> AddrFuncResult {
    let result = cpu.fetch_byte();

    AddrFuncResult::Immediate(result)
}

pub(crate) fn relative(cpu: &mut Processor) -> AddrFuncResult {
    let result = cpu.fetch_byte() as i8;

    AddrFuncResult::Relative(result)
}

pub(crate) fn absolute(cpu: &mut Processor) -> AddrFuncResult {
    let addr = cpu.fetch_word();

    AddrFuncResult::Address(addr)
}

pub(crate) fn absolute_x(cpu: &mut Processor) -> AddrFuncResult {
    let addr = cpu.fetch_word();
    let addrx = addr + cpu.registers.x as u16;
    cpu.cycles += 1;

    AddrFuncResult::Address(addrx)
}

pub(crate) fn absolute_x_more_if_crossed(cpu: &mut Processor) -> AddrFuncResult {
    let addr = cpu.fetch_word();
    let addrx = addr + cpu.registers.x as u16;

    if addrx - addr >= 0xFF {
        cpu.cycles += 1;
    }

    AddrFuncResult::Address(addrx)
}

pub(crate) fn absolute_y(cpu: &mut Processor) -> AddrFuncResult {
    let addr = cpu.fetch_word();
    let addry = addr + cpu.registers.y as u16;
    cpu.cycles += 1;

    AddrFuncResult::Address(addry)
}

pub(crate) fn absolute_y_more_if_crossed(cpu: &mut Processor) -> AddrFuncResult {
    let addr = cpu.fetch_word();
    let addry = addr + cpu.registers.y as u16;

    if addry - addr >= 0xFF {
        cpu.cycles += 1;
    }

    AddrFuncResult::Address(addry)
}

pub(crate) fn zero_page(cpu: &mut Processor) -> AddrFuncResult {
    let addr = cpu.fetch_byte();

    AddrFuncResult::Address(addr as u16)
}

pub(crate) fn zero_page_x(cpu: &mut Processor) -> AddrFuncResult {
    let addr = cpu.fetch_byte();
    let addrx = addr.wrapping_add(cpu.registers.x);
    cpu.cycles += 1;

    AddrFuncResult::Address(addrx as u16)
}

pub(crate) fn zero_page_y(cpu: &mut Processor) -> AddrFuncResult {
    let addr = cpu.fetch_byte();
    let addry = addr.wrapping_add(cpu.registers.y);
    cpu.cycles += 1;

    AddrFuncResult::Address(addry as u16)
}

pub(crate) fn indirect(cpu: &mut Processor) -> AddrFuncResult {
    let mut addr = cpu.fetch_word();
    addr = cpu.read_word(addr);

    AddrFuncResult::Address(addr)
}

pub(crate) fn indexed_indirect_x(cpu: &mut Processor) -> AddrFuncResult {
    let addr = u8::wrapping_add(cpu.fetch_byte(), cpu.registers.x);
    let addr = cpu.read_word(addr as u16);

    cpu.cycles += 1;
    AddrFuncResult::Address(addr)
}

pub(crate) fn indirect_indexed_y_more_if_crossed(cpu: &mut Processor) -> AddrFuncResult {
    let addr = cpu.fetch_byte();
    let addr = cpu.read_word(addr as u16);
    let addry = addr + cpu.registers.y as u16;

    if addry - addr >= 0xFF {
        cpu.cycles += 1;
    }

    AddrFuncResult::Address(addry)
}

pub(crate) fn indirect_indexed_y(cpu: &mut Processor) -> AddrFuncResult {
    let addr = cpu.fetch_byte();
    let addr = cpu.read_word(addr as u16);
    let addry = addr + cpu.registers.y as u16;
    cpu.cycles += 1;

    AddrFuncResult::Address(addry)
}
