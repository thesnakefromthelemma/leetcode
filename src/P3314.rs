use std::cmp::Ordering::Less;

fn min_sat_val(num: i32) -> i32 {
    let mut num_clone = num;
    let mut count_sh = 0;
    loop {
        match num_clone & 1 {
            0 => break,
            _ => {
                num_clone = num_clone >> 1;
                count_sh += 1;
            }
        }
    }
    match 0.cmp(&count_sh) {
        Less => num - (1 << (count_sh - 1)),
        _ => -1,
    }
}

pub fn min_bitwise_array(nums: &mut Vec<i32>) {
    for i in 0..nums.len() {
        nums[i] = min_sat_val(nums[i]);
    }
}

#[cfg(test)]
mod tests {
    use super::min_bitwise_array;

    #[test]
    fn test1() {
        let mut vec = vec![2, 3, 5, 7];
        min_bitwise_array(&mut vec);
        assert_eq!(vec, vec![-1, 1, 4, 3])
    }

    #[test]
    fn test2() {
        let mut vec = vec![11, 13, 31];
        min_bitwise_array(&mut vec);
        assert_eq!(vec, vec![9, 12, 15])
    }
}
