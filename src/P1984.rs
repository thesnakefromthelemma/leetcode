use std::cmp::min;

// Based on constraints
const MAX_DIFF: i32 = 100_001;

pub fn minimum_difference(mut nums: Vec<i32>, k: usize) -> i32 {
    nums.sort();

    let mut min_diff = MAX_DIFF;
    for i in 0..(nums.len() - k + 1) {
        min_diff = min(min_diff, nums[i + k - 1] - nums[i]);
    }

    min_diff
}

#[cfg(test)]
mod tests {
    use super::minimum_difference;

    /*
    Moronic test case
    #[test]
    fn test1() {
        assert_eq!(minimum_difference(vec![90], 1), 0)
    }
    */

    #[test]
    fn test2() {
        assert_eq!(minimum_difference(vec![9, 4, 1, 7], 2), 2)
    }
}
