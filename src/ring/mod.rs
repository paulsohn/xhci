//! TRB Ring.

extern crate alloc;
#[allow(unused_imports)]
use alloc as _alloc;

pub mod trb;
pub mod erst;

pub use erst::EventRingSegmentTableEntry;