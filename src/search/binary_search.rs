/// This function take two arguments: an ordered slice and needle T,
/// returning true if the slice contains the needle.
///
/// # Examples
///
///```rust
/// # use algo_front_end_masters::search::binary_search;
/// let arr: [u32; 5] = [1, 2, 3, 4, 5];
/// let needle = 2;
/// assert!(binary_search(&arr, needle) == true);
/// ```
pub fn binary_search<T: PartialOrd>(haystack: &[T], needle: T) -> bool {
    // the lower bound of the binary search
    let mut low = 0;
    // The upper bound of the binary search
    let mut high = haystack.len();
    // slice mid index
    let mut mid: usize;
    // While the lower bound is smaller than the upper bound, this loop
    // runs. If low has the same value of high, it means that the slice
    // do not contain the needle and we return false.
    while low < high {
        // Calculate the mid index
        mid = low + (high - low) / 2;
        // Compare the element positioned in middle of the slice with the needle
        match haystack[mid].partial_cmp(&needle).unwrap() {
            // Return true if equal
            std::cmp::Ordering::Equal => return true,
            // If the element is smaller than the needle, we make the lower bound equal to
            // the mid index + 1, so we will discard everything to the left of the mid.
            std::cmp::Ordering::Less => low = mid + 1,
            // if the element is greater than the needle, we discard everything to the right of
            // the element by making the upper bound equal to the mid index.
            std::cmp::Ordering::Greater => high = mid,
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::binary_search;

    #[test]
    fn test_binary_search() {
        let arr: Vec<i32> = (0..=10).collect();
        assert_eq!(binary_search(&arr, 9), true);
    }
}
