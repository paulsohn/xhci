//! The xHCI Extended Capabilities

pub mod debug;
pub mod hci_extended_power_management;
pub mod usb_legacy_support;
pub mod xhci_extended_message_interrupt;
pub mod xhci_local_memory;
pub mod xhci_message_interrupt;
pub mod xhci_supported_protocol;

use core::iter::Iterator;
use core::ptr::NonNull;
use volatile::VolatilePtr;

/// An xHCI Capability Header Register.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct CapabilityHeader(u32);
impl CapabilityHeader {
    ro_field!(pub(crate), self, self.0; 0..=7, id, "Cap ID", u8);
    ro_field!(pub(crate), self, self.0; 8..=15, next, "Next Offset", u8);
    // ro_field!(pub(crate), self, self.0; 16..=31, payload, "Payload", u8);
}
impl_debug_from_methods!(CapabilityHeader {
    id, next,
});

/// A struct to access xHCI Extended Capabilities.
#[derive(Debug)]
pub struct List<'r> {
    prev: VolatilePtr<'r, CapabilityHeader>,
    ptr: VolatilePtr<'r, CapabilityHeader>,
}
impl<'r> List<'r> {
    /// Creates a List from the first 
    pub(crate) fn new(
        prev: VolatilePtr<'r, CapabilityHeader>,
        ptr: VolatilePtr<'r, CapabilityHeader>
    ) -> Self {
        // By setting `prev`, we can unify the iteration logic of empty and non-empty lists.
        Self { prev, ptr }
    }
}
impl<'r> Iterator for List<'r> {
    type Item = VolatilePtr<'r, CapabilityHeader>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.prev.as_raw_ptr() == self.ptr.as_raw_ptr() { return None; }

        self.prev = self.ptr;

        let item = self.ptr.read();
        self.ptr = unsafe {
            VolatilePtr::new(
                NonNull::new(
                    (
                        (self.ptr.as_raw_ptr().as_ptr() as usize)
                        + ((item.next() as usize) << 2)
                    ) as *mut _
                ).unwrap()
            )
        };

        Some(self.prev)
    }
}

