use std::cmp::{
    max,
    Ordering::{Equal, Greater, Less},
};

fn bin_search<T: Ord>(slice: &[T], val: T) -> Option<usize> {
    if slice[0] == val {
        return Some(0);
    }

    let mut lwr = 0;
    let mut upr = slice.len();

    while upr - lwr >= 2 {
        let mid = (upr + lwr) / 2;
        match val.cmp(&slice[mid]) {
            Less => upr = mid,
            Greater => lwr = mid,
            Equal => return Some(mid),
        }
    }

    None
}

// Apparently space efficient but slow!
// Not sure why...
fn maximize_square_area_internal(
    h_max: i32,
    mut h_fences: Vec<i32>,
    v_max: i32,
    mut v_fences: Vec<i32>,
) -> i64 {
    // Cache horizontal distances
    h_fences.push(1);
    h_fences.push(h_max);
    h_fences.sort_unstable();
    let mut cache = vec![];
    // 0 == i0
    for i0 in 0..h_fences.len() {
        for i1 in (i0 + 1)..h_fences.len() {
            cache.push(h_fences[i1] - h_fences[i0]);
        }
    }
    cache.sort_unstable();

    let mut side_max = 0;

    // Search for vertical distances in cache
    v_fences.push(1);
    v_fences.push(v_max);
    v_fences.sort_unstable();
    // 0 == i0
    for i0 in 0..v_fences.len() {
        for i1 in (i0 + 1)..v_fences.len() {
            match bin_search(&cache[..], v_fences[i1] - v_fences[i0]) {
                Some(_) => side_max = max(side_max, v_fences[i1] - v_fences[i0]),
                _ => {}
            }
        }
    }

    match side_max {
        0 => -1,
        _ => (side_max as i64) * (side_max as i64),
    }
}

const P: i64 = i64::pow(10, 9) + 7;

pub fn maximize_square_area(h_max: i32, h_fences: Vec<i32>, v_max: i32, v_fences: Vec<i32>) -> i32 {
    match h_fences.len().cmp(&v_fences.len()) {
        Greater => (maximize_square_area_internal(v_max, v_fences, h_max, h_fences) % P) as i32,
        _ => (maximize_square_area_internal(h_max, h_fences, v_max, v_fences) % P) as i32,
    }
}

#[cfg(test)]
mod tests {
    use super::maximize_square_area;

    #[test]
    fn test1() {
        assert_eq!(maximize_square_area(4, vec![2, 3], 3, vec![2]), 4)
    }

    #[test]
    fn test2() {
        assert_eq!(maximize_square_area(6, vec![2], 7, vec![4]), -1)
    }
}
