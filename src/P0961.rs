pub fn repeated_n_times(nums: &Vec<i32>) -> i32 {
    // Check if nums[1] and nums[3] are equal
    if nums[1] == nums[3] {
        nums[1]
    } else {
        // Otherwise check if there are two consecutive equal values
        for i in 0..(nums.len() - 1) {
            if nums[i] == nums[i + 1] {
                return nums[i];
            }
        }
        // Otherwise nums[-1] and nums[0] are equal
        // or the equal values populate the even indices
        nums[0]
    }
}

#[cfg(test)]
mod tests {
    use super::repeated_n_times;

    #[test]
    fn test1() {
        assert_eq!(repeated_n_times(&vec![1,2,3,3]), 3)
    }

    #[test]
    fn test2() {
        assert_eq!(repeated_n_times(&vec![2,1,2,5,3,2]), 2)
    }

    #[test]
    fn test3() {
        assert_eq!(repeated_n_times(&vec![5,1,5,2,5,3,5,4]), 5)
    }
}