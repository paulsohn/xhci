//! Debug Capability.


/// The entry point to the Debug Capability.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Debug {
    /// Capability ID, which is the Capability Header part.
    pub dcid: Id, // off 0x00
    /// Doorbell.
    pub dcdb: Doorbell, // off 0x04
    /// Event Ring Segment Table Size.
    pub dcerstsz: EventRingSegmentTableSize, // off 0x08
    _padding_0c_10: u32,
    /// Event Ring Segment Table Base Address.
    pub dcerstba: EventRingSegmentTableBaseAddress, // off 0x10
    /// Event Ring Dequeue Pointer.
    pub dcerdp: EventRingDequeuePointer, // off 0x18
    /// Control.
    pub dcctrl: Control, // off 0x20
    /// Status.
    pub dcst: Status, // off 0x24
    /// Port Status and Control.
    pub dcportsc: PortStatusAndControl, // off 0x28
    _padding_2c_30: u32,
    /// Debug Capability Context Pointer.
    pub dccp: ContextPointer, // off 0x30
    /// Device Descriptor Info Register 1.
    pub dcddi1: DeviceDescriptorInfo1, // off 0x38
    /// Device Descriptor Info Register 2.
    pub dcddi2: DeviceDescriptorInfo2, // off 0x3c
}

/// Debug Capability ID Register.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Id(u32);
impl Id {
    ro_field!(
        pub(self), self,
        self.0; 16..=20,
        dcerst_max,
        "DCERST Max",
        u32
    );

    /// Returns the maximum number of the elements the Debug Capability Event Ring Segment Table can contain.
    ///
    /// Note that the `DCERST Max` field contains the exponent,
    /// but this method returns the calculated value.
    #[must_use]
    pub fn debug_capability_event_ring_segment_table_max(self) -> u16 {
        2_u16.pow(self.dcerst_max())
    }
}
impl_debug_from_methods! {
    Id {
        debug_capability_event_ring_segment_table_max,
    }
}

/// Debug Capability Doorbell Register.
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct Doorbell(u32);
impl Doorbell {
    rw_field!(
        pub, self,
        self.0; 8..=15,
        "Doorbell Target",
        u8
    );
}

/// Debug Capability Event Ring Segment Table Size Register.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct EventRingSegmentTableSize(u32);
impl EventRingSegmentTableSize {
    rw_field!(pub, self, self.0; 0..=15, "Event Ring Segment Table Size", u16);
}
impl_debug_from_methods! {
    EventRingSegmentTableSize {
        get,
    }
}

/// Debug Capability Event Ring Segment Table Base Address Register.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct EventRingSegmentTableBaseAddress(u64);
impl EventRingSegmentTableBaseAddress {
    rw_zero_trailing!(
        pub, self,
        self.0; 4~; "64-byte aligned",
        "Event Ring Segment Table Base Address",
        u64
    );
}
impl_debug_from_methods! {
    EventRingSegmentTableBaseAddress {
        get,
    }
}

/// Debug Capability Event Ring Dequeue Pointer Register.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct EventRingDequeuePointer(u64);
impl EventRingDequeuePointer {
    rw_field!(
        pub, self,
        self.0; 0..=2,
        dequeue_erst_segment_index,
        "Dequeue ERST Segment Index",
        u8
    );
    rw_zero_trailing!(
        pub, self,
        self.0; 4~; "16-byte aligned",
        dequeue_pointer,
        "Event Ring Dequeue Pointer",
        u64
    );
}
impl_debug_from_methods! {
    EventRingDequeuePointer {
        dequeue_erst_segment_index,
        dequeue_pointer,
    }
}

/// Debug Capability Control Register.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Control(u32);
impl Control {
    ro_bit!(pub, self, self.0; 0, dbc_run, "DbC Run");
    rw_bit!(pub, self, self.0; 1, link_status_event_enable, "Link Status Event Enable");
    rw1s_bit!(pub, self, self.0; 2, halt_out_tr, "Halt OUT TR");
    rw1s_bit!(pub, self, self.0; 3, halt_in_tr, "Halt IN TR");
    rw1c_bit!(pub, self, self.0; 4, dbc_run_change, "DbC Run Change");
    ro_field!(pub, self, self.0; 16..=23, debug_max_burst_size, "Debug Max Burst Size", u8);
    ro_field!(pub, self, self.0; 24..=30, device_address, "Device Address", u8);
    rw_bit!(pub, self, self.0; 31, debug_capability_enable, "Debug Capability Enable");
}
impl_debug_from_methods! {
    Control {
        dbc_run,
        link_status_event_enable,
        halt_out_tr,
        halt_in_tr,
        dbc_run_change,
        debug_max_burst_size,
        device_address,
        debug_capability_enable,
    }
}

/// Debug Capability Status Register.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Status(u32);
impl Status {
    ro_bit!(pub, self, self.0; 0, event_ring_not_empty, "Event Ring Not Empty");
    ro_bit!(pub, self, self.0; 1, dbc_system_bus_reset, "DbC System Bus Reset");
    ro_field!(pub, self, self.0; 24..=31, debug_port_number, "Debug Port Number", u8);
}
impl_debug_from_methods! {
    Status {
        event_ring_not_empty,
        dbc_system_bus_reset,
        debug_port_number,
    }
}

/// Debug Capability Port Status and Control Register.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct PortStatusAndControl(u32);
impl PortStatusAndControl {
    ro_bit!(pub, self, self.0; 0, current_connect_status, "Current Connect Status");
    rw_bit!(pub, self, self.0; 1, port_enabled_disabled, "Port Enabled/Disabled");
    ro_bit!(pub, self, self.0; 4, port_reset, "Port Reset");
    rw_field!(pub, self, self.0; 5..=8, port_link_state, "Port Link State", u8);
    ro_field!(pub, self, self.0; 10..=13, port_speed, "Port Speed", u8);
    rw1c_bit!(pub, self, self.0; 17, connect_status_change, "Connect Status Change");
    rw1c_bit!(pub, self, self.0; 21, port_reset_change, "Port Reset Change");
    rw1c_bit!(pub, self, self.0; 22, port_link_status_change, "Port Link Status Change");
    rw1c_bit!(pub, self, self.0; 23, port_config_error_change, "Port Config Error Change");
}
impl_debug_from_methods! {
    PortStatusAndControl {
        current_connect_status,
        port_enabled_disabled,
        port_reset,
        port_link_state,
        port_speed,
        connect_status_change,
        port_reset_change,
        port_link_status_change,
        port_config_error_change,
    }
}

/// Debug Capability Context Pointer Register.
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct ContextPointer(u64);
impl ContextPointer {
    rw_zero_trailing!(
        pub, self,
        self.0; 4~; "16-byte aligned",
        "Debug Capability Context Base Pointer",
        u64
    );
}

/// Debug Capability Device Descriptor Info Register 1
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct DeviceDescriptorInfo1(u32);
impl DeviceDescriptorInfo1 {
    rw_field!(pub, self, self.0; 0..=7, dbc_protocol, "DbC Protocol", u8);
    rw_field!(pub, self, self.0; 16..=31, vendor_id, "Vendor ID", u16);
}
impl_debug_from_methods! {
    DeviceDescriptorInfo1 {
        dbc_protocol,
        vendor_id,
    }
}

/// Debug Capability Device Descriptor Info Register 2.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct DeviceDescriptorInfo2(u32);
impl DeviceDescriptorInfo2 {
    rw_field!(pub, self, self.0; 0..=15, product_id, "Product ID", u16);
    rw_field!(pub, self, self.0; 16..=31, device_revision, "Device Revision", u16);
}
impl_debug_from_methods! {
    DeviceDescriptorInfo2 {
        product_id,
        device_revision,
    }
}
