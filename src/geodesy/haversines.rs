use crate::earth::sphere::EARTH_RADIUS;
use crate::units::*;
use crate::units::convert::*;
use crate::geodesy::latlng::*;
use crate::geodesy::problems::*;
use std::f64::consts::PI;

fn haversine (Rad(x): Rad) -> Rad {
    let y = (x / 2.0).sin();
    Rad(y * y)
}

fn a_of_haversine (x: &LatLng, y : &LatLng) -> Rad {
  let d_lat = y.lat.0 - x.lat.0;
  let d_lng = y.lng.0 - x.lng.0;
  let Rad(h_lat_f) = haversine(Rad(d_lat));
  let Rad(h_lng_f) = haversine(Rad(d_lng));

  Rad(h_lat_f + x.lat.0.cos() * y.lat.0.cos() * h_lng_f)
}

pub fn distance (x : &LatLng, y : &LatLng) -> Dist {
  let r = EARTH_RADIUS.0.0;
  let d = 2.0 * (a_of_haversine(x, y).0.sqrt().asin());
  Dist{dist : d * r}
}

fn azimuth_fwd_aux (xll: &LatLng, yll: &LatLng) -> Rad {
  let delta_lng = yll.lng.0 - xll.lng.0;
  let x = (delta_lng * yll.lat.0.cos()).sin();
  let y = xll.lat.0.cos() * yll.lat.0.sin() - xll.lat.0.sin() * yll.lat.0.cos() * delta_lng.cos();
  Rad(x.atan2(y))
}

fn azimuth_fwd (x: &LatLng, y: &LatLng) -> Option<Rad> { Some(azimuth_fwd_aux(x, y)) }

impl Rad {
    fn rotate(&self, by: Rad) -> Rad { Rad(self.0 + by.0) }
}

fn azimuth_rev(x: &LatLng, y: &LatLng) -> Option<Rad> {
  azimuth_fwd(y, x).map(|az| az.rotate(Rad(PI)))
}

fn direct(prob: DirectProblem) -> DirectSolution{
    let LatLng{lat : Rad(lat1), lng : Rad(lng1)} = prob.x;
    let az1 = prob.az1.az;
    let Radius(Meter(earth_r)) = EARTH_RADIUS;
    let d = prob.s.dist;
    let d_r = d / earth_r;
  
    // SEE: https://www.movable-type.co.uk/scripts/latlong.html
    let lat2 = (lat1.sin() * d_r.cos() + lat1.cos() * d_r.sin() * az1.cos()).asin();
    let lng2 = lng1 + (az1.sin() * d_r.sin() * lat1.cos()).atan2(d_r.cos() - lat1.sin() * lat2.sin());
    let y : LatLng = LatLng{lat : Rad(lat2), lng : Rad(lng2)};
  
    let az2 =
        (azimuth_fwd(&y, &LatLng{lat : Rad(lat1), lng : Rad(lng1)}))
        .map (|az| -> Az {
            let deg: Deg = rad_to_deg(az.rotate (Rad(PI)));
            Az{az : deg_to_rad(deg.normalize()).0}
        });
  
    DirectSolution{y : LatLng{lat : Rad(lat2), lng : Rad(lng2)}, az2 : az2}
}

fn inverse(InverseProblem{x, y}: &InverseProblem) -> InverseSolution {
    let az1 = azimuth_fwd(x, y).map(|az| Az{az : az.0}).unwrap();
    let az2 = azimuth_rev(x, y).map(|az| Az{az : az.0});
    let s = distance(x, y);
    InverseSolution{s : s, az1 : az1, az2 : az2}
}
