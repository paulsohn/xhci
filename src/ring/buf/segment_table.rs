//! Event Ring Segment Table Entry.

use super::block::Block;
use bit_field::BitField;
use core::ops::{Index, IndexMut};

/// The Event Ring Segment Table entry.
/// This plays the same role as an array pointer, and require special care to guarantee memory safety.
///
/// For example, the entry do not implement `Drop` trait, so the user should manually free its memory.
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct EventRingSegmentTableEntry([u32; 4]);

impl EventRingSegmentTableEntry {
    /// Create new segment table entry from a block buffer.
    /// 
    /// # Panics
    ///
    /// This method will panic if `len >= 4096`.
    pub unsafe fn from_buf(buf: &[Block]) -> Self {
        assert!(buf.len() <= u16::MAX as usize);

        let mut entry = Self([0; 4]);
        entry
            .set_ring_segment_base_address(buf.as_ptr() as usize as u64)
            .set_ring_segment_size(buf.len() as u16);
        entry
    }

    /// Returns the entry count of the segment.
    pub fn len(&self) -> usize {
        return self.ring_segment_size() as usize;
    }

    /// Returns the slice that this entry is representing.
    pub fn as_slice(&self) -> &[Block] {
        unsafe {
            let base = self.ring_segment_base_address() as *const _;
            let len = self.len();

            core::slice::from_raw_parts(base, len)
        }
    }

    /// Returns the mutable slice that this entry is representing.
    pub fn as_mut_slice(&mut self) -> &mut [Block] {
        unsafe {
            let base = self.ring_segment_base_address() as *mut _;
            let len = self.len();

            core::slice::from_raw_parts_mut(base, len)
        }
    }
}

impl EventRingSegmentTableEntry {
    /// Returns the value of the Ring Segment Base Address field.
    pub unsafe fn ring_segment_base_address(&self) -> u64 {
        let l: u64 = self.0[0].into();
        let u: u64 = self.0[1].into();

        (u << 32) | l
    }

    /// Sets the value of the Ring Segment Base Address field.
    ///
    /// # Panics
    ///
    /// This method panics if `p` is not 64-byte aligned.
    pub unsafe fn set_ring_segment_base_address(&mut self, p: u64) -> &mut Self {
        assert_eq!(
            p % 64,
            0,
            "The Ring Segment Base Address must be 64-byte aligned."
        );

        let l = p.get_bits(0..32);
        let u = p.get_bits(32..64);

        self.0[0] = l.try_into().unwrap();
        self.0[1] = u.try_into().unwrap();
        self
    }

    /// Returns the value of the Ring Segment Size field.
    ///
    /// This field represents entry count.
    pub fn ring_segment_size(&self) -> u16 {
        self.0[2].get_bits(0..16) as _
    }
    /// Sets the value of the Ring Segment Size field.
    ///
    /// The value should be entry count.
    pub unsafe fn set_ring_segment_size(&mut self, v: u16) -> &mut Self {
        self.0[2].set_bits(0..16, v.into());
        self
    }
    // rw_field!([2](0..16), ring_segment_size, "Ring Segment Size", u16);

    /// Returns the value of the ring segment end address.
    pub unsafe fn ring_segment_bound_address(&self) -> u64 {
        self.ring_segment_base_address() + (core::mem::size_of::<Block>() * self.ring_segment_size() as usize) as u64
    }
}

impl Index<usize> for EventRingSegmentTableEntry {
    type Output = Block;

    fn index(&self, index: usize) -> &Self::Output {
        let slice = self.as_slice();
        assert!(index < slice.len());

        &slice[index]
    }
}

impl IndexMut<usize> for EventRingSegmentTableEntry {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let slice = self.as_mut_slice();
        assert!(index < slice.len());

        &mut slice[index]
    }
}
