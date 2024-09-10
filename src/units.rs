use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Rad(pub f64);

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Deg(pub f64);

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Min(pub f64);

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Sec(pub f64);

pub trait Normalize {
    fn normalize(&self) -> Self;
}

pub mod convert {
    use super::*;

    pub fn min_to_sec(Min(min): Min) -> Sec {
        Sec(min * 60.0)
    }

    pub fn deg_to_sec(Deg(deg): Deg) -> Sec {
        Sec(deg * 3600.0)
    }

    pub fn deg_to_min(Deg(deg): Deg) -> Min {
        Min(deg * 60.0)
    }

    pub fn min_to_deg(Min(min): Min) -> Deg {
        Deg(min / 60.0)
    }

    pub fn sec_to_deg(Sec(sec): Sec) -> Deg {
        Deg(sec / 3600.0)
    }

    pub fn deg_to_rad(Deg(deg): Deg) -> Rad {
        Rad(deg.to_radians())
    }

    pub fn rad_to_deg(Rad(rad): Rad) -> Deg {
        Deg(rad.to_degrees())
    }
}

#[derive(Debug, PartialEq)]
pub struct DMS {
    pub deg: i32,
    pub min: i32,
    pub sec: f64,
}

// SEE: https://www.wezm.net/v2/posts/2023/divmod/
fn div_mod(n: f64, d: i32) -> (i32, f64) {
    let q = (n / d as f64).floor();
    (q as i32, (n - (q * d as f64)))
}

impl DMS {
    /// Convert from degrees to DMS
    ///
    /// ```
    /// # use auxillary_sphere::units::*;
    /// assert_eq!(format!("{}",
    ///   DMS::from_deg(Deg(-169.06666666622118))),
    ///   "-169°3'59.99999839625161\"");
    /// ```
    pub fn from_deg(Deg(deg): Deg) -> DMS {
        let d_abs = deg.abs();
        let dd = d_abs.floor() as i32;
        let d_frac = d_abs - (dd as f64);
        let (mm, m_frac) = div_mod(d_frac * 60.0, 1);
        match (deg < 0.0, (dd, mm, m_frac * 60.0)) {
            (false, (_, _, ss)) => DMS {
                deg: dd,
                min: mm,
                sec: ss,
            },
            (true, (0, 0, ss)) => DMS {
                deg: 0,
                min: 0,
                sec: -ss,
            },
            (true, (0, _, ss)) => DMS {
                deg: 0,
                min: -mm,
                sec: ss,
            },
            (true, (_, _, ss)) => DMS {
                deg: -dd,
                min: mm,
                sec: ss,
            },
        }
    }

    pub fn to_deg(&self) -> Deg {
        let sign = if self.deg < 0 { -1.0 } else { 1.0 };
        Deg(sign * (self.deg.abs() as f64 + (self.min as f64 / 60.0) + (self.sec / 3600.0)))
    }
}

/// Normalize DMS to the degree equivalent of `0 <= deg < 360`.
///
/// ```
/// # use auxillary_sphere::units::*;
/// assert_eq!(format!("{:.0}", DMS { deg: 0, min: -1, sec: 0.0 }.normalize()), "359°59'0\"");
/// assert_eq!(format!("{:.0}", DMS { deg: 0, min: 0, sec: 61.0 }.normalize()), "0°1'1\"");
/// assert_eq!(format!("{:.0}", DMS { deg: 0, min: 61, sec: 0.0 }.normalize()), "1°0'60\"");
/// ```
///
/// Display doesn't round up `0'59.9"` to `1'0"`.
/// ```
/// # use auxillary_sphere::units::*;
/// assert_eq!(format!("{:.0}", DMS { deg: 1, min: 0, sec: 59.9 }.normalize()), "1°0'60\"");
/// assert_eq!(format!("{:.0}", DMS { deg: 1, min: 0, sec: 60.0 }.normalize()), "1°0'60\"");
/// ```
///
/// Positive normalized.
/// ```
/// # use auxillary_sphere::units::*;
/// assert_eq!(format!("{:.0}, {0:?}", DMS::from_deg(Deg(1.0/60.0)).normalize()),
///  "0°1'0\", DMS { deg: 0, min: 1, sec: 0.0 }");
/// assert_eq!(format!("{:.0}, {0:?}", DMS::from_deg(Deg(1.0/3600.0)).normalize()),
///  "0°0'1\", DMS { deg: 0, min: 0, sec: 1.0 }");
/// ```
///
/// Negative not normalized.
/// ```
/// # use auxillary_sphere::units::*;
/// assert_eq!(format!("{:.0}, {0:?}", DMS::from_deg(Deg(-1.0/60.0))),
///  "-0°1'0\", DMS { deg: 0, min: -1, sec: 0.0 }");
/// assert_eq!(format!("{:.0}, {0:?}", DMS::from_deg(Deg(-1.0/3600.0))),
///  "-0°0'1\", DMS { deg: 0, min: 0, sec: -1.0 }");
/// ```
///
/// Negative normalized.
/// ```
/// # use auxillary_sphere::units::*;
/// assert_eq!(format!("{:.0}", DMS::from_deg(Deg(-1.0/60.0).normalize())), "359°59'0\"");
/// assert_eq!(format!("{:.0}", DMS::from_deg(Deg(-1.0/60.0)).normalize()), "359°59'0\"");
/// assert_eq!(format!("{:.0}", DMS::from_deg(Deg(-1.0/3600.0).normalize())), "359°59'59\"");
/// assert_eq!(format!("{:.0}", DMS::from_deg(Deg(-1.0/3600.0)).normalize()), "359°59'59\"");
/// ```
impl Normalize for DMS {
    fn normalize(&self) -> DMS {
        DMS::from_deg(DMS::to_deg(self).normalize())
    }
}

/// Normalize degree so that `0 <= deg < 360`.
///
/// ```
/// # use auxillary_sphere::units::*;
/// assert_eq!(format!("{:.0}", Deg(0.0).normalize()), "0°");
/// assert_eq!(format!("{:.0}", Deg(1.0).normalize()), "1°");
/// assert_eq!(format!("{:.0}", Deg(-1.0).normalize()), "359°");
/// assert_eq!(format!("{:.0}", Deg(359.0).normalize()), "359°");
/// assert_eq!(format!("{:.0}", Deg(361.0).normalize()), "1°");
/// assert_eq!(format!("{:.0}, {0:?}", DMS::from_deg(Deg(1.0)).normalize()),
///  "1°0'0\", DMS { deg: 1, min: 0, sec: 0.0 }");
/// ```
///
/// ```
/// # use auxillary_sphere::units::*;
/// assert_eq!(format!("{:.4}", Deg(1.0/60.0).normalize()), "0.0167°");
/// assert_eq!(format!("{:.4}", Deg(-1.0/60.0).normalize()), "359.9833°");
/// ```
impl Normalize for Deg {
    fn normalize(&self) -> Deg {
        let d = self.0;
        let x = d % 360.0;
        if x == 0.0 {
            Deg(0.0)
        } else if x < 0.0 {
            Deg(360.0 + x)
        } else {
            Deg(x)
        }
    }
}

impl fmt::Display for Deg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(precision) = f.precision() {
            write!(f, "{1:.*}°", precision, self.0)
        } else {
            write!(f, "{}°", self.0)
        }
    }
}

impl fmt::Display for DMS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sign = if self.deg < 0 || self.min < 0 || self.sec < 0.0 {
            "-"
        } else {
            ""
        };
        let DMS {
            deg: d,
            min: m,
            sec: s,
        } = self;
        if self.sec == 0.0 {
            write!(f, "{}{}°{}'0\"", sign, d.abs(), m.abs())
        } else if let Some(precision) = f.precision() {
            write!(
                f,
                "{}{}°{}'{4:.*}\"",
                sign,
                d.abs(),
                m.abs(),
                precision,
                s.abs()
            )
        } else {
            write!(f, "{}{}°{}'{}\"", sign, d.abs(), m.abs(), s.abs())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn show() {
        let dms = DMS {
            deg: 90,
            min: 12,
            sec: 0.999,
        };
        assert_eq!(format!("{}", dms), "90°12'0.999\"");
    }

    #[test]
    fn from_deg_to_deg() {
        let dms_zero = DMS {
            deg: 0,
            min: 0,
            sec: 0.0,
        };
        let deg_zero = Deg(0.0);
        assert_eq!(dms_zero, DMS::from_deg(deg_zero));
        assert_eq!(deg_zero, dms_zero.to_deg());

        let dms_one = DMS {
            deg: 1,
            min: 0,
            sec: 0.0,
        };
        let deg_one = Deg(1.0);
        assert_eq!(dms_one, DMS::from_deg(deg_one));
        assert_eq!(deg_one, dms_one.to_deg());

        let dms_minus_one = DMS {
            deg: -1,
            min: 0,
            sec: 0.0,
        };
        let deg_minus_one = Deg(-1.0);
        assert_eq!(dms_minus_one, DMS::from_deg(deg_minus_one));
        assert_eq!(deg_minus_one, dms_minus_one.to_deg());

        let dms_169 = DMS {
            deg: 169,
            min: 3,
            sec: 59.99999839625161,
        };
        let deg_169 = Deg(169.06666666622118);
        assert_eq!(dms_169, DMS::from_deg(deg_169));
        assert_eq!(deg_169, dms_169.to_deg());

        let dms_minus_169 = DMS {
            deg: -169,
            min: 3,
            sec: 59.99999839625161,
        };
        let deg_minus_169 = Deg(-169.06666666622118);
        assert_eq!(dms_minus_169, DMS::from_deg(deg_minus_169));
        assert_eq!(deg_minus_169, dms_minus_169.to_deg());
    }
}
