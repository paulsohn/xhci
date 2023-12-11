//! xHCI Message Interrupt Capability.

use volatile::VolatilePtr;
use super::super::addr_to_vptr;

/// The complete xHCI Message Interrupt Capability pointers.
#[allow(missing_debug_implementations)]
pub enum Ptrs<'r> {
    /// Variant using 32-bit Message Address.
    Addr32(VolatilePtr<'r, XhciMessageInterrupt<1>>),
    /// Variant using 64-bit Message Address.
    Addr64(VolatilePtr<'r, XhciMessageInterrupt<2>>),
}
impl Ptrs<'_> {
    /// Create the complete set of pointers from the base address.
    pub unsafe fn new(base: usize) -> Self {
        let control = addr_to_vptr::<MessageControl>(base);
        if control.read().bit64_address_capable() {
            Self::Addr64(addr_to_vptr::<XhciMessageInterrupt<2>>(base))
        } else {
            Self::Addr32(addr_to_vptr::<XhciMessageInterrupt<1>>(base))
        }
    }

    /// Returns if the pointer is 64-bit address capable.
    pub fn bit64_address_capable(&self) -> bool {
        match self {
            Self::Addr64(_) => true,
            _ => false,
        }
    }
}

/// xHCI Message Interrupt Capability.
#[derive(Copy, Clone, Debug, /* volatile_field::StructuralOf */)]
#[repr(C)]
pub struct XhciMessageInterrupt<const N: usize> {
    // The first two bytes of the Capability Header part.
    _id_next: u16,
    /// Message Control.
    pub control: MessageControl,
    /// Message Address.
    addr: [u32; 2], // `address[1]` should be unused in 32-bit addr mode.
    /// Data.
    pub data: u16,
    _padding: u16,
    /// Mask Bits.
    pub mask_bits: u32,
    /// Pending Bits.
    pub pending_bits: u32,
}
impl XhciMessageInterrupt<1> {
    rw_zero_trailing!(
        pub, self,
        self.addr[0]; 2~; "4-byte alligned",
        address,
        "32-bit Message Address",
        u32
    );
}
impl XhciMessageInterrupt<2> {
    rw_double_zero_trailing!(
        pub, self,
        self.addr; [0, 1]; 2~; "4-byte aligned",
        address,
        "64-bit Message Address",
        32, u64
    );
}

/// Message Control.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct MessageControl(u16);
impl MessageControl {
    rw_bit!(pub, self, self.0; 0, msi_enable, "MSI Enable");
    ro_field!(
        pub, self,
        self.0; 1..=3,
        multiple_message_capable,
        "Multiple Message Capable",
        u8
    );
    rw_field!(
        pub, self,
        self.0; 4..=6,
        multiple_message_enable,
        "Multiple Message Enable",
        u8
    );
    ro_bit!(pub, self, self.0; 7, bit64_address_capable, "64 bit address capable");
    ro_bit!(pub, self, self.0; 8, per_vector_masking_capable, "Per-vector masking capable");
}
impl_debug_from_methods! {
    MessageControl {
        per_vector_masking_capable,
        bit64_address_capable,
        multiple_message_enable,
        multiple_message_capable,
        msi_enable,
    }
}
