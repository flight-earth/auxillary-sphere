use auxillary_sphere::units::DMS;
use auxillary_sphere::geodesy::problems::LatLng;

fn main() {
  let london = LatLng{lat: 51.5007, lng: -0.1246};
  let newyork = LatLng{lat: 40.6892, lng: -74.0445};
  let dms = DMS{deg: 90, min: 12, sec: 0.999};
  println!("London: {london}");
  println!("New York: {newyork}");
  println!("DMS: {dms}");
}
