//! A multi-buffer implementations of command ring, transfer ring, and event ring.

extern crate alloc;

use core::mem::ManuallyDrop;

// the allocator should not only allocate the memory,
// but should also map it into the virtual address and return it.
use alloc::alloc::{Allocator, Layout};
use alloc::boxed::Box;
use alloc::vec::Vec;

/// The ring buffer base alignment.
pub const RING_BUF_BASE_ALIGN: usize = 64;

fn alloc_buf<T, A>(size: usize, allocator: A, value: T) -> Box<[T], A>
where
    T: Copy,
    A: Allocator,
{
    unsafe {
        let ptr = allocator.allocate(
            Layout::from_size_align(size * core::mem::size_of::<T>(),
            RING_BUF_BASE_ALIGN).unwrap()
        )
        .ok()
        .map(|ptr| ptr.as_ptr() /* .as_mut_ptr() */ as *mut u8 as *mut T)
        .unwrap_or(core::ptr::null_mut());

        let buf_ptr = core::ptr::slice_from_raw_parts_mut(ptr, size);

        (&mut *buf_ptr).fill(value);

        Box::from_raw_in(
            buf_ptr,
            allocator
        )
    }
}

unsafe fn boxed_slice_from_slice_in<T, A>(slice: &mut [T], allocator: A) -> Box<[T], A>
where
    A: Allocator,
{
    let len = slice.len();
    Vec::<T, A>::from_raw_parts_in(slice.as_mut_ptr(), len, len, allocator).into_boxed_slice()
}

// todo: integrate block and trb modules

pub mod block;
use block::Block;

#[allow(type_alias_bounds)]
type Segment<A: Allocator> = Box<[Block], A>;

use super::trb;

macro_rules! add_pushable_ring {
    ($ring_name:ident, $ring_ty:ident) => {
        paste::paste! {
            #[doc = "A " $ring_ty " ring."]
            #[derive(Debug)]
            pub struct $ring_name<A>
            where
                A: Allocator + Clone + 'static,
            {
                /// Ring segments.
                segs: Vec<Segment<A>, A>,
                /// Current segment index for push.
                seg_cur: usize,
                /// Current block index for push.
                block_cur: usize,
                /// Current cycle bit.
                cycle_bit: bool,
                /// The default allocator for this ring.
                allocator: A,
            }
        }

        impl<A> $ring_name<A>
        where
            A: Allocator + Clone + 'static,
        {
            /// Create an uninitialized ring with no segments allocated.
            pub fn new_uninit(allocator: A) -> Self {
                Self {
                    segs: Vec::new_in(allocator.clone()),
                    seg_cur: 0,
                    block_cur: 0,
                    cycle_bit: true,
                    allocator,
                }
            }

            /// Returns if the ring has any buffer to use.
            pub fn is_init(&self) -> bool {
                self.segs.len() > 0
            }

            /// Create a new ring initialized with single segment of size `size`.
            pub fn new(size: usize, allocator: A) -> Self {
                let mut r = Self::new_uninit(allocator);

                // Add the first segment.
                // `seg_cur`, `block_cur` and `cycle_bit` is already ready-to-go.
                r.add_segment(size);
                r
            }

            /// Get the current pointer.
            unsafe fn get_ptr(&self) -> *mut Block {
                self.segs[self.seg_cur].as_ptr()
                    .add(self.block_cur) as _ // transmute to mut ptr
            }

            /// Write a block into current enqueue pointer, with respect to current cycle bit.
            /// Returns the pointer on which the block was written.
            fn write_block_with_cycle_bit(&mut self, mut block: Block) {
                if self.cycle_bit {
                    block.set_cycle_bit();
                } else {
                    block.clear_cycle_bit();
                }

                unsafe {
                    self.get_ptr().write_volatile(block);
                }
            }

            /// Push a block into the ring.
            /// Returns the previous enqueue pointer, as the pair of the segment and block indices the block was put.
            pub fn push(&mut self, block: Block) -> *const Block {
                assert!(self.is_init());

                // this will assert that block type is one of the allowed types
                let _allowed =
                    crate::ring::trb::$ring_ty::Allowed::try_from(block.into_raw()).unwrap();

                let last_ptr = unsafe { self.get_ptr() };

                // push the desired block.
                self.write_block_with_cycle_bit(block);
                self.block_cur += 1;

                // if next block is the last block of the segment,
                // push a link TRB.
                if self.block_cur == self.segs[self.seg_cur].len() - 1 {
                    let seg_next = if self.seg_cur == self.segs.len() - 1 {
                        0
                    } else {
                        self.seg_cur + 1
                    };
                    let toggle_cond = (seg_next == 0);

                    let link = Block::new(
                        {
                            let seg_next_base = self.segs[seg_next].as_mut_ptr();
                            let mut link = *trb::Link::new()
                                .set_ring_segment_pointer(seg_next_base as usize as u64);
                            if toggle_cond {
                                link.set_toggle_cycle();
                            }
                            link
                        }
                        .into_raw(),
                    );

                    self.write_block_with_cycle_bit(link);
                    self.seg_cur = seg_next;
                    self.block_cur = 0;
                    if toggle_cond {
                        self.cycle_bit ^= true;
                    }
                }

                last_ptr
            }

            /// Add a new segment with size `size` into the ring.
            /// Never call this on initialized ring if you want it single-segmented.
            pub fn add_segment(&mut self, size: usize) {
                self.segs.push(alloc_buf(
                    size,
                    self.allocator.clone(),
                    Block::zero_with_cycle_bit(!self.cycle_bit),
                ));
            }

            /// Get the buffer base pointer of `i`th segment.
            pub unsafe fn get_buf_ptr(&self, i: usize) -> *const Block {
                assert!(i < self.segs.len());

                self.segs[i].as_ptr()
            }
        }
    };
}

add_pushable_ring!(CommandRing, command);
add_pushable_ring!(TransferRing, transfer);

pub mod segment_table;
use segment_table::EventRingSegmentTableEntry;

use crate::registers::runtime::InterrupterRegisterSet;
// use accessor::array::{self, BoundedStructural, BoundedStructuralMut};
use accessor::single;

/// A single-buffer implementation of event ring.
#[allow(missing_debug_implementations)]
pub struct EventRing<A, M>
where
    A: Allocator + Clone + 'static,
    M: accessor::mapper::Mapper,
{
    /// The Interrupter associated to the event ring.
    interrupter: single::ReadWrite<InterrupterRegisterSet, M>,
    /// The ring segments, in the form of `EventRingSegmentTableEntry` rather than slices.
    seg_table: Vec<EventRingSegmentTableEntry, A>,
    /// The vector of allocators. Required to manually drop ring segments.
    allocators: Vec<ManuallyDrop<A>, A>,
    /// Current cycle bit.
    cycle_bit: bool,
    /// The default allocator for this ring.
    allocator: A,
}

impl<A, M> EventRing<A, M>
where
    A: Allocator + Clone + 'static,
    M: accessor::mapper::Mapper,
{
    /// Set event ring segment index and dequeue pointer.
    fn set_erdp(&mut self, seg_idx: u8, dequeue_pointer: u64) {
        use accessor::single::BoundedStructuralMut;

        self.interrupter.structural_mut().erdp.update_volatile(|erdp| {
            erdp
                // .clear_event_handler_busy()
                .set_dequeue_erst_segment_index(seg_idx)
                .set_event_ring_dequeue_pointer(dequeue_pointer);
        });
    }

    /// Create an uninitialized Event Ring from interrupter and the allocator.
    /// No buffers are allocated. This method exists only for completeness.
    #[deprecated]
    pub fn new_uninit(
        interrupter: single::ReadWrite<InterrupterRegisterSet, M>,
        allocator: A,
    ) -> Self {
        Self {
            interrupter,
            seg_table: Vec::new_in(allocator.clone()),
            allocators: Vec::new_in(allocator.clone()),
            cycle_bit: true,
            allocator,
        }
    }

    /// Returns if the ring has any buffer to use.
    #[deprecated]
    pub fn is_init(&self) -> bool {
        self.seg_table.len() > 0
    }

    /// Create a new Event Ring from interrupter, first buffer size, and the allocator.
    pub fn new(
        interrupter: single::ReadWrite<InterrupterRegisterSet, M>,
        size: usize,
        allocator: A,
    ) -> Self {
        #[allow(deprecated)]
        let mut er = Self::new_uninit(interrupter, allocator);

        // Add the first segment.
        er.add_segment(size);

        // initialize dequeue pointer register.
        // let base = er.seg_table[0].as_mut_slice().as_mut_ptr() as usize as u64;
        // er.set_erdp(0, base);

        er
    }

    /// Add a new segment with size `size` into the ring.
    /// Never call this if you want a single-segmented ring.
    ///
    /// # Panics
    ///
    /// This method panics if the ring already has 255 segments.
    pub fn add_segment(&mut self, size: usize) {
        assert!(self.seg_table.len() <= u8::MAX as usize);

        // allocate a new segment.
        let seg: Segment<A> = alloc_buf(
            size,
            self.allocator.clone(),
            Block::zero_with_cycle_bit(!self.cycle_bit),
        );

        let (ptr, al) = Box::into_raw_with_allocator(seg);
        let buf = unsafe { &*ptr };
        self.seg_table
            .push(unsafe { EventRingSegmentTableEntry::from_buf(buf) });
        self.allocators.push(ManuallyDrop::new(al));

        // todo: can we guarantee that seg_table base is 64-byte alligned?

        // update interrupter registers.
        let base = self.seg_table.as_mut_ptr() as usize as u64;
        let len = self.seg_table.len() as u16;

        if len == 1 { // init. set ERDP here.
            self.set_erdp(0, buf.as_ptr() as usize as u64);
        }

        use accessor::single::BoundedStructuralMut;
        let mut intr = self.interrupter.structural_mut();

        // event ring is enabled by erstba write. erstba should be updated last.
        intr.erstsz.update_volatile(|erstsz| erstsz.set(len));
        intr.erstba.update_volatile(|erstba| erstba.set(base));
    }

    /// Performs dequeue operation, and returns the block if not empty.
    /// 
    /// # Panics
    /// 
    /// This method panics if there are no buffers available.
    pub fn pop(&mut self) -> Option<Block> {
        assert!({
            #[allow(deprecated)]
            self.is_init()
        });

        // Get the dequeue pointer.
        let (seg_cur, dq_ptr) = {
            use accessor::single::BoundedStructural;
            let erdp = self.interrupter.structural().erdp.read_volatile();
            (
                erdp.dequeue_erst_segment_index() as usize,
                erdp.event_ring_dequeue_pointer() as usize as *const Block,
            )
        };

        // Get the front block.
        let front = unsafe { dq_ptr.read_volatile() };

        // Check whether the block should be consumed.
        if front.cycle_bit() == self.cycle_bit {
            // Increment the current dequeue pointer.
            let incremented = unsafe { dq_ptr.add(1) } as usize as u64;
            let bound = unsafe { self.seg_table[seg_cur].ring_segment_bound_address() };

            // Determine the new segment index and dequeue pointer.
            let (seg_next, new_dq_pos) = if incremented == bound {
                // Incremented ptr has reached the bound.
                // Flip the cycle bit and move to the next (or front) segment.
                let seg_next = if seg_cur == self.seg_table.len() - 1 {
                    self.cycle_bit ^= true;
                    0
                } else {
                    seg_cur + 1
                };
                let seg_next_base = unsafe { self.seg_table[seg_next].ring_segment_base_address() };

                (seg_next, seg_next_base)
            } else {
                (seg_cur, incremented)
            };

            // Update dequeue pointer register.
            self.set_erdp(seg_next as u8, new_dq_pos);

            Some(front)
        } else {
            None
        }
    }

    // /// Performs dequeue operation, and returns the block if not empty.
    // /// Discriminate the TRB block in respect to its type.
    // pub fn pop_and_discriminate(&mut self) -> Option<trb::event::Allowed> {
    //     self.pop()
    //         .map(|block| block.into_raw().try_into().ok())
    //         .flatten()
    // }
}

impl<A, M> Drop for EventRing<A, M>
where
    A: Allocator + Clone + 'static,
    M: accessor::mapper::Mapper,
{
    fn drop(&mut self) {
        let allocators = self.allocators.iter_mut();
        for (erst, al) in self.seg_table.iter_mut().zip(allocators) {
            let a = unsafe { ManuallyDrop::take(al) };
            let _b = unsafe { boxed_slice_from_slice_in(erst.as_mut_slice(), a) };
            // `_b` is dropped, so that the allocated slice from the erst entry is freed.
        }
        // `self.seg_table` and `self.allocators` are also dropped.
    }
}