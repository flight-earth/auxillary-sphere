pub mod units {
  use std::fmt;

  pub struct Deg(f64);

  #[derive(Debug, PartialEq)]
  pub struct DMS {
    pub deg : i32,
    pub min : i32,
    pub sec : f64,
  }

  impl DMS {
    pub fn from_deg(Deg(deg): Deg) -> DMS {
      let d_abs = deg.abs();
      let dd = d_abs.floor() as i32;
      let d_frac = d_abs - (dd as f64);
      let m_frac = d_frac * 60.0;
      let min = m_frac.floor() as i32;
      let secs = (m_frac - min as f64) * 60.0;
      DMS { deg: {if deg < 0.0 {-dd} else {dd}}, min: min, sec: secs }
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
