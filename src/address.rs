use core::ops::Add;
use std::ops::AddAssign;

use crate::*;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct AddressDiff(pub i8);

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Address(pub u16);

impl Add<AddressDiff> for Address {
    type Output = Address;

    // needs some work, but it will do for now.
    fn add(self, AddressDiff(rhs): AddressDiff) -> Address {
        let Address(lhs) = self;

        let big_lhs = lhs as i32;
        let big_rhs = rhs as i32;

        let answer = (big_lhs + big_rhs) as u16;

        Address(answer)
    }
}

impl AddAssign<AddressDiff> for Address {
    fn add_assign(&mut self, rhs: AddressDiff) {
        *self = *self + rhs
    }
}

impl Add<AddressDiff> for AddressDiff {
    type Output = AddressDiff;

    fn add(self, AddressDiff(rhs): AddressDiff) -> AddressDiff {
        let AddressDiff(lhs) = self;
        AddressDiff(lhs + rhs)
    }
}

impl AddAssign<AddressDiff> for AddressDiff {
    fn add_assign(&mut self, rhs: AddressDiff) {
        *self = *self + rhs
    }
}

impl Address {
    pub fn to_word(self) -> Word {
        self.0
    }
}
