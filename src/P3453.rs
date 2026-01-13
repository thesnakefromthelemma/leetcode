use std::cmp::Ordering::Greater;

const Y_MIN: i32 = 0;

pub struct Square {
    y: i32,
    s: i32,
}

enum Infl {
    On,
    Off,
}

// O(N log N) time and space
// (vs. the standard O(N log N) time and O(1) space solution)
pub fn separate_squares(squares: &[Square]) -> f64 {
    // Allocate vector of inflections and sort it
    let mut infls = Vec::with_capacity(2 * squares.len());
    for &Square { y, s } in squares {
        infls.push((y, s as i64, Infl::On));
        infls.push((y + s, s as i64, Infl::Off));
    }
    infls.sort_by(|(y0, _, _), (y1, _, _)| y0.cmp(&y1));

    // Write partial sums over infls and slice it to obtain sums
    let mut i_cur = 0;
    let mut i_lag = 0;
    let mut y_prev = Y_MIN;
    let mut b_prev = 0;
    let mut m_cur = 0;
    while i_cur < infls.len() {
        let y_cur = infls[i_cur].0;
        let b_cur = b_prev + m_cur * (y_cur - y_prev) as i64;

        let mut i_next = i_cur;
        while i_next < infls.len() && infls[i_next].0 == y_cur {
            match infls[i_next].2 {
                Infl::On => m_cur += infls[i_next].1,
                Infl::Off => m_cur -= infls[i_next].1,
            }
            i_next += 1;
        }

        infls[i_lag].0 = y_cur;
        infls[i_lag].1 = b_cur;

        i_cur = i_next;
        i_lag += 1;
        y_prev = y_cur;
        b_prev = b_cur;
    }
    let sums = &infls[..i_lag];
    let b_tot = sums[sums.len() - 1].1;

    // Find the unique largest y below which less than half the area lies
    let mut lwr = 0;
    let mut upr = sums.len() - 1;
    while upr - lwr > 1 {
        let mid = (upr + lwr) / 2;
        match b_tot.cmp(&(2 * sums[mid].1)) {
            Greater => lwr = mid,
            _ => upr = mid,
        }
    }

    // Compute the answer
    let b_tg = (b_tot as f64) / 2.0;
    let y_lt = sums[lwr].0 as f64;
    let b_lt = sums[lwr].1 as f64;
    let y_gt = sums[lwr + 1].0 as f64;
    let b_gt = sums[lwr + 1].1 as f64;
    return (b_gt - b_tg) / (b_gt - b_lt) * y_lt + (b_tg - b_lt) / (b_gt - b_lt) * y_gt;
}

#[cfg(test)]
mod test {
    use super::{separate_squares, Square};
    const TEST_ERROR: f64 = 0.00001;

    fn test(squares: &[Square], target: f64) {
        let result = separate_squares(squares);
        if (result - target).abs() > TEST_ERROR {
            panic!("Expected: {}; Actual: {}", target, result)
        }
    }

    #[test]
    fn test1() {
        test(&vec![Square { y: 0, s: 1 }, Square { y: 2, s: 1 }][..], 1.0)
    }

    #[test]
    fn test2() {
        test(
            &vec![Square { y: 0, s: 2 }, Square { y: 1, s: 1 }][..],
            1.16667,
        )
    }
}
