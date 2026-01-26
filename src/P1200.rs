use std::cmp::Ordering::{Equal, Greater, Less};

const MAX_DIFF: i32 = 2_000_001;

pub fn minimum_abs_difference(mut nums: Vec<i32>) -> Vec<(i32, i32)> {
    nums.sort_unstable();

    let mut min_diff = MAX_DIFF;
    let mut pairs = Vec::new();
    for i in 0..(nums.len() - 1) {
        let num0 = nums[i];
        let num1 = nums[i + 1];
        let cur_diff = num1 - num0;
        match min_diff.cmp(&cur_diff) {
            Less => {}
            Equal => {
                pairs.push((num0, num1));
            }
            Greater => {
                min_diff = cur_diff;
                pairs.clear();
                pairs.push((num0, num1));
            }
        }
    }

    pairs
}

#[cfg(test)]
mod tests {
    use super::minimum_abs_difference;

    #[test]
    fn test1() {
        assert_eq!(
            minimum_abs_difference(vec![4, 2, 1, 3]),
            vec![(1, 2), (2, 3), (3, 4)]
        )
    }

    #[test]
    fn test2() {
        assert_eq!(minimum_abs_difference(vec![1, 3, 6, 10, 15]), vec![(1, 3)])
    }

    #[test]
    fn test3() {
        assert_eq!(
            minimum_abs_difference(vec![3, 8, -10, 23, 19, -4, -14, 27]),
            vec![(-14, -10), (19, 23), (23, 27)]
        )
    }
}
