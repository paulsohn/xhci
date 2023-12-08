//! The xHCI Extended Capabilities

pub mod usb_debug;
pub mod hci_extended_power_management;
pub mod usb_legacy_support;
pub mod xhci_extended_message_interrupt;
pub mod xhci_local_memory;
pub mod xhci_message_interrupt;
pub mod xhci_supported_protocol;

use core::iter::Iterator;
use core::ptr::NonNull;
use volatile::VolatilePtr;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use super::vptr_to_addr;

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
    type Item = Result<ExtendedCapabilities<'r>, (usize, u8)>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.prev.as_raw_ptr() == self.ptr.as_raw_ptr() { return None; }

        self.prev = self.ptr;

        let base = vptr_to_addr(self.ptr);

        let item = self.ptr.read();
        self.ptr = unsafe {
            VolatilePtr::new(
                NonNull::new(
                    (
                        base + ((item.next() as usize) << 2)
                    ) as *mut _
                ).unwrap()
            )
        };

        let xcap = unsafe {
            ExtendedCapabilities::make_ptrs(base, item.id())
        };

        Some(xcap)
    }
}

macro_rules! ext_cap {
    ($name:ident {
        $($(#[$docs:meta])* $variant:ident($md:ident) = $val:literal),+ $(,)?
    }) => {
        // defining tags
        #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, FromPrimitive)]
        #[repr(u8)]
        enum Ty {
            $(
                $(#[$docs])*
                $variant = $val
            ),+
        }

        // defining disjoint unions
        #[doc = "xHCI Extended Capabilities currently supported."]
        #[allow(missing_debug_implementations)]
        pub enum $name<'r> {
            $(
                $(#[$docs])*
                $variant($md::Ptrs<'r>)
            ),+
        }
        impl $name<'_> {
            unsafe fn make_ptrs(base: usize, id: u8) -> Result<Self, (usize, u8)> {
                // on error, this returns the arguments back
                let ty = FromPrimitive::from_u8(id).ok_or((base, id))?;
                let ok = match ty {
                    $(
                        Ty::$variant => $name::$variant($md::Ptrs::new(base))
                    ),+
                };
                Ok(ok)
            }
        }
    }
}

ext_cap!(ExtendedCapabilities {
    /// USB Legacy Support.
    UsbLegacySupport(usb_legacy_support) = 1,
    /// xHCI Supported Protocol.
    SupportedProtocol(xhci_supported_protocol) = 2,
    /// HCI Extended Power Management.
    ExtendedPowerManagement(hci_extended_power_management) = 3,
    /// xHCI Message Interrupt(MSI).
    MessageInterrupt(xhci_message_interrupt) = 5,
    /// xHCI Local Memory.
    LocalMemory(xhci_local_memory) = 6,
    /// USB Debug.
    UsbDebug(usb_debug) = 10,
    /// xHCI Extended Message Interrupt(MSI-X).
    ExtendedMessageInterrupt(xhci_extended_message_interrupt) = 17,
});
