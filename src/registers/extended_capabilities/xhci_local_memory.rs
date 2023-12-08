//! xHCI Local Memory Capability.

use volatile::VolatilePtr;
use super::super::{addr_to_vptr, addr_len_to_vptr};

/// The complete xHCI Local Memory Capability pointers.
#[allow(missing_debug_implementations)]
pub struct Ptrs<'r> {
    /// The header part.
    pub header: VolatilePtr<'r, Header>,
    /// The memory dword array.
    pub memory: VolatilePtr<'r, [u32]>,    
}
impl Ptrs<'_> {
    /// Create the complete set of pointers from the base address.
    pub unsafe fn new(base: usize) -> Self {
        let header = addr_to_vptr::<Header>(base);

        let mem_base = base + core::mem::size_of::<Header>();
        let mem_len = (header.read().size() as usize) * 256; // *256 as dwords, *1024 as bytes
        let memory = addr_len_to_vptr(mem_base, mem_len);

        Self { header, memory }
    }
}

/// The entry point to xHCI Local Memory Capability.
/// This does not include the actual memory dwords.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Header([u32; 2]);
impl Header {
    rw_bit!(
        pub, self,
        self.0[0]; 16,
        local_memory_enable,
        "Local Memory Enable"
    );

    fn size(self) -> u32 {
        self.0[1]
    }
}
impl_debug_from_methods! { Header { local_memory_enable } }
