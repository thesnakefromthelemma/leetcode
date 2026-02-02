use std::cmp::{max, min};

// O(n) time (single pass), O(1) space
pub fn minimum_cost(nums: &[i32]) -> i32 {
    let head = nums[0];
    let mut tail0 = min(nums[1], nums[2]);
    let mut tail1 = max(nums[1], nums[2]);
    for i in 3..nums.len() {
        let mut tmp = tail0;
        tail0 = min(tmp, nums[i]);
        tmp = max(tmp, nums[i]);
        tail1 = min(tail1, tmp);
    }
    head + tail0 + tail1
}

#[cfg(test)]
mod tests {
    use super::minimum_cost;

    #[test]
    fn test1() {
        assert_eq!(minimum_cost(&[1, 2, 3, 12]), 6)
    }

    #[test]
    fn test2() {
        assert_eq!(minimum_cost(&[5, 4, 3]), 12)
    }

    #[test]
    fn test3() {
        assert_eq!(minimum_cost(&[10, 3, 1, 1]), 12)
    }
}
