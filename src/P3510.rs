use std::cmp::{
    Ordering,
    Ordering::{Equal, Greater, Less},
};

// Assumes 0 <= ind < heap.len()
// No 'Copy' trait imposed on T, but still safe
// (There results a minor missed optimization
// opportunity due to the failure to pass 'val' around)
fn min_heap_step_at_by<F: FnMut(&T, &T) -> Ordering, T>(
    mut compare: F,
    min_heap: &mut [T],
    mut ind: usize,
) {
    let sze = min_heap.len();
    loop {
        match sze.cmp(&(2 * ind + 2)) {
            Less => break,
            Equal => {
                let ind_left = 2 * ind + 1;
                let val = &min_heap[ind];
                let val_left = &min_heap[ind_left];
                match compare(&val, &val_left) {
                    Greater => {
                        min_heap.swap(ind, ind_left);
                        ind = ind_left;
                    }
                    _ => break,
                }
            }
            Greater => {
                let ind_left = 2 * ind + 1;
                let ind_right = 2 * ind + 2;
                let val = &min_heap[ind];
                let val_left = &min_heap[ind_left];
                let val_right = &min_heap[ind_left];
                match (
                    compare(val, val_left),
                    compare(val, val_right),
                    compare(val_left, val_right),
                ) {
                    (Greater, _, Less) => {
                        min_heap.swap(ind, ind_left);
                        ind = ind_left;
                    }
                    (_, Greater, _) => {
                        min_heap.swap(ind, ind_right);
                        ind = ind_right;
                    }
                    _ => break,
                }
            }
        }
    }
}

struct Node<T> {
    val: T,
    next: usize,
}

struct Pair<T> {
    fst: T,
    snd: T,
    ind: usize,
}

pub fn minimum_pair_removal(nums: &[i32]) -> i32 {
    let mut count_inv: u32 = 0;
    let mut count_rem = 0;

    // Comparison for pairs
    let compare = |p0: &Pair<i32>, p1: &Pair<i32>| {
        let naive = (p0.fst + p0.snd).cmp(&(p1.fst + p1.snd));
        match naive {
            Equal => p0.ind.cmp(&p1.ind),
            _ => naive,
        }
    };

    // Allocate and construct link_list
    let mut link_list = Vec::with_capacity(nums.len());
    for i in 0..(nums.len() - 1) {
        link_list.push(Node {
            next: i + 1,
            val: nums[i],
        });
    }

    // Allocate and construct ind_list
    let mut ind_list = Vec::with_capacity(nums.len() - 1);
    for i in 0..(nums.len() - 1) {
        ind_list.push(i);
    }

    // Allocate and construct min_heap
    let mut min_heap = Vec::with_capacity(nums.len() - 1);
    for i in 0..(nums.len() - 1) {
        match nums[i].cmp(&nums[i + 1]) {
            Greater => count_inv += 1,
            _ => { },
        }
        min_heap.push(Pair {
            fst: nums[i],
            snd: nums[i + 1],
            ind: i,
        });
    }
    let mut i = min_heap.len() / 2;
    while i > 0 {
        i -= 1;
        min_heap_step_at_by(compare, &mut min_heap[..], i);
    }

    // Fuse minimal pairs while unsorted
    while count_inv > 0 {
        unimplemented!();
    }

    count_rem
}

#[cfg(test)]
mod tests {
    use super::minimum_pair_removal;

    #[test]
    fn test1() {
        assert_eq!(minimum_pair_removal(&[5,2,3,1]), 2)
    }

    #[test]
    fn test2() {
        assert_eq!(minimum_pair_removal(&[1,2,2]), 0)
    }
}
