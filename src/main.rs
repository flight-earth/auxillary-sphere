use auxillary_sphere::units::DMS;
use auxillary_sphere::geodesy::latlng::LatLng;

fn main() {
  let london = LatLng::new(51.5007, -0.1246);
  let newyork = LatLng::new(40.6892, -74.0445);
  let dms = DMS{deg: 90, min: 12, sec: 0.999};
  println!("London: {london}");
  println!("New York: {newyork}");
  println!("DMS: {dms}");
}
