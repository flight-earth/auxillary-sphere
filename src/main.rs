use auxillary_sphere::geodesy::published::vincenty1975::vincenty_units;
use auxillary_sphere::units::DMS;
use auxillary_sphere::geodesy::latlng::LatLng;
use auxillary_sphere::geodesy::problems::InverseProblem;
use auxillary_sphere::geodesy::haversines::{distance, inverse};

fn main() {
  let london = LatLng::new(51.5007, -0.1246);
  let newyork = LatLng::new(40.6892, -74.0445);
  println!("London: {london}");
  println!("New York: {newyork}");

  let distance_lon_nyc = distance(&london, &newyork);
  let inverse_prob = InverseProblem{x: london, y: newyork};
  let inverse_lon_nyc = inverse(&inverse_prob);

  println!("Distance LON -> NYC: {distance_lon_nyc}");
  println!("Inverse Solution LON NYC {inverse_lon_nyc}");

  let dms = DMS{deg: 90, min: 12, sec: 0.999};
  println!("DMS: {dms}");

  println!("Vincenty 1975 Published Data Checks");
  let _ = vincenty_units();
}
