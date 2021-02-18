pub mod instructions;
pub use instructions::*;

pub mod memory;
pub use memory::*;

pub mod processor;
pub use processor::*;

pub fn check_mask(byte: u8, mask: u8) -> bool {
    byte & mask == mask
}