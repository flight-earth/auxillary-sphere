use std::fmt::Display;

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
        point_to_point,
        problems::{Az, Dist, InverseProblem, InverseSolution},
    },
    units::{
        convert::{deg_to_rad, rad_to_deg},
        Angle, Rad, DMS,
    },
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
pub static TOLERANCE: TestTolerance = Dist { dist: 0.008 };

// From the paper, Vincenty's errors were mm of -0.4, -0.4, -0.7, -0.2 and -0.8.
pub const INDIRECT_DISTANCE_TOLERANCES: [f64; 5] =
    [0.000404, 0.000387, 0.000703, 0.000197, 0.000787];

pub const AZ_TOLERANCE: DMS = DMS {
    deg: 0,
    min: 0,
    sec: 0.016667,
};

// Units of kilometers for distance and tolerance.
pub type TestTolerance = Dist;

pub type DiffDMS = fn(DMS, DMS) -> DMS;
pub type AzTolerance = DMS;
pub type SpanLatLng = fn(LatLng, LatLng) -> Dist;
pub type AzimuthFwd = fn(LatLng, LatLng) -> Option<Rad>;
pub type AzimuthBwd = fn(LatLng, LatLng) -> Option<Rad>;

pub fn describe_inverse_distance(
    x: LatLng,
    y: LatLng,
    s_expected: Dist,
    tolerance: TestTolerance,
) -> String {
    format!("{} to {} = {} ± {}", x, y, s_expected, tolerance)
}

pub fn describe_azimuth_fwd(
    x: LatLng,
    y: LatLng,
    az_actual: Option<Az>,
    az_expected: Az,
    tolerance: AzTolerance,
) -> String {
    format!(
        "{} to {} -> {} ± {} ({:?})",
        x,
        y,
        az_expected,
        tolerance,
        az_actual.map(|x| DMS::from_deg(rad_to_deg(Rad(x.az))).normalize())
    )
}

pub fn describe_azimuth_rev(
    x: LatLng,
    y: LatLng,
    az_actual: Option<Az>,
    az_expected: Az,
    tolerance: AzTolerance,
) -> String {
    format!(
        "{} to {} <- {} ± {} ({:?})",
        x,
        y,
        az_expected,
        tolerance,
        az_actual.map(|x| DMS::from_deg(rad_to_deg(Rad(x.az))).normalize())
    )
}

pub type Assertion = Result<(), String>;

pub fn assert_failure(msg: &str) -> Assertion {
    Err(msg.to_string())
}

pub fn unless(b: bool, m: Assertion) -> Assertion {
    if b {
        Ok(())
    } else {
        m
    }
}

pub fn assert_compare<T: Display + PartialOrd>(
    preface: &str,
    compare: fn(&T, &T) -> bool,
    cmp_symbol: &str,
    key: &T,
    actual: &T,
) -> Assertion {
    unless(
        compare(actual, key),
        assert_failure(&format!(
            "{}expected: {} {} {}",
            if preface.is_empty() {
                "".to_string()
            } else {
                format!("{}\n", preface)
            },
            actual,
            cmp_symbol,
            key
        )),
    )
}

pub fn assert_compare_with<T: Display, U: PartialOrd>(
    convert: fn(&T) -> U,
    preface: &str,
    compare: fn(&U, &U) -> bool,
    cmp_symbol: &str,
    key: &T,
    actual: &T,
) -> Assertion {
    unless(
        compare(&convert(actual), &convert(key)),
        assert_failure(&format!(
            "{}expected: {} {} {}",
            if preface.is_empty() {
                "".to_string()
            } else {
                format!("{}\n", preface)
            },
            actual,
            cmp_symbol,
            key
        )),
    )
}

pub fn test_case(name: &str, test: Assertion) -> Assertion {
    println!("{}", name);
    test
}

pub fn diff(x: f64, y: f64) -> f64 {
    (x - y).abs()
}

pub fn az_to_dms(az: Az) -> DMS {
    rad_to_dms(Rad(az.az))
}

pub fn rad_to_dms(rad: Rad) -> DMS {
    DMS::from_deg(rad_to_deg(rad))
}

pub fn inverse_checks(
    diff_az_fwd: fn(DMS, DMS) -> DMS,
    diff_az_rev: fn(DMS, DMS) -> DMS,
    dist_tolerances: &[TestTolerance],
    az_tolerance: AzTolerance,
    spans: Vec<SpanLatLng>,
    az_fwds: Vec<AzimuthFwd>,
    az_revs: Vec<AzimuthBwd>,
    solns: &[InverseSolution],
    probs: &[InverseProblem],
) -> Vec<Assertion> {
    let f = |dist_tolerance: &TestTolerance,
             span: &SpanLatLng,
             az_fwd: &AzimuthFwd,
             az_rev: &AzimuthBwd,
             soln: &InverseSolution,
             prob: &InverseProblem| {
        let InverseSolution { s, az1, az2 } = soln;
        let InverseProblem { x, y } = prob;
        let s_prime = span(*x, *y);
        let az1_prime = az_fwd(*x, *y);
        let az2_prime = az_rev(*x, *y);

        vec![
            {
                let actual = diff(s_prime.dist, s.dist);
                test_case(
                    &describe_inverse_distance(*x, *y, *s, *dist_tolerance),
                    assert_compare("", |a, b| a <= b, "<=", &dist_tolerance.dist, &actual),
                )
            },
            {
                if let Some(Rad(az1_prime)) = az1_prime {
                    let actual = diff_az_fwd(az_to_dms(*az1), rad_to_dms(Rad(az1_prime)));
                    let expected = az_tolerance;
                    test_case(
                        &describe_azimuth_fwd(
                            *x,
                            *y,
                            Some(Az { az: az1_prime }),
                            *az1,
                            az_tolerance,
                        ),
                        assert_compare_with(
                            |dms| dms.to_deg().0,
                            "",
                            |a, b| a <= b,
                            "<=",
                            &expected,
                            &actual,
                        ),
                    )
                } else {
                    Ok(())
                }
            },
            {
                if let Some(Rad(az2_prime)) = az2_prime {
                    if let Some(az2) = az2 {
                        let actual = diff_az_rev(rad_to_dms(Rad(az2_prime)), az_to_dms(*az2));
                        let expected = az_tolerance;
                        test_case(
                            &describe_azimuth_rev(
                                *x,
                                *y,
                                Some(Az { az: az2_prime }),
                                *az2,
                                az_tolerance,
                            ),
                            assert_compare_with(
                                |dms| dms.to_deg().0,
                                "",
                                |a, b| a <= b,
                                "<=",
                                &expected,
                                &actual,
                            ),
                        )
                    } else {
                        Ok(())
                    }
                } else {
                    Ok(())
                }
            },
        ]
    };

    dist_tolerances
        .iter()
        .zip(spans.iter())
        .zip(az_fwds.iter().zip(az_revs.iter()))
        .zip(solns.iter().zip(probs.iter()))
        .flat_map(
            |(((dist_tolerance, span), (az_fwd, az_rev)), (soln, prob))| {
                f(dist_tolerance, span, az_fwd, az_rev, soln, prob)
            },
        )
        .collect()
}

pub fn vincenty_units() -> Result<(), String> {
    let diff_az_fwd: DiffDMS = |x, y| DMS::abs_diff_dms(x, y);
    let diff_az_rev: DiffDMS = |x, y| DMS::abs_diff_dms_180(y)(x);

    fn span_lat_lng(x: LatLng, y: LatLng) -> Dist {
        let vincenty_distance = point_to_point::vincenty::distance;
        match vincenty_distance(BESSEL, x, y) {
            Ok(d) => d,
            Err(_) => Dist { dist: 0.0 },
        }
    }

    let dist_tolerances: &[Dist] = &DISTANCES
        .iter()
        .map(|&d| Dist { dist: d })
        .collect::<Vec<_>>();

    // TODO: Add direct checks to checks
    let checks = inverse_checks(
        diff_az_fwd,
        diff_az_rev,
        dist_tolerances,
        AZ_TOLERANCE,
        vec![span_lat_lng; 5],
        vec![|_, _| None; 5],
        vec![|_, _| None; 5],
        &inverse_solutions(),
        &inverse_problems(),
    );

    for check in checks {
        check?;
    }

    Ok(())
}
