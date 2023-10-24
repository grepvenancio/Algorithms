use std::{marker::PhantomData, ptr::NonNull};

/// A Queue is a data structure that store 2 pointers, one to the head of the list and other to
/// the tail of it, you can interact with a queue by inserting a node to the tail
/// and remove one from the head.
///
///  (A) -> (B) -> (C) -> (D)
///  head                 tail
pub struct Queue<T> {
    head: Link<T>,
    tail: Link<T>,
    length: usize,
    // Semantic implies that we own T, even if we don't.
    _boo: PhantomData<T>,
}

pub struct IntoIter<T> {
    queue: Queue<T>,
}

pub struct Iter<'a, T> {
    head: Link<T>,
    tail: Link<T>,
    length: usize,
    _boo: PhantomData<&'a T>,
}

pub struct IterMut<'a, T> {
    head: Link<T>,
    tail: Link<T>,
    length: usize,
    _boo: PhantomData<&'a mut T>,
}

struct Node<T> {
    next: Link<T>,
    elem: T,
}

// NonNoll is covarint over T hence the reason why we use it instead of a *mut T
type Link<T> = Option<NonNull<Node<T>>>;

impl<T> Queue<T> {
    /// Create a empty Queue.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use algo_front_end_masters::linked_list::Queue;
    /// let mut queue: Queue<String> = Queue::new();
    /// assert_eq!(queue.len(), 0);
    /// ```
    pub fn new() -> Self {
        Queue {
            head: None,
            tail: None,
            length: 0,
            _boo: PhantomData,
        }
    }

    /// Insert a element to the tail of the queue.
    ///
    /// # Examples
    ///
    ///```rust    
    /// # use algo_front_end_masters::linked_list::Queue;
    /// let mut queue = Queue::new();
    /// queue.enqueue("Hello");
    ///```
    pub fn enqueue(&mut self, elem: T) {
        // SAFETY: We know that is ok to invoke a new_unchecked() because we are inputing a non null
        // *mut T, which make sound to dereference the old tail, as we know it contains a non null
        // pointer.
        unsafe {
            // Create a non null pointer to a heap allocated Node<T>
            let new_tail =
                NonNull::new_unchecked(Box::into_raw(Box::new(Node { elem, next: None })));
            // Check if the there is Some(Link<T>)
            if let Some(old) = self.tail {
                // Set the next field from the old tail to point to our new tail
                (*old.as_ptr()).next = Some(new_tail);
            } else {
                // If the tail is None it means of queue is empty so we need to set the head of
                // our queue to the new tail
                self.head = Some(new_tail);
            }
            // Increase the queue length
            self.length += 1;
            // Set the tail of out queue to the new tail
            self.tail = Some(new_tail);
        }
    }

    /// Remove and returns an element from the head of the queue.
    ///
    /// # Examples
    ///
    ///```rust
    /// # use algo_front_end_masters::linked_list::Queue;
    /// let mut queue = Queue::new();
    /// queue.enqueue("Hello World!");
    /// assert_eq!(queue.deque(), Some("Hello World!"));
    ///```
    pub fn deque(&mut self) -> Option<T> {
        // SAFETY: It's okay to create a box from a *mut T because there is no way
        // to inseart a null pointer to our queue, so the head points to a valid memory
        // address.
        unsafe {
            self.head.map(|head| {
                // Create a owned box from a raw pointer
                let boxed_head = Box::from_raw(head.as_ptr());
                // Set the head of our list to the value contained in the next of our owned box
                self.head = boxed_head.next;
                // If our new head is None our list is empty so we set the tail to None
                if self.head.is_none() {
                    self.tail = None
                }
                // Decrease the queue length
                self.length -= 1;
                // Returns the element contained in our box
                boxed_head.elem
                // our old head is dropped at the end of this scope as the value is owned by
                // boxed_head
            })
        }
    }

    /// Returns a shared reference to the head of the queue.
    ///
    /// # Examples
    ///
    ///```rust
    /// # use algo_front_end_masters::linked_list::Queue;
    /// let mut queue = Queue::new();
    /// queue.enqueue("Hello World!");
    /// assert_eq!(queue.peek(), Some(&"Hello World!"));
    ///```
    pub fn peek(&self) -> Option<&T> {
        // SAFETY: It's okay dereference head because there is no way
        // to inseart a null pointer to our queue, so the head points to a valid memory
        // address.
        unsafe { self.head.map(|head| &(*head.as_ptr()).elem) }
    }

    /// Returns a mut reference to the head of the queue.
    ///
    /// # Examples
    ///
    ///```rust
    /// # use algo_front_end_masters::linked_list::Queue;
    /// let mut queue = Queue::new();
    /// queue.enqueue("Hello World!");
    /// queue.peek_mut().map(|e| *e = "Hello");
    /// assert_eq!(queue.peek_mut(), Some(&mut "Hello"));
    ///```
    pub fn peek_mut(&self) -> Option<&mut T> {
        // SAFETY: It's okay dereference head because there is no way
        // to inseart a null pointer to our queue, so the head points to a valid memory
        // address.
        unsafe { self.head.map(|head| &mut (*head.as_ptr()).elem) }
    }

    /// Returns the length of the queue.
    pub fn len(&self) -> usize {
        self.length
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            head: self.head,
            tail: self.tail,
            length: self.length,
            _boo: PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            head: self.head,
            tail: self.tail,
            length: self.length,
            _boo: PhantomData,
        }
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        while let Some(_) = self.deque() {}
    }
}

impl<T> IntoIterator for Queue<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter { queue: self }
    }
}

impl<'a, T> IntoIterator for &'a Queue<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Queue<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.queue.deque()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.head.map(|head| unsafe {
            self.head = (*head.as_ptr()).next;
            self.length -= 1;
            &(*head.as_ptr()).elem
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.head.map(|head| unsafe {
            self.head = (*head.as_ptr()).next;
            self.length -= 1;
            &mut (*head.as_ptr()).elem
        })
    }
}

impl<T> Extend<T> for Queue<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.enqueue(item);
        }
    }
}

impl<T> FromIterator<T> for Queue<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut queue = Queue::new();
        queue.extend(iter);
        queue
    }
}

#[cfg(test)]
mod tests {
    use super::Queue;

    #[test]
    fn test_queue() {
        let mut queue = Queue::new();
        assert_eq!(queue.len(), 0);
        assert_eq!(queue.peek(), None);
        assert_eq!(queue.peek_mut(), None);
        assert_eq!(queue.deque(), None);

        queue.enqueue("Hello");
        queue.enqueue("World");

        assert_eq!(queue.len(), 2);
        assert_eq!(queue.peek(), Some(&"Hello"));
        assert_eq!(queue.peek_mut(), Some(&mut "Hello"));
        assert_eq!(queue.deque(), Some("Hello"));
        assert_eq!(queue.len(), 1);
        assert_eq!(queue.peek(), Some(&"World"));
        assert_eq!(queue.peek_mut(), Some(&mut "World"));
        queue.peek_mut().map(|x| *x = "World!");
        assert_eq!(queue.deque(), Some("World!"));
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn test_iter_queue() {
        let mut queue_a = Queue::from_iter(2..=10);
        assert_eq!(queue_a.iter().count(), 9);
        assert_eq!(queue_a.len(), 9);
        assert_eq!(queue_a.iter().find(|&&e| e == 5), Some(&5));
        assert_eq!(queue_a.iter().find(|&&e| e == 15), None);
        for e in queue_a.iter_mut() {
            *e *= 2
        }
        let mut queue_b = queue_a.iter().collect::<Queue<_>>();
        assert_eq!(queue_b.iter().count(), 9);
        assert_eq!(queue_b.deque(), Some(&4));
        assert_eq!(queue_b.deque(), Some(&6));
        assert_eq!(queue_b.deque(), Some(&8));
        assert_eq!(queue_b.deque(), Some(&10));
        assert_eq!(queue_b.deque(), Some(&12));
        assert_eq!(queue_b.deque(), Some(&14));
        assert_eq!(queue_b.deque(), Some(&16));
        assert_eq!(queue_b.deque(), Some(&18));
        assert_eq!(queue_b.deque(), Some(&20));
        for _e in &queue_b {}
        assert_eq!(queue_b.len(), 0);
        assert_eq!(queue_b.deque(), None);
    }
}
