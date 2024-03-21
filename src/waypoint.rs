#[derive(Debug)]
pub struct Waypoint<TString> {
    pub name: TString,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: u16,
    pub description: TString,
}
