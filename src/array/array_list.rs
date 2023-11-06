use std::{
    ops::{Deref, DerefMut},
    ptr::{self},
};
use super::RawVec;

pub struct ArrayList<T> {
    buf: RawVec<T>,
    len: usize,
}

impl<T> ArrayList<T> {
    fn ptr(&self) -> *mut T {
        self.buf.ptr.as_ptr()
    }

    fn cap(&self) -> usize {
        self.buf.cap
    }

    pub fn new() -> Self {
        ArrayList {
            buf: RawVec::new(),
            len: 0,
        }
    }

    pub fn push(&mut self, elem: T) {
        if self.len == self.cap() {
            self.buf.grow()
        };
        unsafe {
            ptr::write(self.ptr().add(self.len), elem);
        }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe { Some(ptr::read(self.ptr().add(self.len))) }
        }
    }

    pub fn insert(&mut self, idx: usize, elem: T) {
        assert!(idx <= self.len, "index out of bounds");
        if self.len == self.cap() {
            self.buf.grow();
        }
        unsafe {
            ptr::copy(self.ptr().add(idx), self.ptr().add(idx + 1), self.len - idx);
            ptr::write(self.ptr().add(idx), elem);
            self.len += 1;
        }
    }

    pub fn remove(&mut self, idx: usize) -> Option<T> {
        assert!(idx < self.len, "index out of bounds");
        if self.len == 0 {
            None
        } else {
            unsafe {
                self.len -= 1;
                let result = ptr::read(self.ptr().add(idx));
                ptr::copy(self.ptr().add(idx + 1), self.ptr().add(idx), self.len - idx);
                Some(result)
            }
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<T> Drop for ArrayList<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

impl<T> Deref for ArrayList<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.ptr(), self.len) }
    }
}

impl<T> DerefMut for ArrayList<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.ptr(), self.len) }
    }
}

unsafe impl<T: Send> Send for ArrayList<T> {}
unsafe impl<T: Sync> Sync for ArrayList<T> {}

#[cfg(test)]
mod tests {
    use super::ArrayList;

    #[test]
    fn test_ring_buffer() {
        let mut al = ArrayList::new();
        assert_eq!(al.pop(), None);
        assert!(al.is_empty());
        al.push(10); al.push(15); al.push(20); al.push(25);
        assert_eq!(al.len(), 4);
        assert_eq!(al.remove(0), Some(10));
        assert_eq!(al.remove(0), Some(15));
        al.insert(0, 30);
        assert_eq!(al.len(), 3);
        assert_eq!(al.pop(), Some(25));
        assert_eq!(al.pop(), Some(20));
        assert_eq!(al.pop(), Some(30));
        assert!(al.is_empty());
    }
}
