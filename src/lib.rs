pub mod units {
  use std::fmt;

  #[derive(Debug, PartialEq)]
  pub struct Deg(pub f64);

  #[derive(Debug, PartialEq)]
  pub struct DMS {
    pub deg : i32,
    pub min : i32,
    pub sec : f64,
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
      DMS { deg: {if deg < 0.0 {-dd} else {dd}}, min: mm, sec: m_frac * 60.0 }
    }

    pub fn to_deg(&self) -> Deg {
      let sign = if self.deg < 0 {-1.0} else {1.0};
      Deg(sign * (self.deg.abs() as f64 + (self.min as f64 / 60.0) + (self.sec / 3600.0)))
    }
  }

  impl fmt::Display for Deg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{}°", self.0)
    }
  }

  impl fmt::Display for DMS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{}°{}'{}\"", self.deg, self.min, self.sec)
    }
  }

  #[cfg(test)]
  mod tests {
      use super::*;

      #[test]
      fn show() {
        let dms = DMS { deg: 90, min: 12, sec: 0.999 };
        assert_eq!(format!("{}", dms), "90°12'0.999\"");
      }

      #[test]
      fn from_deg() {
        let dms_zero = DMS{ deg: 0, min: 0, sec: 0.0 };
        let deg_zero = Deg(0.0);
        assert_eq!(dms_zero, DMS::from_deg(deg_zero));

        let dms_one = DMS{ deg: 1, min: 0, sec: 0.0 };
        let deg_one= Deg(1.0);
        assert_eq!(dms_one, DMS::from_deg(deg_one));

        let dms_minus_one = DMS{ deg: -1, min: 0, sec: 0.0 };
        let deg_minus_one= Deg(-1.0);
        assert_eq!(dms_minus_one, DMS::from_deg(deg_minus_one));

        let dms_169 = DMS{ deg: 169, min: 3, sec: 59.99999839625161 };
        let deg_169= Deg(169.06666666622118);
        assert_eq!(dms_169, DMS::from_deg(deg_169));

        let dms_minus_169 = DMS{ deg: -169, min: 3, sec: 59.99999839625161 };
        let deg_minus_169= Deg(-169.06666666622118);
        assert_eq!(dms_minus_169, DMS::from_deg(deg_minus_169));
      }

      #[test]
      fn to_deg() {
        let dms_zero = DMS{ deg: 0, min: 0, sec: 0.0 };
        let deg_zero = Deg(0.0);
        assert_eq!(deg_zero, dms_zero.to_deg());

        let dms_one = DMS{ deg: 1, min: 0, sec: 0.0 };
        let deg_one= Deg(1.0);
        assert_eq!(deg_one, dms_one.to_deg());

        let dms_minus_one = DMS{ deg: -1, min: 0, sec: 0.0 };
        let deg_minus_one= Deg(-1.0);
        assert_eq!(deg_minus_one, dms_minus_one.to_deg());

        let dms_169 = DMS{ deg: 169, min: 3, sec: 59.99999839625161 };
        let deg_169= Deg(169.06666666622118);
        assert_eq!(deg_169, dms_169.to_deg());

        let dms_minus_169 = DMS{ deg: -169, min: 3, sec: 59.99999839625161 };
        let deg_minus_169= Deg(-169.06666666622118);
        assert_eq!(deg_minus_169, dms_minus_169.to_deg());
      }
  }
}

pub mod geodesy {
  use std::fmt;

  pub struct LatLng {
    pub lat : f64,
    pub lng : f64,
  }

  impl fmt::Display for LatLng{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "({}, {})", self.lat, self.lng)
    }
  }

  struct Az {
    az : f64,
  }

  struct Dist {
    dist : f64,
  }

  struct DirectProblem {
    x : LatLng,
    az1 : Az,
    s : Dist,
  }

  struct InverseProblem {
    x : LatLng,
    y : LatLng,
  }

  struct DirectSolution {
    y : LatLng,
    az2 : Az,
  }

  struct InverseSolution {
    s : Dist,
    az1 : Az,
    az2 : Az,
  }
}
