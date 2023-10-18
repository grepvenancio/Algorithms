/// A stack is a LIFO data structure that you can push a item onto it
/// or pop it.
///
///  (D) -> (C) -> (B) -> (A)
///  head
pub struct Stack<T> {
    head: Link<T>,
    length: usize,
}

type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    elem: T,
    next: Link<T>
}

impl<T> Stack<T> {
    /// Create an empty Stack
    ///
    /// Example
    ///```rust
    /// let stack = Stack::new();
    /// assert_eq!(stack.len(), 0);
    ///```
    pub fn new() -> Self {
        Stack { head: None, length: 0 }
    }

    // Push an element onto the stack
    //
    // Example
    ///```rust
    /// let stack = Stack::new();
    /// stack.push("World");
    ///```
    pub fn push(&mut self, elem: T) {
        // Create or new head and set the next value as the current head.
        let new_head = Box::new(Node {elem, next: self.head.take()});
        // Set the new head
        self.head = Some(new_head); 
        // Increase the length
        self.length += 1;
    }

    /// Pop an element from the stack
    ///
    // Example
    ///```rust
    /// let stack = Stack::new();
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
    ///Example
    ///```rust
    /// let stack = Stack::new();
    /// stack.push("Hello World!");
    /// assert_eq!(stack.peek(), Some(&"Hello World!"));
    ///```
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|head| &head.elem)
    }

    /// Returns a mut reference to the top of the stack.
    ///
    ///Example
    ///```rust
    /// let stack = Stack::new();
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

}

#[cfg(test)]
mod tests {
    use super::Stack;

    #[test]
    fn test_stack() {
        let mut stack = Stack::new();

        assert_eq!(stack.len(), 0);
        assert_eq!(stack.pop(), None);
        assert_eq!(stack.peek(), None);
        assert_eq!(stack.peek_mut(), None);

        stack.push("Hello");
        stack.push("World");

        assert_eq!(stack.len(), 2);
        assert_eq!(stack.peek(), Some(&"World"));
        stack.peek_mut().map(|value| *value = "World!");
        assert_eq!(stack.pop(), Some("World!"));
        assert_eq!(stack.peek_mut(), Some(&mut "Hello"));
        assert_eq!(stack.pop(), Some("Hello"));
        assert_eq!(stack.len(), 0);
        assert_eq!(stack.pop(), None);
        assert_eq!(stack.peek(), None);
        assert_eq!(stack.peek_mut(), None);
    }
}

