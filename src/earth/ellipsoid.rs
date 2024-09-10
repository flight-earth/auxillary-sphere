use crate::units::*;

#[derive(Copy, Clone)]
pub struct Ellipsoid {
    pub equatorial_r: Radius,
    pub recip_f: f64,
}

impl ToString for Ellipsoid {
    fn to_string(&self) -> String {
        let r = self.equatorial_r.0;
        format!("R={}, 1/ƒ={}", r, self.recip_f)
    }
}

pub fn flattening(e: &Ellipsoid) -> f64 {
    1.0 / e.recip_f
}

pub fn polar_r(e: &Ellipsoid) -> Radius {
    e.equatorial_r * (1.0 - flattening(e))
}

/// SEE: <https://en.wikipedia.org/wiki/World_Geodetic_System>
/// <https://en.wikipedia.org/wiki/World_Geodetic_System#A_new_World_Geodetic_System:_WGS_84>
pub static WGS84: Ellipsoid = Ellipsoid {
    equatorial_r: Radius(Meter(6378137.0)),
    recip_f: 298.257223563,
};

/// As used by the National Geodetic Survey tool inverse when selecting the
/// ellipsoid 1) GRS80 / WGS84 (NAD83) SEE:
/// <https://www.ngs.noaa.gov/PC_PROD/Inv_Fwd/>
pub static NAD83: Ellipsoid = Ellipsoid {
    recip_f: 298.25722210088,
    ..WGS84
};

/// The Bessel ellipsoid from Vincenty 1975. Note that the flattening from
/// Wikipedia for the Bessel ellipsoid is 299.1528153513233 not 299.1528128. SEE:
/// <https://en.wikipedia.org/wiki/Bessel_ellipsoid>
pub static BESSEL: Ellipsoid = Ellipsoid {
    equatorial_r: Radius(Meter(6377397.155)),
    recip_f: 299.1528128,
};

/// The International ellipsoid 1924 also known as the Hayford ellipsoid from
/// Vincenty 1975. SEE: <https://en.wikipedia.org/wiki/Hayford_ellipsoid>
pub static HAYFORD: Ellipsoid = Ellipsoid {
    equatorial_r: Radius(Meter(6378388.0)),
    recip_f: 297.0,
};

/// Clarke's 1866 ellipsoid approximated in metres. "Clarke actually defined his
/// 1866 spheroid as a = 20,926,062 British feet, b = 20,855,121 British feet"
/// SEE: <https://en.wikipedia.org/wiki/North_American_Datum>
pub static CLARKE: Ellipsoid = Ellipsoid {
    equatorial_r: Radius(Meter(6378206.4)),
    recip_f: 294.978698214,
};

/// The ellipsoid used in Evaluation Direct and Inverse Geodetic Algorithms, by
/// Paul Delorme, Bedford Institute of Oceanography, Dartmouth, Nova Scotia,
/// Canada, 1978.
pub static BEDFORD_CLARKE: Ellipsoid = Ellipsoid {
    recip_f: 294.9786986,
    ..CLARKE
};
