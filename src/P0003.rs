use std::cmp::max;

pub fn length_of_longest_substring(s: String) -> i32 {
    // Assuming the argument consists entirely of ASCII characters
    // (That way we don't have to dick around with a HashWhatever
    // and fuck up cache locality by allocating all over the heap)
    let bytes = s.as_bytes();
    let mut has = [false; 128];

    // Two pointers...
    let mut lower = 0;
    let mut upper = 0;
    let mut max_len = 0;

    while upper < bytes.len() {
        if has[bytes[upper] as usize] {
            while bytes[lower] != bytes[upper] {
                has[bytes[lower] as usize] = false;
                lower += 1;
            }
            lower += 1;
            upper += 1;
        } else {
            has[bytes[upper] as usize] = true;
            upper += 1;
            max_len = max(max_len, upper - lower);
        }
    }

    max_len as i32
}

#[cfg(test)]
mod tests {
    use super::length_of_longest_substring;

    #[test]
    fn test1() {
        assert_eq!(length_of_longest_substring(String::from("abcabcbb")), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(length_of_longest_substring(String::from("bbbbb")), 1);
    }

    #[test]
    fn test3() {
        assert_eq!(length_of_longest_substring(String::from("pwwkew")), 3);
    }
}
