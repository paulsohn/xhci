//! Doorbell Register

/// A type alias to [`Doorbell`] register for backward compability.
#[deprecated = "Use `Doorbell` instead of `Register`."]
pub type Register = Doorbell;

/// The element of the Doorbell Array.
#[repr(transparent)]
#[derive(Copy, Clone, Default)]
pub struct Doorbell(u32);
impl Doorbell {
    rw_field!(pub, self, self.0; 0..=7, doorbell_target, "Doorbell Target", u8);
    rw_field!(pub, self, self.0; 16..=31, doorbell_stream_id, "Doorbell Stream ID", u16);
}
impl core::fmt::Debug for Doorbell {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("doorbell::Register")
            .field("doorbell_target", &self.doorbell_target())
            .field("doorbell_stream_id", &self.doorbell_stream_id())
            .finish()
    }
}
