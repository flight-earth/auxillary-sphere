use crate::geodesy::latlng::LatLng;

struct Az {
    az: f64,
}

struct Dist {
    dist: f64,
}

struct DirectProblem {
    x: LatLng,
    az1: Az,
    s: Dist,
}

struct InverseProblem {
    x: LatLng,
    y: LatLng,
}

struct DirectSolution {
    y: LatLng,
    az2: Option<Az>,
}

struct InverseSolution {
    s: Dist,
    az1: Az,
    az2: Option<Az>,
}
