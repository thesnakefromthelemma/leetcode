use std::cmp::{min, Ordering::Greater};

pub fn minimum_pair_removal(mut nums: Vec<i32>) -> i32 {
    let mut cnt_op = 0;

    while nums.len() >= 2 {
        let mut is_sorted = nums[0] <= nums[1];

        let mut min_lwr = 0;
        let mut min_sum = nums[min_lwr] + nums[min_lwr + 1];

        for i in 1..nums.len() - 1 {
            is_sorted &= nums[i] <= nums[i + 1];

            let cur_sum = nums[i] + nums[i + 1];
            // Optimizer should be able to eliminate branch
            min_lwr = match min_sum.cmp(&cur_sum) {
                Greater => i,
                _ => min_lwr,
            };
            min_sum = min(min_sum, cur_sum);
        }

        match is_sorted {
            true => break,
            // likely
            false => {
                cnt_op += 1;
                nums[min_lwr] = min_sum;
                let _ = nums.remove(min_lwr + 1);
            }
        }
    }

    cnt_op
}

#[cfg(test)]
mod tests {
    use super::minimum_pair_removal;

    #[test]
    fn test1() {
        assert_eq!(minimum_pair_removal(vec![5, 2, 3, 1]), 2)
    }

    #[test]
    fn test2() {
        assert_eq!(minimum_pair_removal(vec![1, 2, 2]), 0)
    }
}
