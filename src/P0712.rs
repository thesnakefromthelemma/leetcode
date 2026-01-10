use std::cmp::min;

// O(bytes0.len() * bytes1.len()) time,
// O(bytes1.len()) space allocated upfront
// (Could be a lot more polymorphic had I just the courage...)
pub fn minimum_delete_sum(bytes0: &[u8], bytes1: &[u8]) -> i32 {
    // For convenience
    let len0 = bytes0.len() + 1;
    let len1 = bytes1.len() + 1;

    // Calloc ringbuffer cache of length
    // the smallest power 2 at least len1 + 1
    let cache_len = 1 << (2 * len1 + 1).ilog2();
    let mut cache = Vec::new();
    cache.resize(cache_len, 0);

    // Indexing into cache with bitmask
    let cache_mask = cache_len - 1;
    let index = |i0, i1| cache_mask & (len1 * i0 + i1);

    // 0 == i0
    {
        /* 0 == i1
        cache[index(0, 0)] = 0;
        */

        // 1 <= i1 < len1
        for i1 in 1..len1 {
            cache[index(0, i1)] = (bytes1[i1 - 1] as i32) + cache[index(0, i1 - 1)];
        }
    }

    // 1 <= i0 < len1
    for i0 in 1..len0 {
        // 0 == i1
        cache[index(i0, 0)] = (bytes0[i0 - 1] as i32) + cache[index(i0 - 1, 0)];

        // 1 <= i1 < len1
        for i1 in 1..len1 {
            cache[index(i0, i1)] = match bytes0[i0 - 1] == bytes1[i1 - 1] {
                true => cache[index(i0 - 1, i1 - 1)],
                false => min(
                    (bytes0[i0 - 1] as i32) + cache[index(i0 - 1, i1)],
                    (bytes1[i1 - 1] as i32) + cache[index(i0, i1 - 1)],
                ),
            };
        }
    }

    cache[index(len0 - 1, len1 - 1)]
}

#[cfg(test)]
mod tests {
    use super::minimum_delete_sum;

    #[test]
    fn test1() {
        assert_eq!(minimum_delete_sum("sea".as_bytes(), "eat".as_bytes()), 231)
    }

    #[test]
    fn test2() {
        assert_eq!(
            minimum_delete_sum("delete".as_bytes(), "leet".as_bytes()),
            403
        )
    }
}
