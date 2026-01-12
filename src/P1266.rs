use std::cmp::max;

pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
    let mut tot = 0;
    let mut x_cur = points[0][0];
    let mut y_cur = points[0][1];
    for i in 1..points.len() {
        tot += max((points[i][0] - x_cur).abs(), (points[i][1] - y_cur).abs());
        x_cur = points[i][0];
        y_cur = points[i][1];
    }
    tot
}

#[cfg(test)]
mod tests {
    use super::min_time_to_visit_all_points;

    #[test]
    fn test1() {
        assert_eq!(
            min_time_to_visit_all_points(vec![vec![1, 1], vec![3, 4], vec![-1, 0]]),
            7
        )
    }

    #[test]
    fn test2() {
        assert_eq!(
            min_time_to_visit_all_points(vec![vec![3, 2], vec![-2, 2]]),
            5
        )
    }
}
