use auxillary_sphere::units::DMS;
use auxillary_sphere::geodesy::latlng::LatLng;
use auxillary_sphere::geodesy::haversines::distance;

fn main() {
  let london = LatLng::new(51.5007, -0.1246);
  let newyork = LatLng::new(40.6892, -74.0445);
  let distance_LON_NYC = distance(&london, &newyork);

  println!("London: {london}");
  println!("New York: {newyork}");
  println!("Distance LON -> NYC: {distance_LON_NYC}");

  let dms = DMS{deg: 90, min: 12, sec: 0.999};
  println!("DMS: {dms}");
}
