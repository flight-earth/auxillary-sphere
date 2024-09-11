pub mod earth {
    pub mod ellipsoid;
    pub mod sphere {
        use super::super::units::{Meter, Radius};

        pub static EARTH_RADIUS: Radius = Radius(Meter(6371000.0));
    }
}
pub mod geodesy {
    pub mod haversines;
    pub mod vincenty;
    pub mod latlng;
    pub mod problems;
    pub mod published {
        pub mod vincenty1975;
    }
    pub mod point_to_point {
        pub mod vincenty;
    }
}
pub mod units;
