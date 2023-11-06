use std::alloc::Layout;
use std::ptr::NonNull;
use std::{alloc, mem};

mod array_list;
mod ring_buffer;
pub use array_list::ArrayList;
pub use ring_buffer::RingBuffer;

struct RawVec<T> {
    ptr: NonNull<T>,
    cap: usize,
}

unsafe impl<T: Send> Send for RawVec<T> {}
unsafe impl<T: Sync> Sync for RawVec<T> {}

impl<T> RawVec<T> {
    fn new() -> Self {
        let cap = if mem::size_of::<T>() == 0 {
            usize::MAX
        } else {
            0
        };
        RawVec {
            ptr: NonNull::dangling(),
            cap,
        }
    }

    fn with_capacity(cap: usize) -> Self {
        let cap = if mem::size_of::<T>() == 0 {
            usize::MAX
        } else {
            cap
        };
        let layout = Layout::array::<T>(cap).unwrap();
        assert!(layout.size() <= isize::MAX as usize, "Allocation too large");
        let ptr = unsafe { alloc::alloc(layout) };
        let ptr = match NonNull::new(ptr as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(layout),
        };
        RawVec { ptr, cap }
    }

    fn grow(&mut self) {
        assert!(mem::size_of::<T>() != 0, "capacity overflow");
        let new_cap = if self.cap == 0 { 1 } else { 2 * self.cap };
        let new_layout = Layout::array::<T>(new_cap).unwrap();
        assert!(
            new_layout.size() <= isize::MAX as usize,
            "Allocation too large"
        );
        let new_ptr = if self.cap == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
        };
        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };
        self.cap = new_cap;
    }
}

impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
        let elem_size = mem::size_of::<T>();
        if self.cap != 0 && elem_size != 0 {
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}
