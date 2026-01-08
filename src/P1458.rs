use std::cmp::{max, min, Ordering::Less};

const MAX_VAL: i32 = 1001;

// Assumes 1 <= len0 <= len1
pub fn max_dot_product(nums0: &[i32], nums1: &[i32]) -> i32 {
    // For convenience
    let len0 = nums0.len();
    let len1 = nums1.len();

    // Compute minimal ringbuffer cache size that's power of 2 and calloc it
    let cache_len = 1 << (4 * len1 - 1).ilog2();
    let cache_mask = cache_len - 1;
    let mut cache: Vec<i32> = Vec::new();
    cache.resize(cache_len, 0);

    let index = |i0, i1| (len1 * i0 + i1) & cache_mask;

    // i0 == 0
    {
        // i1 == 0
        cache[index(0, 0)] = max(0, nums0[0] * nums1[0]);

        // i1 != 0
        for i1 in 1..len1 {
            cache[index(0, i1)] = max(cache[index(0, i1 - 1)], nums0[0] * nums1[i1]);
        }
    }

    // i0 != 0
    for i0 in 1..len0 {
        // i1 == 0
        cache[index(i0, 0)] = max(cache[index(i0 - 1, 0)], nums0[i0] * nums1[0]);

        // i1 != 0
        for i1 in 1..len1 {
            let ind = index(i0, i1);
            cache[ind] = max(cache[index(i0 - 1, i1)], cache[index(i0, i1 - 1)]);
            cache[ind] = max(
                cache[ind],
                cache[index(i0 - 1, i1 - 1)] + nums0[i0] * nums1[i1],
            );
        }
    }

    // Check for and handle the bullshit case
    let val = cache[index(len0 - 1, len1 - 1)];
    match 0.cmp(&val) {
        Less => val,
        _ => handle_empty(nums0, nums1),
    }
}

// Subperformant but wgaf--this case is an affront to god
fn handle_empty(nums0: &[i32], nums1: &[i32]) -> i32 {
    let mut min0 = MAX_VAL;
    for &num in nums0 {
        min0 = min(min0, num.abs());
    }

    let mut min1 = MAX_VAL;
    for &num in nums1 {
        min1 = min(min1, num.abs());
    }

    -min0 * min1
}

#[cfg(test)]
mod tests {
    use super::max_dot_product;

    #[test]
    fn test1() {
        assert_eq!(
            max_dot_product(&vec![2, 1, -2, 5][..], &vec![3, 0, -6][..]),
            18
        )
    }

    #[test]
    fn test2() {
        assert_eq!(max_dot_product(&vec![3, -2][..], &vec![2, -6, 7][..]), 21)
    }

    #[test]
    fn test3() {
        assert_eq!(max_dot_product(&vec![-1, -1][..], &vec![1, 1][..]), -1)
    }
}
