// This linear search function will take a sequence of T and a needle T, and it will
// search in the sequence for an occurrence that is equal to the needle, returning true
// if find it, and false if not.
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
