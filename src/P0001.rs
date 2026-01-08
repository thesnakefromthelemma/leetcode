use std::cmp::Ordering::{Equal, Greater, Less};

// Return indices as (u16, u16) as nums.len() <= 10_000
pub fn two_sum(nums: &[i32], target: i32) -> (u16, u16) {
    // Allocate new indexed array\;(
    let mut inds: Vec<u16> = (0..(nums.len() as u16)).collect();

    // Sort indexed array on values
    inds.sort_unstable_by(|&i0, &i1| nums[i0 as usize].cmp(&nums[i1 as usize]));

    // Inward two-pointer search for values summing to target
    let mut lower = 0;
    let mut upper = nums.len() - 1;
    while lower < upper {
        match (nums[inds[lower] as usize] + nums[inds[upper] as usize]).cmp(&target) {
            Equal => break,
            Less => lower += 1,
            Greater => upper -= 1,
        };
    }

    // Return indices of satisfactory values
    (inds[lower], inds[upper])
}

#[cfg(test)]
mod tests {
    use super::two_sum;

    #[test]
    fn test1() {
        assert_eq!(two_sum(&vec![2, 7, 11, 15][..], 9), (0, 1))
    }

    #[test]
    fn test2() {
        assert_eq!(two_sum(&vec![3, 2, 4][..], 6), (1, 2))
    }

    #[test]
    fn test3() {
        assert_eq!(two_sum(&vec![3, 3][..], 6), (0, 1))
    }
}
