// What the fuck is the difference between this and P3314???
pub fn min_bitwise_array(nums: &mut Vec<i32>) {
    for i in 0..nums.len() {
        let num = nums[i];
        let tocnt = num.trailing_ones();
        nums[i] = match (tocnt as i32).is_positive() {
            true => num - (1 << (tocnt - 1)),
            false => -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::min_bitwise_array;

    #[test]
    fn test1() {
        let mut vec = vec![2, 3, 5, 7];
        min_bitwise_array(&mut vec);
        assert_eq!(vec, vec![-1, 1, 4, 3])
    }

    #[test]
    fn test2() {
        let mut vec = vec![11, 13, 31];
        min_bitwise_array(&mut vec);
        assert_eq!(vec, vec![9, 12, 15])
    }
}