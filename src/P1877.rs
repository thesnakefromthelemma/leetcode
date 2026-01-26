use std::cmp::max;

pub fn min_pair_sum(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let mut max_sum = 2;
    for i in 0..(nums.len() / 2) {
        max_sum = max(max_sum, nums[i] + nums[nums.len() - 1 - i]);
    }
    max_sum
}

#[cfg(test)]
mod tests {
    use super::min_pair_sum;

    #[test]
    fn test1() {
        assert_eq!(min_pair_sum(vec![3,5,2,3]), 7)
    }

    #[test]
    fn test2() {
        assert_eq!(min_pair_sum(vec![3,5,4,2,4,6]), 8)
    }
}