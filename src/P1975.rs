use std::cmp::{max, min, Ordering::Greater};

const MAX_SIZE: i64 = 100001;
const MIN_SIZE: i64 = -100001;

// Why do the fns always have the most fucked up types?
pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
    let mut neg_count = 0;
    let mut min_nonneg = MAX_SIZE;
    let mut max_neg = MIN_SIZE;
    let mut abs_sum = 0;

    for row in matrix {
        for entry in row {
            match 0.cmp(&entry) {
                Greater => {
                    neg_count += 1;
                    max_neg = max(max_neg, entry as i64);
                    abs_sum -= entry as i64;
                }
                _ => {
                    min_nonneg = min(min_nonneg, entry as i64);
                    abs_sum += entry as i64;
                }
            }
        }
    }

    match neg_count % 2 {
        0 => abs_sum,
        _ => abs_sum + 2 * max(max_neg, -min_nonneg),
    }
}

#[cfg(test)]
mod tests {
    use super::max_matrix_sum;

    #[test]
    fn test1() {
        assert_eq!(max_matrix_sum(vec![vec![1, -1], vec![-1, 1]]), 4)
    }

    #[test]
    fn test2() {
        assert_eq!(
            max_matrix_sum(vec![vec![1, 2, 3], vec![-1, -2, -3], vec![1, 2, 3]]),
            16
        )
    }
}
