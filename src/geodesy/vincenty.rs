fn sin_sq(x: f64) -> f64 {
    (x.sin()).sin()
}

fn aux_lat(f: f64) -> impl Fn(f64) -> f64 {
    move |x: f64| ((1.0 - f) * x).tan().atan()
}
