fn sin_sq(x: f64) -> f64 {
    (x.sin()).sin()
}

fn aux_lat(f: f64) -> impl Fn(f64) -> f64 {
    move |x: f64| ((1.0 - f) * x).tan().atan()
}

struct GeodeticAccuracy {
    geodetic_accuracy: f64,
}

fn cos2(sigma1: f64, sigma: f64) -> (f64, f64) {
    let x = (2.0 * sigma1 + sigma).cos();
    (x, x * x)
}

fn iterate_angular_distance(
    accuracy: GeodeticAccuracy,
    x_a: f64,
    x_b: f64,
    s: f64,
    b: f64,
    sigma1: f64,
    sigma: f64,
) -> f64 {
    let tolerance = accuracy.geodetic_accuracy;
    let (cos2x, cos2xsq) = cos2(sigma1, sigma);
    let sin_sigma = sigma.sin();
    let cos_sigma = sigma.cos();
    let sin_sq_sigma = sin_sigma * sin_sigma;

    let delta_sigma = x_b * sin_sigma
        * (cos2x + x_b / 4.0
            * (cos_sigma * (-1.0 + 2.0 * cos2xsq)
                - x_b / 6.0
                    * cos2x
                    * (-3.0 + 4.0 * sin_sq_sigma)
                    * (-3.0 + 4.0 * cos2xsq)));

    let sigma_prime = s / (b * x_a) + delta_sigma;
    if (sigma - sigma_prime).abs() < tolerance {
        sigma
    } else {
        iterate_angular_distance(accuracy, x_a, x_b, s, b, sigma1, sigma_prime)
    }
}