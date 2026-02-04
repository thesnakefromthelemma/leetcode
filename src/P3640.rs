use std::cmp::{
    min,
    max,
    Ordering::{
        Less, 
        Equal,
        Greater
    }
};

// Recursion state
#[derive(Clone, Copy)]
enum State<T> {
    Nil,
    OneInit {
        last: T,
    },
    OneCont {
        last: T,
        sum_init: T,
        sum_init_min: T,
    },
    Two {
        sum0: T,
        sum1: T,
    },
    ThrInit {
        sum0: T,
        sum1: T,
        last: T,
    },
    ThrCont {
        sum0: T,
        sum1: T,
        last: T,
        sum_init: T,
        sum_init_max: T,
        sum_init_min: T,       
    },
}

use State::*;

pub fn max_sum_trionic(nums: &[i32]) -> i64 {
    const SUM_MIN: i64 = -100_000_000_000_001;
    let len = nums.len();

    // Max sum of trionic subarray
    let mut sum_max = SUM_MIN;

    // Single pass iteration, highly branching
    let mut i = 0;
    let mut state = Nil;
    while len > i + 1 {
        match (state, nums[i].cmp(&nums[i + 1])) {
            (Nil, Less) => {
                state = OneInit {
                    last: nums[i] as i64,
                };
            },
            (Nil, _) => { },
            (OneInit { last }, Less) => {
                state = OneCont {
                    last: nums[i] as i64,
                    sum_init: last,
                    sum_init_min: min(0, last),
                };
            },
            (OneInit { last }, Greater) => {
                state = Two {
                    sum0: last + nums[i] as i64,
                    sum1: 0,
                };
            },
            (OneCont { last, sum_init, sum_init_min }, Less) => {
                state = OneCont {
                    last: nums[i] as i64,
                    sum_init: sum_init + last,
                    sum_init_min: min(sum_init_min, sum_init + last),
                };
            },
            (OneCont { last, sum_init, sum_init_min }, Greater) => {
                state = Two {
                    sum0: sum_init + last + nums[i] as i64 - sum_init_min,
                    sum1: 0,
                };
            },
            (Two { sum0, sum1 }, Greater) => {
                state = Two {
                    sum0: sum0,
                    sum1: sum1 + nums[i] as i64,
                };
            },
            (Two { sum0, sum1 }, Less) => {
                state = ThrInit {
                    sum0: sum0,
                    sum1: sum1,
                    last: nums[i] as i64,
                };
            },
            (ThrInit { sum0, sum1, last }, Less) => {
                state = ThrCont {
                    sum0: sum0,
                    sum1: sum1,
                    last: nums[i] as i64,
                    sum_init: last,
                    sum_init_max: last + nums[i] as i64,
                    sum_init_min: min(0, last),
                }
            },
            (ThrInit { sum0, sum1, last }, Greater) => {
                sum_max = max(sum_max, sum0 + sum1 + nums[i] as i64);
                state = Two {
                    sum0: last + nums[i] as i64,
                    sum1: 0,
                }
            },
            (ThrInit { sum0, sum1, last }, Equal) => {
                sum_max = max(sum_max, sum0 + sum1 + last + nums[i] as i64);
                state = Nil;
            },
            (ThrCont { sum0, sum1, last, sum_init, sum_init_max, sum_init_min }, Less) => {
                state = ThrCont {
                    sum0: sum0,
                    sum1: sum1,
                    last: nums[i] as i64,
                    sum_init: sum_init + last,
                    sum_init_max: max(sum_init_max, sum_init + last + nums[i] as i64),
                    sum_init_min: min(sum_init_min, sum_init + last),
                }
            },
            (ThrCont { sum0, sum1, last, sum_init, sum_init_max, sum_init_min }, Greater) => {
                let sum_init_fin = sum_init + last;
                let sum_init_max_fin = max(sum_init_max, sum_init_fin + nums[i] as i64);
                sum_max = max(sum_max, sum0 + sum1 + sum_init_max_fin);
                state = Two {
                    sum0: sum_init + last + nums[i] as i64 - sum_init_min,
                    sum1: 0,
                }
            },
            (ThrCont { sum0, sum1, last, sum_init, sum_init_max, .. }, Equal) => {
                let sum_init_fin = sum_init + last;
                let sum_init_max_fin = max(sum_init_max, sum_init_fin + nums[i] as i64);
                sum_max = max(sum_max, sum0 + sum1 + sum_init_max_fin);
                state = Nil;
            },
            _ => state = Nil,
        };
        i += 1;
    }
    
    // Include last value of nums if appropriate
    match state {
        ThrInit { sum0, sum1, last } => {
            let sum_init_max = last + nums[i] as i64;
            sum_max = max(sum_max, sum0 + sum1 + sum_init_max);
            sum_max
        } 
        ThrCont { sum0, sum1, last, sum_init, sum_init_max, .. } => {
            let sum_init_fin = sum_init + last;
            let sum_init_max_fin = max(sum_init_max, sum_init_fin + nums[i] as i64);
            sum_max = max(sum_max, sum0 + sum1 + sum_init_max_fin);
            sum_max
        }
        _ => sum_max,
    }
}

#[cfg(test)]
mod tests {
    use super::max_sum_trionic;

    #[test]
    fn test1() {
        assert_eq!(max_sum_trionic(&[0, -2, -1, -3, 0, 2, -1]), -4)
    }

    #[test]
    fn test2() {
        assert_eq!(max_sum_trionic(&[1, 4, 2, 7]), 14)
    }
}