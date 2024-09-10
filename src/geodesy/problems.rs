use crate::{
    geodesy::latlng::LatLng,
    units::{convert::rad_to_deg, Rad},
};
use std::fmt;

#[derive(Debug)]
pub struct Az {
    pub az: f64,
}

#[derive(Debug, Clone)]
pub struct Dist {
    pub dist: f64,
}

#[derive(Debug)]
pub struct DirectProblem {
    pub x: LatLng,
    pub az1: Az,
    pub s: Dist,
}

#[derive(Debug)]
pub struct InverseProblem {
    pub x: LatLng,
    pub y: LatLng,
}

#[derive(Debug)]
pub struct DirectSolution {
    pub y: LatLng,
    pub az2: Option<Az>,
}

#[derive(Debug)]
pub struct InverseSolution {
    pub s: Dist,
    pub az1: Az,
    pub az2: Option<Az>,
}

impl fmt::Display for Az {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let deg = rad_to_deg(Rad(self.az));
        write!(f, "{:.2}", deg)
    }
}

impl fmt::Display for Dist {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.3}", self.dist)
    }
}

impl fmt::Display for DirectProblem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(x={}, az1={}, s={})", self.x, self.az1, self.s)
    }
}

impl fmt::Display for InverseProblem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(x={}, y={})", self.x, self.y)
    }
}

impl fmt::Display for DirectSolution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.az2.as_ref() {
            Some(az2) => write!(f, "(y={}, az2={})", self.y, az2),
            None => write!(f, "(y={})", self.y),
        }
    }
}

impl fmt::Display for InverseSolution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.az2.as_ref() {
            Some(az2) => write!(f, "(s={}, az1={}, az2={})", self.s, self.az1, az2),
            None => write!(f, "(s={}, az1={})", self.s, self.az1),
        }
    }
}
