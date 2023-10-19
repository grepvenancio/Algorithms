/// This function will linearly search a slice for an ocurrence of a needle T,
/// and it will return true if find it, and false if not.
///
/// # Examples
///
///```rust
/// # use algo_front_end_masters::search::linear_search;
/// let arr: [u32; 5] = [2, 4, 5, 1, 3];
/// let needle = 1;
/// assert!(linear_search(&arr, needle) == true);
/// ```
pub fn linear_search<T: PartialEq>(haystack: &[T], needle: T) -> bool {
    haystack.iter().any(|e| *e == needle)
}

#[cfg(test)]
mod tests {
    use super::linear_search;

    #[test]
    fn test_linear_search() {
        let arr: [i32; 3] = [10, 20, 30];
        assert_eq!(linear_search(&arr, 10), true);
        assert_eq!(linear_search(&arr, 40), false);
    }
}
