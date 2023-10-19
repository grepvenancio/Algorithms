# Intro

This repo contains a series of algorithms and data structures that are shown in the Front End Master's 
Free Algorithm course but implemented in Rust.

## Big O

Big O is a way to gategorize the growth of an algorithm's complexity (time or space complexity)
in relation to its inputs.

A **O (N)** algorithm means that the complexity of the algorithm will grow linearly based on input.

The runtime of the following code is *linear* (O (N)). Because the more elements are added to the input, 
an additional operation for each element will be needed to calculate the sum.


```rust
pub fn sum_char_code_on(n: &str) -> usize {
    let mut sum = 0;
    for char in n.chars() {
        sum += char as usize;
    }
    return sum;
}
```

### Constants

One could say that the *algo* below has **O (2N)** complexity, but actually it is **O (N)**. 
We will always drop constants in the Big O notation. So O **(~~2~~N)** turns to **O (N)**, 
as eventually the contants become irrelevant as the input grow. 
Take, for example: Let's say that we have an input quantity of 100, so -> N = 100, 
and we have two *algo*, one is: **O (10N)** and the other is: **O (N²)**, in the first case we would 
need a total of 10x100 = 1 000 "operations" to compute the input, in the latter case we would 
need 100² = 10 000 "operations" to compute all of the input, a difference of 10x.

```rust
pub fn sum_char_code_o2n(n: &str) -> usize {
    let mut sum = 0;
    for (index, _char) in n.chars().enumerate() {
        sum += char as usize;
    }

    for (index, _char) in n.chars().enumerate() {
        sum += char as usize;
    }
    return sum;
}
```

## O (N²)

The time complexity of the following code will grow O(N²) time complexity, as it needs
to do a "2D" loop over the input.

```rust
fn sum_char_code_square(n: &str) -> usize {
    let mut sum = 0;
    for char in n.chars() {
        for char in n.chars() {
            sum += char as usize
        }
    }
    return sum;
}
```

## O (n log n)

TODO: QuickSort.

## O (log n)

see: [Binary Search](#binary-search)

TODO: Binary search tree.

![Big-O Complexity.](/images/Comparison_computational_complexity.svg)

# Search

Search algorithms can be sumarized as a series of instructions with the purpose of finding a specific value in
a sequence of data.

## Linear Search

A linear search will take two arguments: A sequence of data and a needle that will be the value that we want to find in 
the sequence. The ruining time of a linear search is **O (N)**

Click here[]() to see a linear search implementation.

## Binary Search

Binary search is a search algorithm that, instead of scanning each element in the sequence to find a specific
ocurrence, it will take as an argument an ordered sequence and will jump to half of the sequence and verify if the value
to be found is bigger or smaller than the element that is positioned in the half of the sequence, 
If it's smaller, the *algo* will half the left side of the sequence; if it's bigger, it will half 
the right side, and it will continue to do it until it finds the value. The running time of a 
binary search is **O (log n)** because the number of steps necessary to search for a value in the sequence
will be equal to *log<sub>2</sub>N*, so if we have a sequence 256 items long, we will need
*log<sub>2</sub>256 = 8* steps to find a value in it.

Click here[]() to see a binary search implementation.

# Other 
see: [Two crystal balls problem]()

# Sort

Sorting algorithms are a series of algorithms specialised in taking an unsorted sequence of data as input, 
outputting a sorted sequence of data.

## Bubble Sort

Bubble sort is a sorting algorithm with **O (N²)** time complexity. It is a relatively simple algorithm that 
will take a subsequent pair of data. Let's say X and Y, if X is greater than Y then they swap their positions,
If not, the *algo* will take another pair now, Y and Z and make the same comparison. If X was greater than Y 
the pair in this scenario would be X and Z. The *algo* will continue in this loop of instructions 
until it sorts the entire sequence.

Click here[]() to see a Bubble Sort search implementation.

# Linked List

A Linked List is a primordial data structure that can be summarized as a Node oriented data structure, 
which means that each value pushed onto a linked list is encapsulated in a Node that has pointers[¹] 
that connect one Node to other Nodes, forming a linked list.

[¹]: Technically, it is not true for stack allocated linked list.

## Queue

A Queue is a FIFO (First In First Out) data structure that stores a head and a tail pointer, 
which you can append a node to it's tail and take it from it's head. Normally we call these two operations. 
as: unqueue and deque. Each node in a queue has a inner value and a pointer to the next node, 
when you unqueue a empty queue you set the tail and the head to point to this new allocated node, the sequential
unqueue operation will make the next pointer of the tail Node point to a new Node and make the tail to point
to this new Node. When you deque you take the Node that is being pointed from head, make the head points
to the next Node and returns the contained value. In a queue, you normally can deque and unqueue elements with
a **O (1)** time complexity, but because it's not a contiguous space of memory and you do not store a pointer
for each element in the queue, if you want to access any element that is not the head or tail, you will need to traverse the queue.

Click here[]() to see a Queue implementation.

## Stack

A Stack is a LIFO (Last In First Out) data structure, where you can push and element to the top of the 
stack and pop, wich will remove the top item from the stack.

Click here[]() to see a Stack implementation.

# Array

An array is a contiguous space of memory that contains N numbers of elements of the same size, 
so you can acess a specific element in the array by just providing an offset and multiplying by the data 
size contained in the array.
