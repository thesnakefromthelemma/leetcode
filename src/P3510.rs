use std::cmp::{
    Ordering,
    Ordering::{Equal, Greater, Less},
};
use std::ops::Add;

struct Ref {
    head: usize,
    tail: usize,
}

struct Link<T> {
    val: T,
    next: usize,
}

#[derive(Clone)]
#[derive(Copy)]
struct Pair<T> {
    fst: T,
    snd: T,
    ind: usize,
}

// Assumes 0 <= ind < min_heap.len()
fn min_heap_step_at<T: Copy + Ord + Add<Output = T>>(
    ref_list: &mut [Ref],
    min_heap: &mut [Pair<T>],
    mut ind: usize,
) {
    // Comparison for pairs
    let compare = |p0: &Pair<T>, p1: &Pair<T>| {
        let naive = (p0.fst + p0.snd).cmp(&(p1.fst + p1.snd));
        match naive {
            Equal => p0.ind.cmp(&p1.ind),
            _ => naive,
        }
    };

    // Heapification withg reference update
    let sze = min_heap.len();
    let val = min_heap[ind];
    loop {
        match sze.cmp(&(2 * ind + 2)) {
            Less => {
                min_heap[ind] = val;
                break;
            },
            Equal => {
                let ind_left = 2 * ind + 1;
                let val_left = min_heap[ind_left];
                match compare(&val, &val_left) {
                    Greater => {
                        min_heap[ind] = val_left;
                        ind = ind_left;
                    }
                    _ => {
                        min_heap[ind] = val;
                        break;
                    }
                }
            }
            Greater => {
                let ind_left = 2 * ind + 1;
                let ind_right = 2 * ind + 2;
                let val_left = min_heap[ind_left];
                let val_right = min_heap[ind_left];
                match (
                    compare(&val, &val_left),
                    compare(&val, &val_right),
                    compare(&val_left, &val_right),
                ) {
                    (Greater, _, Less) => {
                        min_heap[ind] = val_left;
                        ind = ind_left;
                    }
                    (_, Greater, _) => {
                        min_heap[ind] = val_right;
                        ind = ind_right;
                    }
                    _ => {
                        min_heap[ind] = val;
                        let ref_head = val.fst;
                        let ref_tail = val.snd;
                        ref_list[ref_head].head = ind;
                        ref_list[ref_tail].tail = ind;                        
                        break;
                    }
                }
            }
        }
    }
}

pub fn minimum_pair_removal(nums: &[i32]) -> i32 {
    let mut count_inv: u32 = 0;
    let mut count_rem = 0;

    // Allocate and construct link_list
    let mut link_list = Vec::with_capacity(nums.len());
    for i in 0..(nums.len() - 1) {
        link_list.push(Link {
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
        // min_heap_step_at(&mut min_heap[..], i);
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
