use crate::*;

pub(crate) fn implied(_: &mut Processor) -> AddrFuncResult {
    AddrFuncResult::Implied
}

pub(crate) fn immediate(cpu: &mut Processor) -> AddrFuncResult {
    let result = cpu.fetch_byte();

    AddrFuncResult::Immediate(result)
}

pub(crate) fn relative(cpu: &mut Processor) -> AddrFuncResult {
    let result = cpu.fetch_byte() as SByte;

    AddrFuncResult::Relative(AddressDiff(result))
}

pub(crate) fn absolute(cpu: &mut Processor) -> AddrFuncResult {
    let address = cpu.fetch_word();

    AddrFuncResult::Address(Address(address))
}

pub(crate) fn absolute_x(cpu: &mut Processor) -> AddrFuncResult {
    let address = cpu.fetch_word();
    let address_x = address + cpu.registers.x as Word;
    cpu.cycles += 1;

    AddrFuncResult::Address(Address(address_x))
}

pub(crate) fn absolute_x_more_if_crossed(cpu: &mut Processor) -> AddrFuncResult {
    let address = cpu.fetch_word();
    let address_x = address + cpu.registers.x as Word;

    if address_x - address >= 0xFF {
        cpu.cycles += 1;
    }

    AddrFuncResult::Address(Address(address_x))
}

#[allow(dead_code)]
pub(crate) fn absolute_y(cpu: &mut Processor) -> AddrFuncResult {
    let address = cpu.fetch_word();
    let address_y = address + cpu.registers.y as Word;
    cpu.cycles += 1;

    AddrFuncResult::Address(Address(address_y))
}

pub(crate) fn absolute_y_more_if_crossed(cpu: &mut Processor) -> AddrFuncResult {
    let address = cpu.fetch_word();
    let address_y = address + cpu.registers.y as Word;

    if address_y - address >= 0xFF {
        cpu.cycles += 1;
    }

    AddrFuncResult::Address(Address(address_y))
}

pub(crate) fn zero_page(cpu: &mut Processor) -> AddrFuncResult {
    let address = cpu.fetch_byte();

    AddrFuncResult::Address(Address(address as Word))
}

pub(crate) fn zero_page_x(cpu: &mut Processor) -> AddrFuncResult {
    let address = cpu.fetch_byte();
    let address_x = address.wrapping_add(cpu.registers.x);
    cpu.cycles += 1;

    AddrFuncResult::Address(Address(address_x as Word))
}

pub(crate) fn zero_page_y(cpu: &mut Processor) -> AddrFuncResult {
    let address = cpu.fetch_byte();
    let address_y = address.wrapping_add(cpu.registers.y);
    cpu.cycles += 1;

    AddrFuncResult::Address(Address(address_y as Word))
}

pub(crate) fn indexed_indirect_x(cpu: &mut Processor) -> AddrFuncResult {
    let address = cpu.fetch_byte() + cpu.registers.x;
    let address = cpu.read_word(Address(address.into()));

    AddrFuncResult::Address(Address(address))
}

pub(crate) fn indirect_indexed_y_more_if_crossed(cpu: &mut Processor) -> AddrFuncResult {
    let address = cpu.fetch_byte();
    let address = cpu.read_word(Address(address.into()));
    let address_y = address + cpu.registers.y as Word;

    if address_y - address >= 0xFF {
        cpu.cycles += 1;
    }

    AddrFuncResult::Address(Address(address_y))
}
