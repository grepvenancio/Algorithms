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

# Array definition

An array is a contiguous space of memory that contains N numbers of elements of the same size, 
so you can acess a specific element in the array by just providing an offset and multiplying by the data 
size contained in the array.

# Search

Search algorithms can be sumarized as a series of instructions with the purpose of finding a specific value in
a sequence of data.

## Linear Search

A linear search will take two arguments: A sequence of data, a needle that will be value that we want to search in 
the sequence. The ruining time of a linear search is **O (N)**

Click here[]() to see a linear search implementation.

## Binary Search

Binary search is a search algorithm that, instead of scanning each element in the sequence, finds a specific
ocurrence, it will take as an argument an ordered sequence and will jump to half of the sequence and verify if the value
to be found is bigger or smaller than the element that is positioned in the other half of the sequence, 
If it's smaller, the *algo* will half the left side of the sequence; if it's bigger, it will half 
the right side, and it will continue to do it until it finds the value. The running time of a 
binary search is **O (log n)** because of the number of steps necessary to search for a value in the sequence.
will be equal to *log<sub>2</sub>N*, so if we have a sequence 256 items long, we will need
*log<sub>2</sub>256 = 8* steps to find a value in it.

Click here[]() to see a binary search implementation.

# Other 
see: [Two crystal balls problem]()
