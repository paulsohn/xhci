//! WIP

use bit_field::BitField;
use num_traits::FromPrimitive;
use num_derive::FromPrimitive;

/// 32-byte Input Context.
pub type Input32Byte = Input<()>;
/// 64-byte Input Context.
pub type Input64Byte = Input<[u32; 8]>;

/// 32-byte Output (Device) Context.
pub type Output32Byte = Output<()>;
/// 64-byte Output (Device) Context.
pub type Output64Byte = Output<[u32; 8]>;

/// A Context Pad type.
/// 
/// `()` for 32-byte contexts, and `[u32;4]` for 64-byte contexts.
pub trait ContextPad: Copy + Clone + Default {}
impl ContextPad for () {}
impl ContextPad for [u32; 8] {}

/// The number of Endpoint Contexts in a Device Context.
pub const NUM_OF_ENDPOINT_CONTEXTS: usize = 31;
fn assert_dci(dci: usize) {
    assert_ne!(
        dci, 0,
        "Call `.slot()` instead of `.endpoint(0)` to get the Slot Context."
    );
    assert!(dci <= NUM_OF_ENDPOINT_CONTEXTS, "DCI must be {} at max.", NUM_OF_ENDPOINT_CONTEXTS);
}

/// Input Context.
#[repr(C, align(16))]
#[derive(Copy, Clone, Debug)]
pub struct Input<PAD: ContextPad> {
    control: (InputControl, PAD),
    device: Device<PAD>,
}
/// A trait to handle Input Context.
pub trait InputHandler {
    /// Returns a shared reference of Input Control Context.
    fn control(&self) -> &InputControl;
    /// Returns a mutable reference of Input Control Context.
    fn control_mut(&mut self) -> &mut InputControl;
    /// Returns a shared reference of Slot Context.
    fn slot(&self) -> &Slot;
    /// Returns a mutable reference of Slot Context.
    fn slot_mut(&mut self) -> &mut Slot;
    /// Returns a shared reference of Endpoint Context index `dci`.
    /// 
    /// # Panics
    ///
    /// This method panics if `dci > 31 || dci == 0`.
    /// Call [`InputHandler::slot`] if you want the Slot Context.
    fn endpoint(&self, dci: usize) -> &Endpoint;
    /// Returns a mutable reference of Endpoint Context index `dci`.
    /// 
    /// # Panics
    ///
    /// This method panics if `dci > 31 || dci == 0`.
    /// Call [`InputHandler::slot`] if you want the Slot Context.
    fn endpoint_mut(&mut self, dci: usize) -> &mut Endpoint;
}
impl<PAD: ContextPad> Input<PAD> {
    /// Creates a new Input Context.
    pub fn new() -> Self {
        Default::default()
    }
}
impl<PAD: ContextPad> Default for Input<PAD> {
    fn default() -> Self {
        Self { control: Default::default(), device: Default::default() }
    }
}
impl<PAD: ContextPad> InputHandler for Input<PAD> {
    fn control(&self) -> &InputControl {
        &self.control.0
    }

    fn control_mut(&mut self) -> &mut InputControl {
        &mut self.control.0
    }

    fn slot(&self) -> &Slot {
        &self.device.slot.0
    }

    fn slot_mut(&mut self) -> &mut Slot {
        &mut self.device.slot.0
    }

    fn endpoint(&self, dci: usize) -> &Endpoint {
        assert_dci(dci);

        &self.device.endpoints[dci - 1].0
    }

    fn endpoint_mut(&mut self, dci: usize) -> &mut Endpoint {
        assert_dci(dci);

        &mut self.device.endpoints[dci - 1].0
    }
}

/// Output Device Context.
#[repr(C, align(64))]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Output<PAD: ContextPad> {
    device: Device<PAD>,
}
/// A trait to handle Output Context.
pub trait OutputHandler {
    /// Returns a shared reference of Slot Context.
    fn slot(&self) -> &Slot;
    /// Returns a mutable reference of Slot Context.
    fn slot_mut(&mut self) -> &mut Slot;
    /// Returns a shared reference of Endpoint Context index `dci`.
    /// 
    /// # Panics
    ///
    /// This method panics if `dci > 31 || dci == 0`.
    /// Call [`InputHandler::slot`] if you want the Slot Context.
    fn endpoint(&self, dci: usize) -> &Endpoint;
    /// Returns a mutable reference of Endpoint Context index `dci`.
    /// 
    /// # Panics
    ///
    /// This method panics if `dci > 31 || dci == 0`.
    /// Call [`InputHandler::slot`] if you want the Slot Context.
    fn endpoint_mut(&mut self, dci: usize) -> &mut Endpoint;
}
impl<PAD: ContextPad> Output<PAD> {
    /// Creates a new Output (Device) Context.
    pub fn new() -> Self {
        Default::default()
    }
}
impl<PAD: ContextPad> Default for Output<PAD> {
    fn default() -> Self {
        Self { device: Default::default() }
    }
}
impl<PAD: ContextPad> OutputHandler for Output<PAD> {
    fn slot(&self) -> &Slot {
        &self.device.slot.0
    }

    fn slot_mut(&mut self) -> &mut Slot {
        &mut self.device.slot.0
    }

    fn endpoint(&self, dci: usize) -> &Endpoint {
        assert_dci(dci);

        &self.device.endpoints[dci - 1].0
    }

    fn endpoint_mut(&mut self, dci: usize) -> &mut Endpoint {
        assert_dci(dci);
        
        &mut self.device.endpoints[dci - 1].0
    }
}

/// Device Context.
#[repr(C)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub(crate) struct Device<PAD: ContextPad> {
    pub(crate) slot: (Slot, PAD),
    pub(crate) endpoints: [(Endpoint, PAD); NUM_OF_ENDPOINT_CONTEXTS],
}

/// The (first 32 bytes of the) Input Control Context.
/// 
/// Note that in 64-byte context mode, next 32 bytes are reserved as zero and irrelevant to the software.
#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub struct InputControl([u32; 8]);
impl InputControl {
    /// Creates an empty context.
    pub const fn new() -> Self {
        Self([0; 8])
    }

    ro_field!(pub(self), self, self.0[0];.., drop_context_flags, "Drop Context Flags", u32);

    /// Returns the `i`th Drop Context flag. `i` starts from 0.
    ///
    /// # Panics
    ///
    /// This method panics if `i < 2 || i > 31`.
    #[must_use]
    pub fn drop_context_flag(&self, i: usize) -> bool {
        self.ensure_drop_context_index_within_range(i);

        self.0[0].get_bit(i)
    }

    /// Sets the `i`th Drop Context flag. `i` starts from 0.
    ///
    /// # Panics
    ///
    /// This method panics if `i < 2 || i > 31`.
    pub fn set_drop_context_flag(&mut self, i: usize) -> &mut Self {
        self.ensure_drop_context_index_within_range(i);

        self.0[0].set_bit(i, true);
        self
    }

    /// Clears the `i`th Drop Context flag. `i` starts from 0.
    ///
    /// # Panics
    ///
    /// This method panics if `i < 2 || i > 31`.
    pub fn clear_drop_context_flag(&mut self, i: usize) -> &mut Self {
        self.ensure_drop_context_index_within_range(i);

        self.0[0].set_bit(i, false);
        self
    }

    ro_field!(pub(self), self, self.0[1];.., add_context_flags, "Add Context Flags", u32);

    /// Returns the `i`th Add Context flag. `i` starts from 0.
    ///
    /// # Panics
    ///
    /// This method panics if `i > 31`.
    #[must_use]
    pub fn add_context_flag(&self, i: usize) -> bool {
        self.ensure_add_context_index_within_range(i);

        self.0[1].get_bit(i)
    }

    /// Sets the `i`th Add Context flag. `i` starts from 0.
    ///
    /// # Panics
    ///
    /// This method panics if `i > 31`.
    pub fn set_add_context_flag(&mut self, i: usize) -> &mut Self {
        self.ensure_add_context_index_within_range(i);

        self.0[1].set_bit(i, true);
        self
    }

    /// Clears the `i`th Add Context flag. `i` starts from 0.
    ///
    /// # Panics
    ///
    /// This method panics if `i > 31`.
    pub fn clear_add_context_flag(&mut self, i: usize) -> &mut Self {
        self.ensure_add_context_index_within_range(i);

        self.0[1].set_bit(i, false);
        self
    }

    rw_field!(
        pub, self, self.0[7];0..=7,
        configuration_value, "Configuration Value", u8
    );
    rw_field!(
        pub, self, self.0[7];8..=15,
        interface_number, "Interface Number", u8
    );
    rw_field!(
        pub, self, self.0[7];16..=23,
        alternate_setting, "Alternate Setting", u8
    );

    #[doc(hidden)]
    fn ensure_drop_context_index_within_range(&self, i: usize) {
        assert!(
            (2..=31).contains(&i),
            "The index of Drop Context flag must be within 2..=31."
        );
    }

    #[doc(hidden)]
    fn ensure_add_context_index_within_range(&self, i: usize) {
        assert!(
            i <= 31,
            "The index of Add Context flag must be less than 32."
        );
    }
}
impl_debug_from_methods!(InputControl {
    drop_context_flags,
    add_context_flags,
    configuration_value,
    interface_number,
    alternate_setting,
});

/// The (first 32 bytes of the) Slot Context.
/// 
/// Note that in 64-byte context mode, next 32 bytes are reserved as zero and irrelevant to the software.
#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub struct Slot([u32; 8]);
impl Slot {
    /// Creates an empty context.
    pub const fn new() -> Self {
        Self([0; 8])
    }

    rw_field!(pub, self, self.0[0]; 0..=19, route_string, "Route String", u32);
    rw_field!(pub, self, self.0[0]; 20..=23, speed, "Speed", u8);
    rw_bit!(pub, self, self.0[0]; 25, multi_tt, "Multi-TT");
    rw_bit!(pub, self, self.0[0]; 26, hub, "Hub");
    rw_field!(pub, self, self.0[0]; 27..=31, context_entries, "Context Entries", u8);

    rw_field!(pub, self, self.0[1]; 0..=15, max_exit_latency, "Max Exit Latency", u16);
    rw_field!(pub, self, self.0[1]; 16..=23, root_hub_port_number, "Root Hub Port Number", u8);
    rw_field!(pub, self, self.0[1]; 24..=31, number_of_ports, "Number of Ports", u8);

    rw_field!(pub, self, self.0[2]; 0..=7, parent_hub_slot_id, "Parent Hub Slot ID", u8);
    rw_field!(pub, self, self.0[2]; 8..=15, parent_port_number, "Parent Port Number", u8);
    rw_field!(pub, self, self.0[2]; 16..=17, tt_think_time, "TT Think Time", u8);
    rw_field!(pub, self, self.0[2]; 22..=31, interrupter_target, "Interrupter Target", u16);

    rw_field!(pub, self, self.0[3]; 0..=7, usb_device_address, "USB Device Address", u8);
    /// Returns Slot State.
    ///
    /// # Panics
    ///
    /// This method panics if the Slot State represents Reserved.
    #[must_use]
    pub fn slot_state(&self) -> SlotState {
        let v = self.0[3].get_bits(27..=31);
        let s = FromPrimitive::from_u32(v);
        s.expect("Slot State represents Reserved.")
    }
    /// Sets Slot State.
    pub fn set_slot_state(&mut self, state: SlotState) -> &mut Self {
        self.0[3].set_bits(27..=31, state as _);
        self
    }
}
impl_debug_from_methods!(Slot {
    route_string,
    speed,
    multi_tt,
    hub,
    context_entries,
    max_exit_latency,
    root_hub_port_number,
    number_of_ports,
    parent_hub_slot_id,
    parent_port_number,
    tt_think_time,
    interrupter_target,
    usb_device_address,
    slot_state,
});

/// The (first 32 bytes of the) Endpoint Context.
/// 
/// Note that in 64-byte context mode, next 32 bytes are reserved as zero and irrelevant to the software.
#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub struct Endpoint([u32; 8]);
impl Endpoint {
    /// Creates an empty context.
    pub const fn new() -> Self {
        Self([0; 8])
    }

    /// Returns Endpoint State.
    ///
    /// # Panics
    ///
    /// This method panics if the Endpoint State represents Reserved.
    #[must_use]
    pub fn endpoint_state(&self) -> EndpointState {
        let v = self.0[0].get_bits(0..=2);
        let s = FromPrimitive::from_u32(v);
        s.expect("Endpoint State represents Reserved.")
    }
    /// Sets Endpoint State.
    pub fn set_endpoint_state(&mut self, s: EndpointState) -> &mut Self {
        self.0[0].set_bits(0..=2, s as _);
        self
    }

    rw_field!(pub, self, self.0[0];8..=9, mult, "Mult", u8);
    rw_field!(pub, self, self.0[0];10..=14, max_primary_streams, "Max Primary Streams", u8);
    rw_bit!(pub, self, self.0[0];15, linear_stream_array, "Linear Stream Array");
    rw_field!(pub, self, self.0[0];16..=23, interval, "Interval", u8);
    rw_field!(
        pub, self,
        self.0[0]; 24..=31,
        max_endpoint_service_time_interval_payload_high,
        "Max Endpoint Service Time Interval Payload High",
        u8
    );

    rw_field!(pub, self, self.0[1];1..=2, error_count, "Error Count", u8);
    /// Returns Endpoint Type.
    #[must_use]
    pub fn endpoint_type(&self) -> EndpointType {
        let v = self.0[1].get_bits(3..=5);
        let t = FromPrimitive::from_u32(v);
        t.expect("Invalid Endpoint Type.")
    }

    /// Sets Endpoint Type.
    pub fn set_endpoint_type(&mut self, t: EndpointType) -> &mut Self {
        self.0[1].set_bits(3..=5, t as _);
        self
    }

    rw_bit!(pub, self, self.0[1];7, host_initiate_disable, "Host Initiate Disable");
    rw_field!(pub, self, self.0[1];8..=15, max_burst_size, "Max Burst Size", u8);
    rw_field!(pub, self, self.0[1];16..=31, max_packet_size, "Max Packet Size", u16);

    rw_bit!(pub, self, self.0[2];0, dequeue_cycle_state, "Dequeue Cycle State");
    rw_double_zero_trailing!(
        pub, self,
        self.0; [2, 3]; 6~; "64-byte aligned",
        tr_dequeue_pointer,
        "TR Dequeue Pointer",
        32, u64
    );

    rw_field!(pub, self, self.0[4];0..=15, average_trb_length, "Average TRB Length", u16);
    rw_field!(
        pub, self,
        self.0[4];16..=31,
        max_endpoint_service_time_interval_payload_low,
        "Max Endpoint Service Time Interval Payload Low",
        u16
    );
}
impl_debug_from_methods!(Endpoint {
    endpoint_state,
    mult,
    max_primary_streams,
    linear_stream_array,
    interval,
    max_endpoint_service_time_interval_payload_high,
    error_count,
    endpoint_type,
    host_initiate_disable,
    max_burst_size,
    max_packet_size,
    dequeue_cycle_state,
    tr_dequeue_pointer,
    average_trb_length,
    max_endpoint_service_time_interval_payload_low,
});

/// Slot State.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, FromPrimitive)]
pub enum SlotState {
    /// Disabled/Enabled.
    DisabledEnabled = 0,
    /// Default.
    Default = 1,
    /// Addressed.
    Addressed = 2,
    /// Configured.
    Configured = 3,
}

/// Endpoint State.
///
/// The descriptions of each variant are taken from Table 6-8 of eXtensible Host Controller Interface for Universal Serial Bus(xHCI) Requirements Specification May2019 Revision 1.2.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, FromPrimitive)]
pub enum EndpointState {
    /// The endpoint is not operational.
    Disabled = 0,
    /// The endpoint is operational, either waiting for a doorbell ring or processing TDs.
    Running = 1,
    /// The endpoint is halted due to a Halt condition detected on the USB.
    Halted = 2,
    /// The endpoint is not running due to a Stop Endpoint Command or recovering from a Halt
    /// condition.
    Stopped = 3,
    /// The endpoint is not running due to a TRB Erorr.
    Error = 4,
}

/// Endpoint Type.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, FromPrimitive)]
pub enum EndpointType {
    /// Not Valid.
    NotValid = 0,
    /// Isoch Out.
    IsochOut = 1,
    /// Bulk Out.
    BulkOut = 2,
    /// Interrupt Out.
    InterruptOut = 3,
    /// Control Bidirectional.
    Control = 4,
    /// Isoch In.
    IsochIn = 5,
    /// Bulk In.
    BulkIn = 6,
    /// Interrupt In.
    InterruptIn = 7,
}