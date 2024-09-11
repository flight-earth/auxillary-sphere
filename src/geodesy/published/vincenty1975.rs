// Test data from ...
//
// Direct and Inverse Solutions of Geodesics on the Ellipsoid with Applications
// of Nested Equations
// Survey Review XXII, 176
// T. Vincenty, April 1975.
use crate::{
    earth::ellipsoid::*,
    geodesy::{
        latlng::LatLng,
        problems::{Az, Dist, InverseProblem, InverseSolution},
    },
    units::{convert::deg_to_rad, Normalize, Rad, DMS},
};

// SEE: https://stackoverflow.com/questions/23810032/how-to-specify-const-array-in-global-scope-in-rust
pub static ELLIPSOIDS: &'static [Ellipsoid] = &[BESSEL, HAYFORD, HAYFORD, HAYFORD, HAYFORD];

// Distances in meters
pub static DISTANCES: &'static [f64] = &[
    14110526.170,
    4085966.703,
    8084823.839,
    19960000.000,
    19780006.558,
];

pub static X_AZIMUTHS: &'static [DMS] = &[
    DMS {
        deg: 96,
        min: 36,
        sec: 8.79960,
    },
    DMS {
        deg: 95,
        min: 27,
        sec: 59.63089,
    },
    DMS {
        deg: 15,
        min: 44,
        sec: 23.74850,
    },
    DMS {
        deg: 89,
        min: 0,
        sec: 0.0,
    },
    DMS {
        deg: 4,
        min: 59,
        sec: 59.99995,
    },
];

pub static Y_AZIMUTHS: &'static [DMS] = &[
    DMS {
        deg: 137,
        min: 52,
        sec: 22.01454,
    },
    DMS {
        deg: 118,
        min: 5,
        sec: 58.96161,
    },
    DMS {
        deg: 144,
        min: 55,
        sec: 39.92147,
    },
    DMS {
        deg: 91,
        min: 0,
        sec: 6.11733,
    },
    DMS {
        deg: 174,
        min: 59,
        sec: 59.88481,
    },
];

pub static INVERSE_PROBLEM_DATA: &'static [((DMS, DMS), (DMS, DMS))] = &[
    (
        (
            DMS {
                deg: 55,
                min: 45,
                sec: 0.00000,
            },
            DMS {
                deg: 0,
                min: 0,
                sec: 0.0,
            },
        ),
        (
            DMS {
                deg: -33,
                min: 26,
                sec: 0.00000,
            },
            DMS {
                deg: 108,
                min: 13,
                sec: 0.00000,
            },
        ),
    ),
    (
        (
            DMS {
                deg: 37,
                min: 19,
                sec: 54.95367,
            },
            DMS {
                deg: 0,
                min: 0,
                sec: 0.0,
            },
        ),
        (
            DMS {
                deg: 26,
                min: 7,
                sec: 42.83946,
            },
            DMS {
                deg: 41,
                min: 28,
                sec: 35.50729,
            },
        ),
    ),
    (
        (
            DMS {
                deg: 35,
                min: 16,
                sec: 11.24862,
            },
            DMS {
                deg: 0,
                min: 0,
                sec: 0.0,
            },
        ),
        (
            DMS {
                deg: 67,
                min: 22,
                sec: 14.77638,
            },
            DMS {
                deg: 137,
                min: 47,
                sec: 28.31435,
            },
        ),
    ),
    (
        (
            DMS {
                deg: 1,
                min: 0,
                sec: 0.00000,
            },
            DMS {
                deg: 0,
                min: 0,
                sec: 0.0,
            },
        ),
        (
            DMS {
                deg: 0,
                min: -59,
                sec: 53.83076,
            },
            DMS {
                deg: 179,
                min: 17,
                sec: 48.02997,
            },
        ),
    ),
    (
        (
            DMS {
                deg: 1,
                min: 0,
                sec: 0.00000,
            },
            DMS {
                deg: 0,
                min: 0,
                sec: 0.0,
            },
        ),
        (
            DMS {
                deg: 1,
                min: 1,
                sec: 15.18952,
            },
            DMS {
                deg: 179,
                min: 46,
                sec: 17.84244,
            },
        ),
    ),
];

fn to_inverse_problem(x: (DMS, DMS), y: (DMS, DMS)) -> InverseProblem {
    InverseProblem {
        x: x.into(),
        y: y.into(),
    }
}

pub fn inverse_problems() -> Vec<InverseProblem> {
    INVERSE_PROBLEM_DATA
        .iter()
        .map(move |(x, y)| to_inverse_problem(*x, *y))
        .collect()
}

pub fn inverse_solutions() -> Vec<InverseSolution> {
    DISTANCES
        .iter()
        .zip(X_AZIMUTHS.iter().zip(Y_AZIMUTHS.iter()))
        .map(|(&distance, (&x_azimuth, &y_azimuth))| InverseSolution {
            s: Dist { dist: distance },
            az1: Az {
                az: deg_to_rad(x_azimuth.to_deg()).0,
            },
            az2: Some(Az {
                az: deg_to_rad(y_azimuth.to_deg()).0,
            }),
        })
        .collect()
}

pub fn direct_pairs() -> Vec<(InverseProblem, InverseSolution)> {
    inverse_problems()
        .into_iter()
        .zip(inverse_solutions().into_iter())
        .collect()
}

pub fn direct_problems() -> Vec<InverseProblem> {
    direct_pairs().into_iter().map(|(p, _)| p).collect()
}

pub fn direct_solutions() -> Vec<InverseSolution> {
    direct_pairs().into_iter().map(|(_, s)| s).collect()
}

// Units of mm.
pub const TOLERANCE: f64 = 0.008;

// From the paper, Vincenty's errors were mm of -0.4, -0.4, -0.7, -0.2 and -0.8.
pub const INDIRECT_DISTANCE_TOLERANCES: [f64; 5] =
    [0.000404, 0.000387, 0.000703, 0.000197, 0.000787];

pub const AZ_TOLERANCE: DMS = DMS {
    deg: 0,
    min: 0,
    sec: 0.016667,
};

// Units of kilometers for distance and tolerance.
pub type Distance = f64;
pub type TestTolerance = f64;

pub type DiffDMS = fn(DMS, DMS) -> DMS;
pub type AzTolerance = DMS;
pub type SpanLatLng = fn(LatLng, LatLng) -> Distance;
pub type AzimuthFwd = fn(LatLng, LatLng) -> Option<Rad>;
pub type AzimuthBwd = fn(LatLng, LatLng) -> Option<Rad>;

pub fn describe_inverse_distance(
    x: DMS,
    y: DMS,
    s_expected: Distance,
    tolerance: TestTolerance,
) -> String {
    format!("{} to {} = {} ± {}", x, y, s_expected, tolerance)
}

pub fn describe_azimuth_fwd(
    x: DMS,
    y: DMS,
    az_actual: Option<DMS>,
    az_expected: DMS,
    tolerance: AzTolerance,
) -> String {
    format!(
        "{} to {} -> {} ± {} ({:?})",
        x,
        y,
        az_expected,
        tolerance,
        az_actual.map(|x| x.normalize())
    )
}

pub fn describe_azimuth_rev(
    x: DMS,
    y: DMS,
    az_actual: Option<DMS>,
    az_expected: DMS,
    tolerance: AzTolerance,
) -> String {
    format!(
        "{} to {} <- {} ± {} ({:?})",
        x,
        y,
        az_expected,
        tolerance,
        az_actual.map(|x| x.normalize())
    )
}
