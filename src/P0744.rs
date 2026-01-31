use std::cmp::Ordering::Less;

// Binary search...duh
pub fn next_greatest_letter(letters: &[char], target: char) -> char {
    let len = letters.len();
    let def = letters[0];
    if target < letters[0] || target >= letters[len - 1] {
        return def;
    } else {
        let mut lwr = 0;
        let mut upr = len - 1;
        while upr - lwr >= 2 {
            let mid = (lwr + upr) / 2;
            match target.cmp(&letters[mid]) {
                Less => upr = mid,
                _ => lwr = mid,
            }
        }
        letters[upr]
    }
}

#[cfg(test)]
mod tests {
    use super::next_greatest_letter;

    #[test]
    fn test1() {
        assert_eq!(next_greatest_letter(&['c', 'f', 'j'], 'a'), 'c')
    }

    #[test]
    fn test2() {
        assert_eq!(next_greatest_letter(&['c', 'f', 'j'], 'c'), 'f')
    }

    #[test]
    fn test3() {
        assert_eq!(next_greatest_letter(&['x', 'x', 'y', 'y'], 'z'), 'x')
    }
}
