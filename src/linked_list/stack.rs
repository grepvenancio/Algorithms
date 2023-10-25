use std::fmt::Debug;

/// A stack is a LIFO data structure that you can push a item onto it
/// or pop it.
///
///  (D) -> (C) -> (B) -> (A) 
///  head
pub struct Stack<T> {
    head: Link<T>,
    length: usize,
}

pub struct IntoIter<T> {
    stack: Stack<T>,
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
    length: usize,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
    length: usize,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> Stack<T> {
    /// Create an empty Stack
    ///
    /// # Examples
    ///
    ///```rust
    /// # use algo_front_end_masters::linked_list::Stack;
    /// let stack: Stack<String> = Stack::new();
    /// assert_eq!(stack.len(), 0);
    ///```
    pub fn new() -> Self {
        Stack {
            head: None,
            length: 0,
        }
    }

    // Push an element onto the stack
    //
    /// # Examples
    ///
    ///```rust
    /// # use algo_front_end_masters::linked_list::Stack;
    /// let mut stack = Stack::new();
    /// stack.push("World");
    ///```
    pub fn push(&mut self, elem: T) {
        // Create or new head and set the next value as the current head.
        let new_head = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        // Set the new head
        self.head = Some(new_head);
        // Increase the length
        self.length += 1;
    }

    /// Pop an element from the stack
    ///
    /// # Examples
    ///
    ///```rust
    /// # use algo_front_end_masters::linked_list::Stack;
    /// let mut stack = Stack::new();
    /// stack.push("Hello");
    /// stack.push("World!");
    /// assert_eq!(stack.pop(), Some("World!"));
    ///```
    pub fn pop(&mut self) -> Option<T> {
        // Take the head as value and put a None in place of by using the take() method
        self.head.take().map(|head| {
            // Set the head (None) to value next of the head we popped
            self.head = head.next;
            // Decrease the lentgh
            self.length -= 1;
            // Return the elem contained in the Node
            head.elem
        })
    }

    /// Returns a shared reference to the top of the stack.
    ///
    /// # Examples
    ///
    ///```rust
    /// # use algo_front_end_masters::linked_list::Stack;
    /// let mut stack = Stack::new();
    /// stack.push("Hello World!");
    /// assert_eq!(stack.peek(), Some(&"Hello World!"));
    ///```
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|head| &head.elem)
    }

    /// Returns a mut reference to the top of the stack.
    ///
    /// # Examples
    ///
    ///```rust
    /// # use algo_front_end_masters::linked_list::Stack;
    /// let mut stack = Stack::new();
    /// stack.push("Hello World!");
    /// stack.peek_mut().map(|e| *e = "Hello");
    /// assert_eq!(stack.peek_mut(), Some(&mut "Hello"));
    ///```
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|head| &mut head.elem)
    }

    /// Returns the length of the stack.
    pub fn len(&self) -> usize {
        self.length
    }

    /// Returns an Iterator over the stack. 
    ///
    /// The iterator yields all items from start to end.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use algo_front_end_masters::linked_list::Stack;
    /// let mut stack = Stack::from_iter(1..=3);
    /// let mut iterator = stack.iter();
    ///
    /// assert_eq!(iterator.next(), Some(&3));
    /// assert_eq!(iterator.next(), Some(&2));
    /// assert_eq!(iterator.next(), Some(&1));
    /// assert_eq!(iterator.next(), None);
    /// ```
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
            length: self.length,
        }
    }

    /// Returns an Iterator that allows modifyng each value. 
    ///
    /// The iterator yields all items from start to end.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use algo_front_end_masters::linked_list::Stack;
    /// let mut stack = Stack::from_iter(1..=3);
    ///
    /// for elem in stack.iter_mut() {
    ///     *elem *= 2;
    /// }
    ///
    /// assert_eq!(stack.pop(), Some(6));
    /// assert_eq!(stack.pop(), Some(4));
    /// assert_eq!(stack.pop(), Some(2));
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_deref_mut(),
            length: self.length,
        }
    }

    /// Returns the true if the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.length == 0 
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> IntoIterator for Stack<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { stack: self }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

impl<'a, T> IntoIterator for &'a Stack<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Stack<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|head| {
            self.next = head.next.as_deref();
            self.length -= 1;
            &head.elem
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|head| {
            self.next = head.next.as_deref_mut();
            self.length -= 1;
            &mut head.elem
        })
    }
}

impl<T> Extend<T> for Stack<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push(item);
        }
    }
}

impl<T> FromIterator<T> for Stack<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut stack = Self::new();
        stack.extend(iter);
        stack
    }
}

impl<T: Clone> Clone for Stack<T> {
    fn clone(&self) -> Self {
        let mut clone_stack = Self::new();
        let tmp = self.iter().collect::<Stack<_>>();
        clone_stack.extend(tmp.into_iter().cloned());
        clone_stack
    }
}

impl<T: Debug> Debug for Stack<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

impl<T: Eq> Eq for Stack<T> {}

impl<T: PartialOrd> PartialOrd for Stack<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.iter().partial_cmp(other)
    }
}

impl<T: PartialEq> PartialEq for Stack<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().eq(other)
    }
}

impl<T: Ord> Ord for Stack<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.iter().cmp(other)
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}


#[cfg(test)]
mod tests {
    use super::Stack;

    #[test]
    fn test_stack() {
        let mut stack = Stack::new();
        assert!(stack.is_empty());
        assert_eq!(stack.pop(), None);
        assert_eq!(stack.peek(), None);
        assert_eq!(stack.peek_mut(), None);

        stack.push("Hello");
        stack.push("World");

        let stack_clone = stack.clone();
        assert_eq!(stack, stack_clone);

        assert_eq!(stack.len(), 2);
        assert_eq!(stack.peek(), Some(&"World"));

        if let Some(head) = stack.peek_mut() { *head = "World!" }

        assert_eq!(stack.pop(), Some("World!"));
        assert_eq!(stack.peek_mut(), Some(&mut "Hello"));
        assert_eq!(stack.pop(), Some("Hello"));
        assert_eq!(stack.len(), 0);
        assert_eq!(stack.pop(), None);
        assert_eq!(stack.peek(), None);
        assert_eq!(stack.peek_mut(), None);
    }

    #[test]
    fn test_stack_iter() {
        let mut stack_a = Stack::from_iter(2..=10);
        assert_eq!(stack_a.len(), 9);
        assert_eq!(stack_a.pop(), Some(10));
        for e in stack_a.iter_mut() {
            *e *= 2;
        }
        assert_eq!(stack_a.pop(), Some(18));
        let mut stack_b = stack_a.iter().collect::<Stack<_>>();
        assert_eq!(stack_b.pop(), Some(&4));
        assert_eq!(stack_b.len(), 6);
    }

    #[test]
    fn test_eq_ord_stack() {
        let mut stack_a = Stack::<i32>::new();
        let mut stack_b = Stack::<i32>::new();
        assert!(stack_a == stack_b);
        stack_a.push(1);
        stack_a.push(2);
        assert!(stack_a > stack_b);
        assert!(stack_a != stack_b);
        stack_b.push(1);
        stack_b.push(2);
        assert!(stack_a == stack_b);
        let _ = stack_b.pop();
        stack_b.push(4);
        assert!(stack_b > stack_a);
    }
}
