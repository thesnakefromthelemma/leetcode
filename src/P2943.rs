use std::cmp::{max, min};

// What a stupid fucking problem!
pub fn maximize_square_hole_area(
    _n: i32,
    _m: i32,
    mut h_bars: Vec<i32>,
    mut v_bars: Vec<i32>,
) -> i32 {
    h_bars.sort();
    let mut max_stretch_h = 1;
    let mut cur_stretch_h = 2;
    for i in 1..h_bars.len() {
        match h_bars[i - 1] + 1 == h_bars[i] {
            true => cur_stretch_h += 1,
            false => {
                max_stretch_h = max(max_stretch_h, cur_stretch_h);
                cur_stretch_h = 2;
            }
        }
    }
    max_stretch_h = max(max_stretch_h, cur_stretch_h);

    v_bars.sort();
    let mut max_stretch_v = 1;
    let mut cur_stretch_v = 2;
    for i in 1..v_bars.len() {
        match v_bars[i - 1] + 1 == v_bars[i] {
            true => cur_stretch_v += 1,
            false => {
                max_stretch_v = max(max_stretch_v, cur_stretch_v);
                cur_stretch_v = 2;
            }
        }
    }
    max_stretch_v = max(max_stretch_v, cur_stretch_v);

    min(max_stretch_v, max_stretch_h) * min(max_stretch_v, max_stretch_h)
}

#[cfg(test)]
mod tests {
    use super::maximize_square_hole_area;

    #[test]
    fn test1() {
        assert_eq!(maximize_square_hole_area(2, 1, vec![2, 3], vec![2]), 4)
    }

    #[test]
    fn test2() {
        assert_eq!(maximize_square_hole_area(1, 1, vec![2], vec![2]), 4)
    }

    #[test]
    fn test3() {
        assert_eq!(maximize_square_hole_area(2, 3, vec![2, 3], vec![2, 4]), 4)
    }
}
