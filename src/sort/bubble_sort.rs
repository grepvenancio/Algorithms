/// This function takes a mut reference to a slice and reorder it with
/// bubble sort.
///
/// # Examples
///
///```rust
/// # use algo_front_end_masters::sort::bubble_sort;
/// let mut arr: [i32; 5] = [2, 3, 5, 4, 1];
/// bubble_sort(&mut arr);
/// assert_eq!(arr, [1, 2, 3, 4, 5]);
/// ```
pub fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len(); // Get the length of the input array 'arr'.
                         // The outer loop iterates from 0 to 'len - 1'.
    for i in 0..len {
        // 'i' represents the number of elements that are already in their final sorted position.
        // The inner loop iterates from 0 to '(len - 1 - i)'.
        // It compares adjacent elements and swaps them if they are out of order.
        for j in 0..(len - 1 - i) {
            // Compare the adjacent elements using 'partial_cmp' and handle the result.
            match arr[j].partial_cmp(&arr[j + 1]).unwrap() {
                // If the current element is greater than the next element, swap them.
                std::cmp::Ordering::Greater => arr.swap(j, j + 1),
                // If the current element is equal to or less than the next element, continue to the next pair.
                std::cmp::Ordering::Equal | std::cmp::Ordering::Less => continue,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::bubble_sort;

    #[test]
    fn test_bubble_sort() {
        let mut arr: [i32; 5] = [2, 3, 5, 4, 1];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }
}
