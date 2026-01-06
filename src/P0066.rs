// Dumbass choice of ordering of digits tbh
pub fn plus_one(digits: &mut Vec<i32>) {
    let mut i = digits.len();
    loop {
        match i {
            0 => {
                // We dont worry about the case of vec![] due to the spec
                digits[0] = 1;
                digits.push(0);
                break;
            }
            _ => match digits[i - 1] {
                9 => {
                    digits[i - 1] = 0;
                    i -= 1;
                }
                _ => {
                    digits[i - 1] += 1;
                    break;
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::plus_one;

    #[test]
    fn test1() {
        let mut digits = vec![1, 2, 3];
        plus_one(&mut digits);
        assert_eq!(digits, vec![1, 2, 4])
    }

    #[test]
    fn test2() {
        let mut digits = vec![4, 3, 2, 1];
        plus_one(&mut digits);
        assert_eq!(digits, vec![4, 3, 2, 2])
    }

    #[test]
    fn test3() {
        let mut digits = vec![9];
        plus_one(&mut digits);
        assert_eq!(digits, vec![1, 0])
    }
}
