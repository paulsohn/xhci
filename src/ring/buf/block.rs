//! TRB Block.

use bit_field::BitField;
// use core::convert::TryInto;
use num_traits::FromPrimitive;

use crate::ring::trb;

/// TRB Block.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
pub struct Block([u32; 4]);

impl Block {
    /// New (invalid) block initialized with zero, except for cycle_bit.
    pub fn zero_with_cycle_bit(cycle_bit: bool) -> Self {
        let mut block = Self([0; 4]);
        if cycle_bit {
            block.set_cycle_bit();
        }
        block
    }

    /// New block from a raw array.
    pub fn new(raw: [u32; 4]) -> Self {
        Self(raw)
    }

    /// Returns the raw array.
    pub fn into_raw(self) -> [u32; 4] {
        self.0
    }

    rw_bit!([3](0), cycle_bit, "Cycle bit");
    rw_bit!([3](4), chain_bit, "Chain bit");

    /// Returns the value of the TRB Type, or `None` if invalid.
    pub fn trb_type(&self) -> Option<trb::Type> {
        trb::Type::from_u32(self.0[3].get_bits(10..=15))
    }

    // pub(crate) fn set_trb_type(&mut self, ty: trb::Type) -> &mut Self {
    //     self.0[3].set_bits(10..=15, ty as u32);
    //     self
    // }
}
