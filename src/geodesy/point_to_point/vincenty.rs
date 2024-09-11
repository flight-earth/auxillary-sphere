use std::f64::consts::PI;

use crate::{earth::ellipsoid::{flattening, polar_r, Ellipsoid}, geodesy::{latlng::LatLng, problems::{Az, Dist, InverseProblem, InverseSolution}, vincenty::GeodeticAccuracy}, units::{Meter, Rad, Radius}};

#[derive(Debug, Clone, Copy)]
enum GeodeticInverse {
    GeodeticInverse(InverseSolution),
    GeodeticInverseAntipodal,
    GeodeticInverseAbnormal,
}

#[derive(Debug, Clone, Copy)]
struct InverseStep {
    tolerance: GeodeticAccuracy,
    a: f64,
    b: f64,
    f: f64,
    l: f64,
    sin_u1: f64,
    sin_u2: f64,
    cos_u1: f64,
    cos_u2: f64,
    sin_u1_sin_u2: f64,
    cos_u1_cos_u2: f64,
}

impl InverseStep {
    fn iloop(&self, lambda: f64) -> GeodeticInverse {
        if lambda.abs() > PI {
            return GeodeticInverse::GeodeticInverseAntipodal;
        }

        let sin_lambda = lambda.sin();
        let cos_lambda = lambda.cos();

        let i_prime = self.cos_u1 * sin_lambda;
        let j_prime = -self.sin_u1 * self.cos_u2 + self.cos_u1 * self.sin_u2 * cos_lambda;

        let i = self.cos_u2 * sin_lambda;
        let j = self.cos_u1 * self.sin_u2 - self.sin_u1 * self.cos_u2 * cos_lambda;

        let sin2_sigma = i * i + j * j;
        let sin_sigma = sin2_sigma.sqrt();
        let cos_sigma = self.sin_u1_sin_u2 + self.cos_u1_cos_u2 * cos_lambda;

        let sigma = sin_sigma.atan2(cos_sigma);

        let sin_alpha = self.cos_u1_cos_u2 * sin_lambda / sin_sigma;
        let cos2_alpha = 1.0 - sin_alpha * sin_alpha;
        let c = self.f / 16.0 * cos2_alpha * (4.0 + self.f * (4.0 - 3.0 * cos2_alpha));
        let u2 = cos2_alpha * (self.a * self.a - self.b * self.b) / (self.b * self.b);

        let cos2_sigma_m = if cos2_alpha == 0.0 { 0.0 } else { cos_sigma - 2.0 * self.sin_u1_sin_u2 / cos2_alpha };
        let cos2_2_sigma_m = cos2_sigma_m * cos2_sigma_m;

        let a = 1.0 + u2 / 16384.0 * (4096.0 + u2 * (-768.0 + u2 * (320.0 - 175.0 * u2)));
        let b = u2 / 1024.0 * (256.0 + u2 * (-128.0 + u2 * (74.0 - 47.0 * u2)));

        let y = cos_sigma * (-1.0 + 2.0 * cos2_2_sigma_m) - b / 6.0 * cos2_sigma_m * (-3.0 + 4.0 * sin2_sigma) * (-3.0 + 4.0 * cos2_2_sigma_m);

        let delta_sigma = b * sin_sigma * (cos2_sigma_m + b / 4.0 * y);

        let x = cos2_sigma_m + c * cos_sigma * (-1.0 + 2.0 * cos2_2_sigma_m);
        let lambda_prime = self.l + (1.0 - c) * self.f * sin_alpha * (sigma + c * sin_sigma * x);

        let s = self.b * a * (sigma - delta_sigma);
        let alpha1 = i.atan2(j);
        let alpha2 = i_prime.atan2(j_prime);

        if (lambda - lambda_prime).abs() >= self.tolerance.accuracy {
            self.iloop(lambda_prime)
        } else {
            GeodeticInverse::GeodeticInverse(InverseSolution {
                s: Dist{dist:s},
                az1: Az{az:alpha1},
                az2: Some(Az{az:alpha2}),
            })
        }
    }
}

fn normalize_lng(lng: f64) -> f64 {
    lng % (2.0 * PI)
}

fn inverse(ellipsoid: Ellipsoid, tolerance: GeodeticAccuracy, p: InverseProblem) -> GeodeticInverse {
    let Rad(phi1) = p.x.lat;
    let Rad(l1) = p.x.lng;
    let Rad(phi2) = p.y.lat;
    let Rad(l2) = p.y.lng;

    let Radius(Meter(a)) = ellipsoid.equatorial_r;
    let Radius(Meter(b)) = polar_r(&ellipsoid);
    let f = flattening(&ellipsoid);

    let aux_lat = |phi: f64| ((1.0 - f) * phi.tan()).atan();
    let u1 = aux_lat(phi1);
    let u2 = aux_lat(phi2);

    let l = {
        let l_prime = l2 - l1;
        if l_prime.abs() <= PI {
            l_prime
        } else {
            normalize_lng(l2) - normalize_lng(l1)
        }
    };

    let sin_u1 = u1.sin();
    let sin_u2 = u2.sin();
    let cos_u1 = u1.cos();
    let cos_u2 = u2.cos();
    let sin_u1_sin_u2 = sin_u1 * sin_u2;
    let cos_u1_cos_u2 = cos_u1 * cos_u2;

    let step = InverseStep {
        tolerance,
        a,
        b,
        f,
        l,
        sin_u1,
        sin_u2,
        cos_u1,
        cos_u2,
        sin_u1_sin_u2,
        cos_u1_cos_u2,
    };

    step.iloop(l)
}

fn distance_unchecked(ellipsoid: Ellipsoid, prob: InverseProblem) -> GeodeticInverse {
    if prob.x == prob.y {
        GeodeticInverse::GeodeticInverse(InverseSolution {
            s: Dist{dist: 0.0},
            az1: Az{az:0.0},
            az2: Some(Az{az:PI}),
        })
    } else {
        inverse(ellipsoid, GeodeticAccuracy { accuracy: 1e-12 }, prob)
    }
}

pub(crate) fn distance(e: Ellipsoid, x: LatLng, y: LatLng) -> Result<Dist, String> {
    let prob = InverseProblem { x, y };
    match distance_unchecked(e, prob) {
        GeodeticInverse::GeodeticInverse(solution) => Ok(solution.s),
        _ => Err("Distance calculation failed".to_string()),
    }
}