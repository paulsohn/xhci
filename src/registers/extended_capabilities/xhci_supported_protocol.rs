//! xHCI Supported Protocol Capability

use bit_field::BitField;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use volatile::VolatilePtr;
use super::super::{addr_to_vptr, addr_len_to_vptr};

/// The complete xHCI Supported Protocol Capability pointers.
#[allow(missing_debug_implementations)]
pub struct Ptrs<'r> {
    /// The header part.
    pub header: VolatilePtr<'r, Header>,
    /// The Protocol Speed ID array.
    pub psi_array: VolatilePtr<'r, [ProtocolSpeedId]>,    
}
impl Ptrs<'_> {
    /// Create the complete set of pointers from the base address.
    pub unsafe fn new(base: usize) -> Self {
        let header = addr_to_vptr::<Header>(base);

        let psi_base = base + core::mem::size_of::<Header>();
        let psi_len = header.read().protocol_speed_id_count() as usize;
        let psi_array = addr_len_to_vptr(psi_base, psi_len);

        Self { header, psi_array }
    }
}

/// The entry point to xHCI Supported Protocol Capability.
/// This does not include the Protocol Speed ID array.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Header([u32; 4]);
impl Header {
    ro_field!(pub, self, self.0[0]; 16..=23, minor_revision, "Minor Revision", u8);
    ro_field!(pub, self, self.0[0]; 24..=31, major_revision, "Major Revision", u8);
    ro_field!(pub, self, self.0[1]; 0..=31, name_string, "Name String", u8);
    ro_field!(pub, self, self.0[2]; 0..=7, compatible_port_offset, "Compatible Port Offset", u8);
    ro_field!(pub, self, self.0[2]; 8..=15, compatible_port_count, "Compatible Port Count", u8);

    ro_bit!(pub, self, self.0[2]; 17, high_speed_only, "**USB2** High-Speed Only");
    ro_bit!(pub, self, self.0[2]; 18, integrated_hub_implemented, "**USB2** Integrated Hub Implemented");
    ro_bit!(pub, self, self.0[2]; 19, hardware_lpm_capability, "**USB2** Hardware LPM Capability");
    ro_bit!(pub, self, self.0[2]; 20, besl_lpm_capability, "**USB2** BESL LPM Capability");

    ro_bit!(pub, self, self.0[2]; 24, link_soft_error_count_capability, "**USB3** Link Soft Error Count Capability");

    ro_field!(pub, self, self.0[2]; 25..=27, hub_depth, "Hub Depth", u8);
    ro_field!(pub, self, self.0[2]; 28..=31, protocol_speed_id_count, "Protocol Speed ID Count", u8);
    ro_field!(pub, self, self.0[3]; 0..=4, protocol_slot_type, "Protocol Slot Type", u8);
}
impl_debug_from_methods! {
    Header {
        minor_revision,
        major_revision,
        name_string,
        compatible_port_offset,
        compatible_port_count,
        link_soft_error_count_capability,
        high_speed_only,
        integrated_hub_implemented,
        hardware_lpm_capability,
        besl_lpm_capability,
        hub_depth,
        protocol_speed_id_count,
        protocol_slot_type,
    }
}

/// Protocol Speed ID
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ProtocolSpeedId(u32);
impl ProtocolSpeedId {
    ro_field!(pub, self, self.0; 0..=3, protocol_speed_id_value, "Protocol Speed ID Value", u8);

    /// Returns the value of the Protocol Speed ID Exponent field.
    #[must_use]
    pub fn protocol_speed_id_exponent(self) -> BitRate {
        FromPrimitive::from_u8(self.0.get_bits(4..=5) as u8).unwrap()
    }

    /// Returns the value of the PSI Type field.
    #[must_use]
    pub fn psi_type(self) -> PsiType {
        let r = FromPrimitive::from_u32(self.0.get_bits(6..=7));
        r.expect("The PSI Type must not take the reserved value.")
    }

    ro_bit!(pub, self, self.0; 8, psi_full_duplex, "PSI Full-Duplex");

    /// Returns the value of the Link Protocol field.
    #[must_use]
    pub fn link_protocol(self) -> LinkProtocol {
        let r = FromPrimitive::from_u32(self.0.get_bits(14..=15));
        r.expect("The Link Protocol field must not take the reserved value.")
    }

    ro_field!(
        pub, self,
        self.0; 16..=31,
        protocol_speed_id_mantissa,
        "Protocol Speed ID Mantissa",
        u16
    );
}
impl_debug_from_methods! {
    ProtocolSpeedId {
        protocol_speed_id_value,
        protocol_speed_id_exponent,
        psi_type,
        psi_full_duplex,
        link_protocol,
        protocol_speed_id_mantissa,
    }
}

/// Bit Rate
///
/// [`ProtocolSpeedId::protocol_speed_id_exponent`] returns a value of this type.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, FromPrimitive)]
pub enum BitRate {
    /// Bits Per Second
    Bits = 0,
    /// Kb/s
    Kb = 1,
    /// Mb/s
    Mb = 2,
    /// Gb/s
    Gb = 3,
}

/// PSI Type
///
/// [`ProtocolSpeedId::psi_type`] returns a value of this type.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, FromPrimitive)]
pub enum PsiType {
    /// Symmetric.
    ///
    /// Single DSI Dword.
    Symmetric = 0,
    /// Asymmetric Rx.
    ///
    /// Paired with Asymmetric Tx PSI Dword.
    AsymmetricRx = 2,
    /// Asymmetric Tx.
    ///
    /// Immediately follows Rx Asymmetric PSI Dword.
    AsymmetricTx = 3,
}

/// Link-level protocol
///
/// [`ProtocolSpeedId::link_protocol`] returns a value of this type.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, FromPrimitive)]
pub enum LinkProtocol {
    /// Super Speed
    SuperSpeed = 0,
    /// Super Speed Plus
    SuperSpeedPlus = 1,
}
