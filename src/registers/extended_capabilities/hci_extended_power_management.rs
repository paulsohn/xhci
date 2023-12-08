//! HCI Extended Power Management Capability.

use volatile::VolatilePtr;
use super::super::addr_to_vptr;

/// The complete set of pointers of HCI Extended Power Management Capability.
#[allow(missing_debug_implementations)]
pub struct Ptrs<'r> {
    /// The only pointer.
    pub ptr: VolatilePtr<'r, HciExtendedPowerManagement>
}
impl Ptrs<'_> {
    /// Create the complete set of pointers from the base address.
    pub unsafe fn new(base: usize) -> Self {
        Self { ptr: addr_to_vptr(base) }
    }
}

/// HCI Extended Power Management Capability.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct HciExtendedPowerManagement {
    // The first two bytes of the Capability Header part.
    _id_next: u16,
    /// Power Management Capabilities.
    pub pmc: PowerManagementCapabilities,
    /// Power Management Control Status Register.
    pub pmcsr: PowerManagementControlStatusRegister,
    /// PMESR_BSE.
    pub pmcsr_bse: PmesrBse,
    /// Data.
    pub data: u8,
}

/// Power Management Capabilities.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct PowerManagementCapabilities(u16);
impl PowerManagementCapabilities {
    ro_field!(pub, self, self.0; 11..=15, pme_support, "PME_Support", u8);
    ro_bit!(pub, self, self.0; 10, d2_support, "D2_Support");
    ro_bit!(pub, self, self.0; 9, d1_support, "D1_Support");
    ro_field!(pub, self, self.0; 6..=8, aux_current, "Aux_Current", u8);
    ro_bit!(pub, self, self.0; 5, dsi, "DSI");
    ro_bit!(pub, self, self.0; 3, pme_clock, "PME Clock");
    ro_field!(pub, self, self.0; 0..=2, version, "Version", u8);
}
impl_debug_from_methods! {
    PowerManagementCapabilities {
        pme_support,
        d2_support,
        d1_support,
        aux_current,
        dsi,
        pme_clock,
        version,
    }
}

/// Power Management Control/Status Register.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct PowerManagementControlStatusRegister(u16);
impl PowerManagementControlStatusRegister {
    rw1c_bit!(pub, self, self.0; 15, pme_status, "PME_Status");
    ro_field!(pub, self, self.0; 13..=14, data_scale, "Data_Scale", u8);
    rw_field!(pub, self, self.0; 9..=12, data_select, "Data_Select", u8);
    rw_bit!(pub, self, self.0; 8, pme_en, "PME_En");
    rw_field!(pub, self, self.0; 0..=1, power_state, "PowerState", u8);
}
impl_debug_from_methods! {
    PowerManagementControlStatusRegister {
        pme_status,
        data_scale,
        data_select,
        pme_en,
        power_state,
    }
}

/// `PMESR_BSE` Register.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct PmesrBse(u8);
impl PmesrBse {
    ro_bit!(pub, self, self.0; 7, bpcc_en, "BPCC_En");
    ro_bit!(pub, self, self.0; 6, b2_b3, "B2_B3");
}
impl_debug_from_methods! {
    PmesrBse {
        bpcc_en,
        b2_b3,
    }
}