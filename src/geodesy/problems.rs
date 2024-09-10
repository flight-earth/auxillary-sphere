use crate::geodesy::latlng::LatLng;
use std::fmt;

pub struct Az {
    pub az: f64,
}

pub struct Dist {
    pub dist: f64,
}

impl fmt::Display for Dist {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.dist)
    }
}

pub struct DirectProblem {
    pub x: LatLng,
    pub az1: Az,
    pub s: Dist,
}

pub struct InverseProblem {
    pub x: LatLng,
    pub y: LatLng,
}

pub struct DirectSolution {
    pub y: LatLng,
    pub az2: Option<Az>,
}

pub struct InverseSolution {
    pub s: Dist,
    pub az1: Az,
    pub az2: Option<Az>,
}
