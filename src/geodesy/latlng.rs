use crate::units::convert::*;
use crate::units::{Deg, Rad};
use std::fmt;

#[derive(Debug)]
pub struct LatLng {
    pub lat: Rad,
    pub lng: Rad,
}

impl LatLng {
    pub fn new(lat: f64, lng: f64) -> Self {
        LatLng {
            lat: deg_to_rad(Deg(lat)),
            lng: deg_to_rad(Deg(lng)),
        }
    }
}

impl fmt::Display for LatLng {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lat = rad_to_deg(self.lat);
        let lng = rad_to_deg(self.lng);
        write!(f, "({}, {})", lat, lng)
    }
}
