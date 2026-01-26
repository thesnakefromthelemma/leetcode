use std::cmp::max;

// Suboptimal solution! Just saving for future ref...
// what the FUCK is this type???
pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
    let len = matrix.len();
    let len_row = matrix[0].len();

    let mut nursery = Vec::new();
    nursery.resize(len_row * (len_row + 1) / 2, 0);
    let index = |lwr, upr| lwr + upr * (upr - 1) / 2;

    let mut area_max = 0;

    // 0 <= i < len
    for i in 0..len {
        let mut lag = 0;
        for upr in 1..(len_row + 1) {
            match matrix[i][upr - 1] {
                '0' => {
                    lag = upr;
                    for lwr in 0..upr {
                        area_max = max(area_max, nursery[index(lwr, upr)]);
                        nursery[index(lwr, upr)] = 0;
                    }
                }
                '1' => {
                    for lwr in 0..lag {
                        area_max = max(area_max, nursery[index(lwr, upr)]);
                        nursery[index(lwr, upr)] = 0;
                    }
                    for lwr in lag..upr {
                        nursery[index(lwr, upr)] += (upr - lwr) as i32;
                        area_max = max(area_max, nursery[index(lwr, upr)]);
                    }
                }
                _ => panic!("Entry other than '0' or '1'."),
            }
        }
    }

    area_max
}

#[cfg(test)]
mod tests {
    use super::maximal_rectangle;

    #[test]
    fn test1() {
        assert_eq!(
            maximal_rectangle(vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0']
            ]),
            6
        )
    }

    #[test]
    fn test2() {
        assert_eq!(maximal_rectangle(vec![vec!['0']]), 0)
    }

    #[test]
    fn test3() {
        assert_eq!(maximal_rectangle(vec![vec!['1']]), 1)
    }
}