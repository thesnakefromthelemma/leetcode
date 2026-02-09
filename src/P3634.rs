use std::cmp::{max, Ordering::Greater};

pub fn min_removal(mut nums: Vec<i32>, k: i32) -> i32 {
    let len = nums.len();
    let k_fr = k as i64;

    // Ugh
    nums.sort_unstable();

    // Sliding window
    let mut len_max = 0;
    let mut lwr = 0;
    let mut upr = 0;
    loop {
        while len > upr && k_fr * (nums[lwr] as i64) >= nums[upr] as i64 { upr += 1; };
        len_max = max(len_max, upr - lwr);
        match len.cmp(&upr) {
            Greater => while k_fr * (nums[lwr] as i64) < nums[upr] as i64 { lwr += 1; },
            _ => break,
        };
    };

    (len - len_max) as i32
}

#[cfg(test)]
mod tests {
    use super::min_removal;

    #[test]
    fn test1() {
        assert_eq!(min_removal(vec![2, 1, 5], 2), 1)
    }

    #[test]
    fn test2() {
        assert_eq!(min_removal(vec![1, 6, 2, 9], 3), 2)
    }

    #[test]
    fn test3() {
        assert_eq!(min_removal(vec![4, 6], 2), 0)
    }
}