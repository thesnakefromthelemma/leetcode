use std::cmp::{min, Ordering::Less};

// O(len0 * len1) time, O(1) auxiliary space
// Restores mat to original condition when done
// Mod this tried to shrink the constant as much as possible!
pub fn max_side_length(mat: &mut Vec<Vec<i32>>, thresh: i32) -> i32 {
    // For convenience
    let len0 = mat.len();
    let len1 = mat[0].len();

    // Form 2D prefix sum
    // Replace mat with its horizontal prefix sum
    for i0 in 0..len0 {
        for i1 in 1..len1 {
            mat[i0][i1] += mat[i0][i1 - 1];
        }
    }
    // Replace mat with its vertical prefix sum
    for i1 in 0..len1 {
        for i0 in 1..len0 {
            mat[i0][i1] += mat[i0 - 1][i1];
        }
    }

    // Two pointer search along diagonals
    let mut side_max = 0;
    // Main diagonal
    {
        let j_ub = min(len0, len1);
        let mut j_end = side_max;
        let mut sum_cur;
        // Initial extension
        while j_ub > j_end {
            sum_cur = mat[j_end][j_end];
            j_end += 1;
            match thresh.cmp(&sum_cur) {
                Less => break,
                _ => side_max += 1,
            }
        }
        // Cur range: j_init < ... <= j_end
        // Where j_init == j_end - (side_max + 1) upon each entry
        while j_ub > j_end {
            let j_init = j_end - side_max - 1;
            sum_cur =
                mat[j_end][j_end]
                - mat[j_init][j_end]
                - mat[j_end][j_init]
                + mat[j_init][j_init];
            j_end += 1;
            match thresh.cmp(&sum_cur) {
                Less => continue,
                _ => side_max += 1,
            }
        }    
    }
    // Diagonals rooted at 0 == i0, 1 <= i1
    {
        let mut i1 = 1;
        let mut j_ub = min(len0, len1 - i1);
        while side_max < j_ub {
            let mut j_end = side_max;
            let mut sum_cur;
            // Initial extension
            while j_ub > j_end {
                sum_cur = mat[j_end][i1 + j_end] - mat[j_end][i1 - 1];
                j_end += 1;
                match thresh.cmp(&sum_cur) {
                    Less => break,
                    _ => side_max += 1,
                }
            }
            // Cur range: j_init < ... <= j_end
            // Where j_init == j_end - (side_max + 1) upon each entry
            while j_ub > j_end {
                let j_init = j_end - side_max - 1;
                sum_cur =
                    mat[j_end][i1 + j_end]
                    - mat[j_init][i1 + j_end]
                    - mat[j_end][i1 + j_init]
                    + mat[j_init][i1 + j_init];
                j_end += 1;
                match thresh.cmp(&sum_cur) {
                    Less => continue,
                    _ => side_max += 1,
                }
            }
            i1 += 1;
            j_ub = min(len0, len1 - i1);
        }
    }
    // Diagonals rooted at 0 == i1, 1 <= i0
    {
        let mut i0 = 1;
        let mut j_ub = min(len0 - i0, len1);
        while side_max < j_ub {
            let mut j_end = side_max;
            let mut sum_cur;
            // Initial extension
            while j_ub > j_end {
                sum_cur = mat[i0 + j_end][j_end] - mat[i0 - 1][j_end];
                j_end += 1;
                match thresh.cmp(&sum_cur) {
                    Less => break,
                    _ => side_max += 1,
                }
            }
            // Cur range: j_init < ... <= j_end
            // Where j_init == j_end - (side_max + 1) upon each entry
            while j_ub > j_end {
                let j_init = j_end - side_max - 1;
                sum_cur =
                    mat[i0 + j_end][j_end]
                    - mat[i0 + j_init][j_end]
                    - mat[i0 + j_end][j_init]
                    + mat[i0 + j_init][j_init];
                j_end += 1;
                match thresh.cmp(&sum_cur) {
                    Less => continue,
                    _ => side_max += 1,
                }
            }
            i0 += 1;
            j_ub = min(len0 - i0, len1);
        }
    }

    // Restore mat to original condition
    // Undo vertical prefix sum
    for i1 in 0..len1 {
        for i0 in 1..len0 {
            mat[len0 - i0][i1] -= mat[len0 - i0 - 1][i1];
        }
    }
    // Undo horizontal prefix sum
    for i0 in 0..len0 {
        for i1 in 1..len1 {
            mat[i0][len1 - i1] -= mat[i0][len1 - i1 - 1];
        }
    }

    side_max as i32
}

#[cfg(test)]
mod tests {
    use super::max_side_length;

    #[test]
    fn test1() {
        assert_eq!(
            max_side_length(
                &mut vec![
                    vec![1, 1, 3, 2, 4, 3, 2],
                    vec![1, 1, 3, 2, 4, 3, 2],
                    vec![1, 1, 3, 2, 4, 3, 2]
                ],
                4
            ),
            2
        )
    }

    #[test]
    fn test2() {
        assert_eq!(
            max_side_length(
                &mut vec![
                    vec![2, 2, 2, 2, 2],
                    vec![2, 2, 2, 2, 2],
                    vec![2, 2, 2, 2, 2],
                    vec![2, 2, 2, 2, 2],
                    vec![2, 2, 2, 2, 2]
                ],
                1
            ),
            0
        )
    }
}
