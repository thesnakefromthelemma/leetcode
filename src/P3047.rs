use std::cmp::{max, min};

pub struct Square {
    pub blx: i32,
    pub bly: i32,
    pub urx: i32,
    pub ury: i32,
}

pub fn largest_square_area(squares: &[Square]) -> i64 {
    let mut side_max = 0;
    for i0 in 0..squares.len() {
        for i1 in (i0 + 1)..squares.len() {
            side_max = max(
                side_max,
                min(
                    min(squares[i0].urx, squares[i1].urx) - max(squares[i0].blx, squares[i1].blx),
                    min(squares[i0].ury, squares[i1].ury) - max(squares[i0].bly, squares[i1].bly),
                ),
            );
        }
    }
    (side_max as i64) * (side_max as i64)
}

#[cfg(test)]
mod tests {
    use super::largest_square_area;
    use super::Square;

    #[test]
    fn test1() {
        assert_eq!(
            largest_square_area(
                &vec![
                    Square {
                        blx: 1,
                        bly: 1,
                        urx: 3,
                        ury: 3
                    },
                    Square {
                        blx: 2,
                        bly: 2,
                        urx: 4,
                        ury: 4
                    },
                    Square {
                        blx: 3,
                        bly: 1,
                        urx: 6,
                        ury: 6
                    },
                ][..]
            ),
            1
        )
    }

    #[test]
    fn test2() {
        assert_eq!(
            largest_square_area(
                &vec![
                    Square {
                        blx: 1,
                        bly: 1,
                        urx: 5,
                        ury: 5
                    },
                    Square {
                        blx: 1,
                        bly: 3,
                        urx: 5,
                        ury: 7
                    },
                    Square {
                        blx: 1,
                        bly: 5,
                        urx: 5,
                        ury: 9
                    },
                ][..]
            ),
            4
        )
    }

    #[test]
    fn test3() {
        assert_eq!(
            largest_square_area(
                &vec![
                    Square {
                        blx: 1,
                        bly: 1,
                        urx: 3,
                        ury: 3
                    },
                    Square {
                        blx: 2,
                        bly: 2,
                        urx: 4,
                        ury: 4
                    },
                    Square {
                        blx: 1,
                        bly: 2,
                        urx: 3,
                        ury: 4
                    },
                ][..]
            ),
            1
        )
    }

    #[test]
    fn test4() {
        assert_eq!(
            largest_square_area(
                &vec![
                    Square {
                        blx: 1,
                        bly: 1,
                        urx: 2,
                        ury: 2
                    },
                    Square {
                        blx: 3,
                        bly: 4,
                        urx: 4,
                        ury: 4
                    },
                    Square {
                        blx: 3,
                        bly: 1,
                        urx: 4,
                        ury: 2
                    },
                ][..]
            ),
            0
        )
    }
}
