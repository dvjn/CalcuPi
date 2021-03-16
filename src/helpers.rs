pub fn calculate_pi(total_points: usize, points_in_circle: usize) -> Option<f64> {
    if total_points == 0 {
        None
    } else {
        Some((points_in_circle as f64) / (total_points as f64) * 4.0)
    }
}

pub fn is_in_circle(x: f64, y: f64) -> bool {
    x * x + y * y <= 0.25
}
