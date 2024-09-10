use std::fmt;

pub struct LatLng {
    pub lat: f64,
    pub lng: f64,
}

impl fmt::Display for LatLng {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.lat, self.lng)
    }
}

struct Az {
    az: f64,
}

struct Dist {
    dist: f64,
}

struct DirectProblem {
    x: LatLng,
    az1: Az,
    s: Dist,
}

struct InverseProblem {
    x: LatLng,
    y: LatLng,
}

struct DirectSolution {
    y: LatLng,
    az2: Az,
}

struct InverseSolution {
    s: Dist,
    az1: Az,
    az2: Az,
}
