use auxillary_sphere::geodesy::LatLng;

fn main() {
  let london = LatLng{lat: 51.5007, lng: -0.1246};
  let newyork = LatLng{lat: 40.6892, lng: -74.0445};
  println!("London: {london}");
  println!("New York: {newyork}");
}
