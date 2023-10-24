/// This function takes a slice &[bool] and returns the index of the first occurrence of true.
///
/// # Examples
///
///```rust
/// # use algo_front_end_masters::search::two_crystal_balls;
/// let arr_bool: [bool; 5] = [false, false, true, true, true];
/// assert_eq!(two_crystal_balls(&arr_bool), Some(2));
///```
pub fn two_crystal_balls(breaks: &[bool]) -> Option<usize> {
    // Calculate the jump distance based on the square root of the number of elements.
    let jmp = f32::sqrt(breaks.len() as f32) as usize;
    // Initialize the address (index) to 0.
    let mut addr: usize = 0;
    // Perform the first part of the search using a while loop.
    while addr < breaks.len() {
        // Check if the ball breaks at the current index.
        if breaks[addr] {
            // If it breaks, move back by the jump distance and exit the loop.
            addr -= jmp;
            break;
        }
        // If it doesn't break, move forward by the jump distance.
        addr += jmp;
    }
    // Perform the second part of the search using a for loop
    for e in breaks.iter().skip(addr) {
        // Check if the ball breaks at the current index.
        if *e {
            // If it breaks, return the current index as Some.
            return Some(addr);
        }
        addr += 1;
    }
    // If no breaking point is found, return None.
    None
}

#[cfg(test)]
mod tests {
    use super::two_crystal_balls;

    #[test]
    fn test_binary_search() {
        let arr: [bool; 10] = [
            false, false, false, false, true, true, true, true, true, true,
        ];
        let index = two_crystal_balls(&arr);
        assert_eq!(index, Some(4));
        let arr: [bool; 10] = [
            false, false, false, false, false, false, false, false, false, false,
        ];
        let index = two_crystal_balls(&arr);
        assert_eq!(index, None);
    }
}
