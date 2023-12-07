//! xHCI registers

use core::ptr::NonNull;

use volatile::VolatilePtr;
use volatile::access::ReadOnly;

pub use capability::Capability;
pub use doorbell::Doorbell;
pub use operational::{Operational, PortRegisterSet};
pub use runtime::InterrupterRegisterSet;
pub use runtime::Runtime;

pub mod capability;
pub mod doorbell;
pub mod operational;
pub mod runtime;

/// The access point to xHCI registers.
#[allow(missing_debug_implementations)]
pub struct Registers<'r> {
    /// Host Controller Capability Register
    pub capability: VolatilePtr<'r, Capability, ReadOnly>,
    /// Host Controller Operational Register
    pub operational: VolatilePtr<'r, Operational>,
    /// Port Register Set Array
    pub port_register_set_array: VolatilePtr<'r, [PortRegisterSet]>,
    /// Runtime Registers
    pub runtime: VolatilePtr<'r, Runtime>,
    /// Interrupter Register Set Array
    pub interrupter_register_set_array: VolatilePtr<'r, [InterrupterRegisterSet]>,
    /// Doorbell Array
    pub doorbell_array: VolatilePtr<'r, [Doorbell]>, // nit: this should be write only
}

impl Registers<'_> {
    /// Creates an instance of [`Regs`].
    pub unsafe fn new(mmio_base: usize) -> Self {
        assert!(mmio_base != 0, "`mmio_base` should be non null.");

        let capability = addr_to_vptr(mmio_base).read_only();
        let cap_value: Capability = capability.read();

        let op_base = mmio_base + (cap_value.caplength.get() as usize);
        let operational = addr_to_vptr(op_base);

        let port_arr_base = op_base + 0x400;
        let port_arr_len = cap_value.hcsparams1.number_of_ports() as usize;
        let port_register_set_array = addr_len_to_vptr(port_arr_base, port_arr_len);

        let rt_base = mmio_base + (cap_value.rtsoff.get() as usize);
        let runtime = addr_to_vptr(rt_base);

        let intr_arr_base = rt_base + 0x20;
        let intr_arr_len = 1024 as usize;
        let interrupter_register_set_array = addr_len_to_vptr(intr_arr_base, intr_arr_len);

        let db_arr_base = mmio_base + (cap_value.dboff.get() as usize);
        let db_arr_len = cap_value.hcsparams1.number_of_device_slots() as usize;
        let doorbell_array = addr_len_to_vptr(db_arr_base, db_arr_len);

        Self {
            capability,
            operational,
            port_register_set_array,

            runtime,
            interrupter_register_set_array,

            doorbell_array,
        }
    }
}

unsafe fn addr_to_vptr<T>(addr: usize) -> VolatilePtr<'static, T> {
    VolatilePtr::new(
        NonNull::new(addr as *mut T).unwrap()
    )
}

unsafe fn addr_len_to_vptr<T>(base: usize, len: usize) -> VolatilePtr<'static, [T]> {
    VolatilePtr::new(
        NonNull::new(
            core::ptr::slice_from_raw_parts_mut(
                base as *mut T,
                len
            )
        ).unwrap()
    )
}