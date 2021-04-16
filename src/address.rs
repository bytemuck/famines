use core::ops::Add;
use std::ops::AddAssign;

use crate::*;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct RelativeAddress(pub i8);

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Address(pub u16);

impl Add<RelativeAddress> for Address {
    type Output = Address;

    // needs some work, but it will do for now.
    fn add(self, RelativeAddress(rhs): RelativeAddress) -> Address {
        let Address(lhs) = self;

        let big_lhs = lhs as i32;
        let big_rhs = rhs as i32;

        let answer = (big_lhs + big_rhs) as u16;

        Address(answer)
    }
}

impl AddAssign<RelativeAddress> for Address {
    fn add_assign(&mut self, rhs: RelativeAddress) {
        *self = *self + rhs
    }
}

impl Add<RelativeAddress> for RelativeAddress {
    type Output = RelativeAddress;

    fn add(self, RelativeAddress(rhs): RelativeAddress) -> RelativeAddress {
        let RelativeAddress(lhs) = self;
        RelativeAddress(lhs + rhs)
    }
}

impl AddAssign<RelativeAddress> for RelativeAddress {
    fn add_assign(&mut self, rhs: RelativeAddress) {
        *self = *self + rhs
    }
}

impl Address {
    pub fn to_word(self) -> Word {
        self.0
    }
}
