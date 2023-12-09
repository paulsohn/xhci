//! Host Controller Runtime Registers.

/// Runtime Registers
///
/// Note that this struct does not contain the interrupter register sets. Refer to
/// [`InterrupterRegisterSet`].
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Runtime {
    /// Microframe Index Register
    pub mfindex: MicroframeIndexRegister, // off 0x00
    _padding_04_1f: [u32; 3],
}

/// Microframe Index Register
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct MicroframeIndexRegister(u32);
impl MicroframeIndexRegister {
    ro_field!(pub, self, self.0; 0..=13, microframe_index, "Microframe Index", u16);
}
impl_debug_from_methods! {
    MicroframeIndexRegister {
        microframe_index,
    }
}

/// Interrupter Register Set
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct InterrupterRegisterSet {
    /// Interrupter Management Register
    pub iman: InterrupterManagementRegister, // off 0x00
    /// Interrupter Moderation Register
    pub imod: InterrupterModerationRegister, // 0ff 0x04
    /// Event Ring Segment Table Size Register
    pub erstsz: EventRingSegmentTableSizeRegister, // off 0x08
    _padding_0c_0f: u32,
    /// Event Ring Segment Table Base Address Register
    pub erstba: EventRingSegmentTableBaseAddressRegister, // off 0x10
    /// Event Ring Dequeue Pointer Register
    pub erdp: EventRingDequeuePointerRegister, // off 0x18
}

/// Interrupter Management Register.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct InterrupterManagementRegister(u32);
impl InterrupterManagementRegister {
    rw1c_bit!(pub, self, self.0; 0, interrupt_pending, "Interrupt Pending");
    rw_bit!(pub, self, self.0; 1, interrupt_enable, "Interrupt Enable");
}
impl_debug_from_methods! {
    InterrupterManagementRegister {
        interrupt_pending,
        interrupt_enable,
    }
}

/// Interrupter Moderation Register.
#[repr(transparent)]
#[derive(Copy, Clone, Default)]
pub struct InterrupterModerationRegister(u32);
impl InterrupterModerationRegister {
    rw_field!(
        pub, self,
        self.0; 0..=15,
        interrupt_moderation_interval,
        "Interrupt Moderation Interval",
        u16
    );
    rw_field!(
        pub, self,
        self.0; 16..=31,
        interrupt_moderation_counter,
        "Interrupt Moderation Counter",
        u16
    );
}
impl_debug_from_methods! {
    InterrupterModerationRegister{
        interrupt_moderation_interval,
        interrupt_moderation_counter,
    }
}

/// Event Ring Segment Table Size Register.
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct EventRingSegmentTableSizeRegister(u32);
impl EventRingSegmentTableSizeRegister {
    rw_field!(
        pub, self,
        self.0; 0..=15,
        "Event Ring Segment Table Size (the number of segments)",
        u16
    );
}

/// Event Ring Segment Table Base Address Register.
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct EventRingSegmentTableBaseAddressRegister(u64);
impl EventRingSegmentTableBaseAddressRegister {
    rw_zero_trailing!(
        pub, self,
        self.0; 6~; "64-byte aligned",
        "Event Ring Segment Table Base Address",
        u64
    );
}

/// Event Ring Dequeue Pointer Register.
#[repr(transparent)]
#[derive(Copy, Clone, Default)]
pub struct EventRingDequeuePointerRegister(u64);
impl EventRingDequeuePointerRegister {
    rw_field!(
        pub, self,
        self.0; 0..=2,
        dequeue_erst_segment_index,
        "Dequeue ERST Segment Index",
        u8
    );
    rw1c_bit!(pub, self, self.0; 3, event_handler_busy, "Event Handler Busy");
    rw_zero_trailing!(
        pub, self,
        self.0; 4~; "16-byte aligned",
        event_ring_dequeue_pointer,
        "current Event Ring Dequeue Pointer",
        u64
    );
}
impl_debug_from_methods! {
    EventRingDequeuePointerRegister{
        dequeue_erst_segment_index,
        event_handler_busy,
        event_ring_dequeue_pointer
    }
}
