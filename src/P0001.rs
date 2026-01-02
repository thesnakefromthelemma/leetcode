use std::cmp::Ordering::{Equal, Greater, Less};

// Why not return a pair of usizes?
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // Allocate new indexed array\;
    // it's not clear that packing it tighter
    // (i.e., using i32 instead of usize) is a real win
    let mut inds: Vec<i32> = (0..(nums.len() as i32)).collect();

    // Sort indexed array on values
    inds.sort_unstable_by(|i0, i1| nums[*i0 as usize].cmp(&nums[*i1 as usize]));

    // Inward two-pointer search for values suumming to target
    let mut lower: usize = 0;
    let mut upper: usize = nums.len() - 1;
    while lower < upper {
        match (nums[inds[lower] as usize] + nums[inds[upper] as usize]).cmp(&target) {
            Equal => break,
            Less => lower += 1,
            Greater => upper -= 1,
        };
    }

    // Return indices of satisfactory values
    vec![inds[lower], inds[upper]]
}

#[cfg(test)]
mod tests {
    use super::two_sum;

    #[test]
    fn test1() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn test2() {
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    fn test3() {
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}