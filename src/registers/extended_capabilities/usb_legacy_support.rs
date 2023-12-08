//! USB Legacy Support Capability

// use volatile::VolatilePtr;

/// USB Legacy Support Capability.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct UsbLegacySupport {
    /// The Capability Header part.
    pub cap_header: CapabilityHeader,
    /// USB Legacy Support Control/Status.
    pub usblegctlsts: UsbLegacySupportControlStatus,
}

/// The Capability Header part of the USB Legacy Support Capability.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct CapabilityHeader(u32);
impl CapabilityHeader {
    rw_bit!(pub, self, self.0; 16, hc_bios_owned_semaphore, "HC BIOS Owned Semaphore");
    rw_bit!(pub, self, self.0; 24, hc_os_owned_semaphore, "HC OS Owned Semaphore");
}
impl_debug_from_methods! {
    CapabilityHeader {
        hc_bios_owned_semaphore,
        hc_os_owned_semaphore,
    }
}

/// USB Legacy Support Control/Status.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct UsbLegacySupportControlStatus(u32);
impl UsbLegacySupportControlStatus {
    rw_bit!(pub, self, self.0; 0, usb_smi_enable, "USB SMI Enable");
    rw_bit!(
        pub, self,
        self.0; 4,
        smi_on_host_system_error_enable,
        "SMI on Host System Error Enable"
    );
    rw_bit!(pub, self, self.0; 13, smi_on_os_ownership_enable, "SMI on OS Ownership Enable");
    rw_bit!(pub, self, self.0; 14, smi_on_pci_command_enable, "SMI on PCI Command Enable");
    rw_bit!(pub, self, self.0; 15, smi_on_bar_enable, "SMI on BAR Enable");
    ro_bit!(pub, self, self.0; 16, smi_on_event_interrupt, "SMI on Event Interrupt");
    ro_bit!(pub, self, self.0; 20, smi_on_host_system_error, "SMI on Host System Error");
    rw1c_bit!(pub, self, self.0; 29, smi_on_os_ownership_change, "SMI on OS Ownership Change");
    rw1c_bit!(pub, self, self.0; 30, smi_on_pci_command, "SMI on PCI Command");
    rw1c_bit!(pub, self, self.0; 31, smi_on_bar, "SMI on BAR");
}
impl_debug_from_methods! {
    UsbLegacySupportControlStatus {
        usb_smi_enable,
        smi_on_host_system_error_enable,
        smi_on_os_ownership_enable,
        smi_on_pci_command_enable,
        smi_on_bar_enable,
        smi_on_event_interrupt,
        smi_on_host_system_error,
        smi_on_os_ownership_change,
        smi_on_pci_command,
        smi_on_bar,
    }
}
