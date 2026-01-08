use std::cmp::{min, max, Ordering::{Less, Equal}};

const MAX_VAL: i32 = 1001;

fn index_dp(len0: usize, len1: usize, mask: usize, ind0: usize, ind1: usize) -> usize {
    let sum_of = |n| n * (n + 1) / 2;

    let rank = ind0 + ind1;

    let spill0 = rank - min(rank, len0);
    let spill1 = rank - min(rank, len1 - 1);

    (sum_of(ind0 + ind1) + ind0 - sum_of(spill0) - sum_of(spill1)) & mask
}

pub fn max_dot_product(nums0: &Vec<i32>, nums1: &Vec<i32>) -> i32 {
    let len0 = nums0.len();
    let len1 = nums1.len();
    // So we can assume len0 <= len1
    if len0 > len1 {
        return max_dot_product(nums1, nums0);
    }
    // Annoying case, evict it so that we can assume len0, len1 >= 2
    if len0 < 2 || len1 < 2 {
        return handle_empty(nums0, nums1);
    }

    // Compute minimal cache size that's power of 2 and calloc it
    let cache_len = 1 << (2 * match len0.cmp(&len1) {
        Equal => 2 * min(len0, len1) - 1,
        _ => 2 * min(len0, len1),
    } - 1).ilog2();
    let cache_mask = cache_len - 1;
    let mut cache: Vec<i32> = Vec::new();
    cache.resize(cache_len, 0);

    // For convenience
    let index = |ind0, ind1| index_dp(len0, len1, cache_mask, ind0, ind1);

    // i0 == 0, i1 == 0
    cache[index(0, 0)] = max(0, nums0[0] * nums1[0]);

    for rank in 1..len0 {
        cache[index(0, rank)] = max(cache[index(0, rank - 1)], nums0[0] * nums1[rank]);

        for i0 in 1..rank {
            let i1 = rank - i0;
            let ind = index(i0, i1);
            cache[ind] = max(cache[index(i0 - 1, i1)], cache[index(i0, i1 - 1)]);
            cache[ind] = max(cache[ind], cache[index(i0 - 1, i1 - 1)] + nums0[i0] * nums1[i1]);
        }

        cache[index(rank, 0)] = max(cache[index(rank - 1, 0)], nums0[rank] * nums1[0]);
    }

    for rank in len0..len1 {
        cache[index(0, rank)] = max(cache[index(0, rank - 1)], nums0[0] * nums1[rank]);

        for i0 in 1..len0 {
            let i1 = rank - i0;
            let ind = index(i0, i1);
            cache[ind] = max(cache[index(i0 - 1, i1)], cache[index(i0, i1 - 1)]);
            cache[ind] = max(cache[ind], cache[index(i0 - 1, i1 - 1)] + nums0[i0] * nums1[i1]);
        }
    }

    for rank in len1..(len0 + len1 - 2) {
        for i0 in (rank - len1 + 1)..len0 {
            let i1 = rank - i0;
            let ind = index(i0, i1);
            cache[ind] = max(cache[index(i0 - 1, i1)], cache[index(i0, i1 - 1)]);
            cache[ind] = max(cache[ind], cache[index(i0 - 1, i1 - 1)] + nums0[i0] * nums1[i1]);
        }
    }

    // We don't actually write cache[index(len0 - 0, len1 - 1)] to the cache,
    // since we immediately access and consume it; instead we write to val
    let mut val = max(cache[index(len0 - 2, len1 - 1)], cache[index(len0 - 1, len1 - 2)]);
    val = max(val, cache[index(len0 - 2, len1 - 2)] + nums0[len0 - 1] * nums1[len1 - 1]);

    // Check and handle bullshit case
    match 0.cmp(&val) {
        Less => val,
        _ => handle_empty(nums0, nums1),
    }
}

// Subperformant but wgaf--this case is an affront to god
fn handle_empty(nums0: &Vec<i32>, nums1: &Vec<i32>) -> i32 {
    let mut min0 = MAX_VAL;
    for num in nums0 {
        min0 = min(min0, num.abs());
    }

    let mut min1 = MAX_VAL;
    for num in nums1 {
        min1 = min(min1, num.abs());
    }

    - min0 * min1
}

#[cfg(test)]
mod tests {
    use super::max_dot_product;

    #[test]
    fn test1() {
        assert_eq!(max_dot_product(&vec![2,1,-2,5], &vec![3,0,-6]), 18)
    }

    #[test]
    fn test2() {
        assert_eq!(max_dot_product(&vec![3,-2], &vec![2,-6,7]), 21)
    }

    #[test]
    fn test3() {
        assert_eq!(max_dot_product(&vec![-1,-1], &vec![1,1]), -1)
    }
}