pub fn point_dist(p1: (i32, i32), p2: (i32, i32)) -> f32 {
    // (x_b - x_a)^2 + (y_b - y_a)^2 }
    (((p2.0 - p1.0).pow(2) + (p2.1 - p1.1).pow(2)) as f32).sqrt()
}
