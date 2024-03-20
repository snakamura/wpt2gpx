#[derive(Debug)]
pub struct Waypoint {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: u16,
    pub description: String,
}
