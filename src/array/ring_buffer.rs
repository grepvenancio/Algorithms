use std::ptr;
use super::RawVec;

pub struct RingBuffer<T> {
    buf: RawVec<T>,
    head: usize,
    len: usize,
}


unsafe impl<T: Send> Send for RingBuffer<T> {}
unsafe impl<T: Sync> Sync for RingBuffer<T> {}

impl<T> RingBuffer<T> {
    pub fn new() -> Self {
        RingBuffer { buf: RawVec::new(), head: 0, len: 0 }
    }

    pub fn with_capacity(cap: usize) -> Self {
        RingBuffer { buf: RawVec::with_capacity(cap), head: 0, len: 0 }
    }

    fn is_full(&self) -> bool {
        self.len == self.cap()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn cap(&self) -> usize {
        self.buf.cap
    }

    fn ptr(&self) -> *mut T {
        self.buf.ptr.as_ptr()
    }

    pub fn push_back(&mut self, elem: T) {
        if self.is_full() { self.buf.grow() }
        unsafe {
            ptr::write(self.ptr().add(self.len % self.cap()), elem)
        }
        self.len += 1;
    }

    pub fn push_front(&mut self, elem: T) {
        if self.is_full() { self.buf.grow() }
        if !self.is_empty() {
            self.head -= 1;
        }
        unsafe {
            ptr::write(self.ptr().add(self.head % self.cap()), elem)
        }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            let elem = unsafe {
                ptr::read(self.ptr().add(self.head % self.cap()))
            };
            self.len -= 1;
            self.head += 1;
            Some(elem)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RingBuffer;

    #[test]
    fn test_ring_buffer() {
        let mut rg = RingBuffer::new();
        assert_eq!(rg.pop(), None);
        assert!(rg.is_empty());
        rg.push_back(10); rg.push_back(15); rg.push_back(20); rg.push_back(25);
        assert_eq!(rg.len(), 4);
        assert_eq!(rg.pop(), Some(10));
        assert_eq!(rg.pop(), Some(15));
        rg.push_front(30);
        assert_eq!(rg.len(), 3);
        assert_eq!(rg.pop(), Some(30));
        assert_eq!(rg.pop(), Some(20));
        assert_eq!(rg.pop(), Some(25));
        assert!(rg.is_empty());
    }
}
