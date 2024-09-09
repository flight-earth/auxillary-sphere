pub mod units {
  use std::fmt;

  pub struct DMS {
    pub deg : i32,
    pub min : i32,
    pub sec : f64,
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
    fn check_show() {
      let dms = DMS { deg: 90, min: 12, sec: 0.999 };
      assert_eq!(format!("{}", dms), "90°12'0.999\"");
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
