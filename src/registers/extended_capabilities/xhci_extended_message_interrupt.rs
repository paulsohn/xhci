//! xHCI Extended Message Interrupt Capability.

use volatile::VolatilePtr;
use super::super::addr_to_vptr;

/// The complete set of pointers of xHCI Extended Message Interrupt Capability.
#[allow(missing_debug_implementations)]
pub struct Ptrs<'r> {
    /// The only pointer.
    pub ptr: VolatilePtr<'r, XhciExtendedMessageInterrupt>
}
impl Ptrs<'_> {
    /// Create the complete set of pointers from the base address.
    pub unsafe fn new(base: usize) -> Self {
        Self { ptr: addr_to_vptr(base) }
    }
}

/// xHCI Extended Message Interrupt Capability.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XhciExtendedMessageInterrupt {
    // The first two bytes of the Capability Header part.
    _id_next: u16,
    /// Message Control.
    pub control: MessageControl,
    /// Message Upper Address.
    pub upper_address: u32,
    /// Table Offset and BIR.
    pub table_offset: TableOffset,
}

/// Message Control.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct MessageControl(u16);
impl MessageControl {
    ro_field!(pub, self, self.0; 0..=10, table_size, "Table Size", u16);
    rw_bit!(pub, self, self.0; 15, msi_x_enable, "MSI-X Enable");
}
impl_debug_from_methods! {
    MessageControl {
        msi_x_enable,
        table_size,
    }
}

/// Table Offset and BIR.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct TableOffset(u32);
impl TableOffset {
    ro_field!(pub, self, self.0; 0..=2, bir, "BIR value", u8);

    /// Returns the 8-byte aligned offset.
    #[must_use]
    pub fn offset(self) -> u32 {
        self.0 & !0b111
    }
}
impl_debug_from_methods! {
    TableOffset {
        offset,
        bir,
    }
}
