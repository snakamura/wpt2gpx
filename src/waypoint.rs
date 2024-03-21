#[derive(PartialEq, Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;

    const WAYPOINT: Waypoint<&str> = Waypoint {
        name: "AAT054",
        latitude: 36.276783,
        longitude: 140.145417,
        altitude: 540,
        description: "ASIO HG TO",
    };

    #[test]
    fn test_display_name() {
        assert_eq!(WAYPOINT.display_name(), "AAT054 ASIO HG TO");
    }

    #[test]
    fn test_display_name_no_description() {
        let waypoint = Waypoint {
            description: "",
            ..WAYPOINT
        };
        assert_eq!(waypoint.display_name(), "AAT054");
    }
}
