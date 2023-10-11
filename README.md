# Intro

This repo contains a series of algorithms and data structures that are shown in the Front End Master's 
Free Algorithm course but implemented in Rust.

## Big O

Big O is a way to gategorize the growth of an algorithm's complexity (time or space complexity)
in relation to its inputs.

A **O (N)** algorithm means that the complexity of the algorithm will grow linearly based on input.

↓---- Complexity order
O (N)
   ^--------- Input

The runtime of the following code is *linear* (O (N)). Because more elements are added to the input, 
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

One could say that this algo has O (2N) complexity, but actually it is O (N). 
Because we will always drop constants. O (~~2~~N) -> O (N).

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

TODO: Binary search tree.

![Big-O Complexity.](https://en.wikipedia.org/wiki/Big_O_notation#/media/File:Comparison_computational_complexity.svg)
