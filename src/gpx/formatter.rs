use super::super::waypoint::Waypoint;
use quick_xml::events::BytesText;
use quick_xml::writer::Writer;
use quick_xml::Error;
use std::io;

pub fn format<TString>(waypoints: &[Waypoint<TString>]) -> Result<Vec<u8>, Error>
where
    TString: AsRef<str>,
{
    let mut writer = Writer::new_with_indent(io::Cursor::new(Vec::<u8>::new()), b' ', 2);
    writer
        .create_element("gpx")
        .with_attributes([
            ("xmlns", "http://www.topografix.com/GPX/1/0"),
            ("version", "1.0"),
        ])
        .write_inner_content::<_, Error>(|writer| format_waypoints(waypoints, writer))?;
    Ok(writer.into_inner().into_inner())
}

fn format_waypoints<TString, W>(
    waypoints: &[Waypoint<TString>],
    writer: &mut Writer<W>,
) -> Result<(), Error>
where
    TString: AsRef<str>,
    W: io::Write,
{
    for waypoint in waypoints {
        format_waypoint(waypoint, writer)?;
    }
    Ok(())
}

fn format_waypoint<TString, W>(
    waypoint: &Waypoint<TString>,
    writer: &mut Writer<W>,
) -> Result<(), Error>
where
    TString: AsRef<str>,
    W: io::Write,
{
    writer
        .create_element("wpt")
        .with_attributes([
            ("lat", format!("{:.6}", waypoint.latitude).as_str()),
            ("lon", format!("{:.6}", waypoint.longitude).as_str()),
        ])
        .write_inner_content::<_, Error>(|writer| {
            writer
                .create_element("ele")
                .write_text_content(BytesText::new(waypoint.altitude.to_string().as_str()))?;
            writer
                .create_element("name")
                .write_text_content(BytesText::new(&waypoint.display_name()))?;
            Ok(())
        })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format() -> Result<(), Error> {
        let waypoints = vec![
            Waypoint {
                name: "AAT054",
                latitude: 36.276783,
                longitude: 140.145417,
                altitude: 540,
                description: "ASIO HG TO",
            },
            Waypoint {
                name: "ACT052",
                latitude: 36.269475,
                longitude: 140.141306,
                altitude: 524,
                description: "COO TO",
            },
        ];

        let bytes = format(&waypoints)?;
        let s = String::from_utf8(bytes)?;
        assert_eq!(
            s,
            r#"<gpx xmlns="http://www.topografix.com/GPX/1/0" version="1.0">
  <wpt lat="36.276783" lon="140.145417">
    <ele>540</ele>
    <name>AAT054 ASIO HG TO</name>
  </wpt>
  <wpt lat="36.269475" lon="140.141306">
    <ele>524</ele>
    <name>ACT052 COO TO</name>
  </wpt>
</gpx>"#,
        );

        Ok(())
    }

    #[test]
    fn test_format_waypoints() -> Result<(), Error> {
        let waypoints = vec![
            Waypoint {
                name: "AAT054",
                latitude: 36.276783,
                longitude: 140.145417,
                altitude: 540,
                description: "ASIO HG TO",
            },
            Waypoint {
                name: "ACT052",
                latitude: 36.269475,
                longitude: 140.141306,
                altitude: 524,
                description: "COO TO",
            },
        ];

        let mut writer = Writer::new(io::Cursor::new(Vec::<u8>::new()));
        format_waypoints(&waypoints, &mut writer)?;

        let s = String::from_utf8(writer.into_inner().into_inner())?;
        assert_eq!(
            s,
            concat!(
                r#"<wpt lat="36.276783" lon="140.145417"><ele>540</ele><name>AAT054 ASIO HG TO</name></wpt>"#,
                r#"<wpt lat="36.269475" lon="140.141306"><ele>524</ele><name>ACT052 COO TO</name></wpt>"#
            )
        );

        Ok(())
    }

    #[test]
    fn test_format_waypoint() -> Result<(), Error> {
        let waypoint = Waypoint {
            name: "AAT054",
            latitude: 36.276783,
            longitude: 140.145417,
            altitude: 540,
            description: "ASIO HG TO",
        };

        let mut writer = Writer::new(io::Cursor::new(Vec::<u8>::new()));
        format_waypoint(&waypoint, &mut writer)?;

        let s = String::from_utf8(writer.into_inner().into_inner())?;
        assert_eq!(
            s,
            r#"<wpt lat="36.276783" lon="140.145417"><ele>540</ele><name>AAT054 ASIO HG TO</name></wpt>"#
        );

        Ok(())
    }

    #[test]
    fn test_format_waypoint_fractions() -> Result<(), Error> {
        let waypoint = Waypoint {
            name: "AAT054",
            latitude: 36.276,
            longitude: 140.14541789,
            altitude: 1540,
            description: "ASIO HG TO",
        };

        let mut writer = Writer::new(io::Cursor::new(Vec::<u8>::new()));
        format_waypoint(&waypoint, &mut writer)?;

        let s = String::from_utf8(writer.into_inner().into_inner())?;
        assert_eq!(
            s,
            r#"<wpt lat="36.276000" lon="140.145418"><ele>1540</ele><name>AAT054 ASIO HG TO</name></wpt>"#
        );

        Ok(())
    }
}
