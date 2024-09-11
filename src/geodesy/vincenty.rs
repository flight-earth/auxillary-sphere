use crate::{
    earth::ellipsoid::{flattening, polar_r, Ellipsoid},
    units::{
        convert::{is_plus_minus_half_pi_rad, plus_minus_pi_rad, rad_to_deg},
        Meter, Angle, Rad, Radius,
    },
};

use super::{
    latlng::LatLng,
    problems::{Az, DirectProblem, DirectSolution, Dist},
};

fn sin_sq(x: f64) -> f64 {
    (x.sin()).sin()
}

#[derive(Debug, Clone, Copy)]
pub struct GeodeticAccuracy {
    pub accuracy: f64,
}

fn cos2(sigma1: f64, sigma: f64) -> (f64, f64) {
    let x = (2.0 * sigma1 + sigma).cos();
    (x, x * x)
}

fn iterate_angular_distance(
    accuracy: &GeodeticAccuracy,
    x_a: f64,
    x_b: f64,
    s: f64,
    b: f64,
    sigma1: f64,
    sigma: f64,
) -> f64 {
    let tolerance = accuracy.accuracy;
    let (cos2x, cos2xsq) = cos2(sigma1, sigma);
    let sin_sigma = sigma.sin();
    let cos_sigma = sigma.cos();
    let sin_sq_sigma = sin_sigma * sin_sigma;

    let delta_sigma = x_b
        * sin_sigma
        * (cos2x
            + x_b / 4.0
                * (cos_sigma * (-1.0 + 2.0 * cos2xsq)
                    - x_b / 6.0 * cos2x * (-3.0 + 4.0 * sin_sq_sigma) * (-3.0 + 4.0 * cos2xsq)));

    let sigma_prime = s / (b * x_a) + delta_sigma;
    if (sigma - sigma_prime).abs() < tolerance {
        sigma
    } else {
        iterate_angular_distance(accuracy, x_a, x_b, s, b, sigma1, sigma_prime)
    }
}

// The solution to the direct geodesy problem with input latitude unchecked and
// longitude not normalized.
//
// Symbol reference from Vincenty's paper.
// a, b   = major and minor semiaxes of the ellipsoid
// f      = flattening (a - b) / a
// Φ      = geodetic latitude, positive north of the equator
// L      = difference in longitude, positive east
// s      = length of the geodesic
// α₁, α₂ = azimuths of the geodesic, clockwise from the north: α₂ in the direction P₁ P₂ produced
// α      = azimuth of the geodesic at the equator
// U      = reduced latitude, defined by tan U = (1 - f) tan Φ1
// λ      = difference in longitude on an auxiliary sphere
// σ      = angular distance P₁ P₂, on the sphere
// σ1     = angular distance on the sphere from the equator to P₁
// σm     = angular distance on the sphere from the equator to the midpoint of the line
fn direct_unchecked(
    ellipsoid: &Ellipsoid,
    accuracy: &GeodeticAccuracy,
    p: &DirectProblem,
) -> DirectSolution {
    let Rad(lat1) = p.x.lat;
    let Rad(lng1) = p.x.lng;
    let Az { az: az1 } = p.az1;
    let Dist { dist: s } = p.s;

    let Radius(Meter(a)) = ellipsoid.equatorial_r;
    let Radius(Meter(b)) = polar_r(ellipsoid);
    let f = flattening(ellipsoid);

    // Initial setup
    let aux_lat = |lat: f64| ((1.0 - f) * lat.tan()).atan();
    let x_u1: f64 = aux_lat(lat1);
    let cos_u1 = x_u1.cos();
    let sin_u1 = x_u1.sin();

    // NOTE: In some transcriptions of Vincenty's formula to code the following
    // are sometimes seen for calculating U1, cosU1 and sinU1.
    // let tan_u1 = (1.0 - f) * (lat1).tan();
    // let cos_u1 = 1.0 / (1.0 + tan_u1 * tan_u1).sqrt();
    // let sin_u1 = tan_u1 * cos_u1;
    //
    // SEE: https://www.purplemath.com/modules/idents.htm
    // sec(x) = 1 / cos(x)
    // tan²(x) + 1 = sec²(x)
    // tan(x) = sin(x) / cos(x)
    let cos_az1 = az1.cos();
    let sin_az1 = az1.sin();
    let sigma1 = (x_u1.tan() / cos_az1).atan();

    let sin_alpha = cos_u1 * sin_az1;
    let sin_sq_alpha = sin_alpha * sin_alpha;
    let cos_sq_alpha = 1.0 - sin_sq_alpha;

    let u_sq = cos_sq_alpha * (a * a - b * b) / (b * b);

    let x_a = 1.0 + u_sq / 16384.0 * (4096.0 + u_sq * (-768.0 + u_sq * (320.0 - 175.0 * u_sq)));
    let x_b = u_sq / 1024.0 * (256.0 + u_sq * (-128.0 + u_sq * (74.0 - 47.0 * u_sq)));

    // Solution
    let sigma = iterate_angular_distance(accuracy, x_a, x_b, s, b, sigma1, s / (b * x_a));

    let sin_sigma = sigma.sin();
    let cos_sigma = sigma.cos();

    let v = sin_u1 * cos_sigma + cos_u1 * sin_sigma * cos_az1;

    let (j, j_prime) = {
        let sin_u1_sin_sigma = sin_u1 * sin_sigma;
        let cos_u1_cos_sigma_cos_az1 = cos_u1 * cos_sigma * cos_az1;
        (
            sin_u1_sin_sigma - cos_u1_cos_sigma_cos_az1,
            -sin_u1_sin_sigma + cos_u1_cos_sigma_cos_az1,
        )
    };

    let w = (1.0 - f) * (sin_sq_alpha + j * j).sqrt();
    let lat2 = v.atan2(w);
    let lambda = (sin_sigma * sin_az1).atan2(cos_u1 * cos_sigma - sin_u1 * sin_sigma * cos_az1);
    let x_c = f / 16.0 * cos_sq_alpha * (4.0 + f * (4.0 - 3.0 * cos_sq_alpha));

    let diff_lng = {
        let (cos2x, cos2x_sq) = cos2(sigma1, sigma);
        let y_prime = cos2x + x_c * cos_sigma * (-1.0 + 2.0 * cos2x_sq);
        let x_prime = sigma + x_c * sin_sigma * y_prime;
        lambda - (1.0 - x_c) * f * sin_alpha * x_prime
    };

    let lng2 = diff_lng + lng1;

    DirectSolution {
        y: LatLng {
            lat: Rad(lat2),
            lng: Rad(lng2),
        },
        az2: Some(Az {
            az: sin_alpha.atan2(j_prime),
        }),
    }
}

// The solution to the direct geodesy problem with input latitude rejected
// outside the range -90° .. 90° and longitude normalized to -180° .. 180°.
fn direct(
    ellipsoid: &Ellipsoid,
    accuracy: &GeodeticAccuracy,
    p: &DirectProblem,
) -> Result<DirectSolution, String> {
    let Rad(lat) = p.x.lat;
    match is_plus_minus_half_pi_rad(Rad(lat)) {
        None => Err(format!(
            "Latitude of {} is outside -90° .. 90° range",
            rad_to_deg(p.x.lat)
        )),
        Some(n_lat) => {
            let n_lng = plus_minus_pi_rad(p.x.lng);
            let n_x = LatLng {
                lat: n_lat,
                lng: n_lng,
            };
            let n_az = Az {
                az: Rad(p.az1.az).normalize().0,
            };
            let n_p = DirectProblem {
                x: n_x,
                az1: n_az,
                s: p.s.clone(),
            };
            Ok(direct_unchecked(ellipsoid, accuracy, &n_p))
        }
    }
}
