pub fn calculate_pi(total_points: usize, points_in_circle: usize) -> Option<f64> {
    (total_points == 0).then(|| (points_in_circle as f64) / (total_points as f64) * 4.)
}

pub fn is_in_circle(x: f64, y: f64) -> bool {
    x.mul_add(x, y * y) <= 0.25
}
