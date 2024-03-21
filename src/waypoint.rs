#[derive(Debug)]
pub struct Waypoint<TString> {
    pub name: TString,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: u16,
    pub description: TString,
}

impl<TString> Waypoint<TString> {
    pub fn display_name(&self) -> String
    where
        TString: AsRef<str>,
    {
        let description = self.description.as_ref();
        format!(
            "{}{}{}",
            self.name.as_ref(),
            if description.is_empty() { "" } else { " " },
            description
        )
    }
}
