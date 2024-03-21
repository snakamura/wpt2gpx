use crate::waypoint::Waypoint;
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
        .write_inner_content::<_, Error>(|writer| {
            for waypoint in waypoints.iter() {
                writer
                    .create_element("wpt")
                    .with_attributes([
                        ("lat", format!("{:.6}", waypoint.latitude).as_str()),
                        ("lon", format!("{:.6}", waypoint.longitude).as_str()),
                    ])
                    .write_inner_content::<_, Error>(|writer| {
                        writer
                            .create_element("ele")
                            .write_text_content(BytesText::new(
                                waypoint.altitude.to_string().as_str(),
                            ))?;
                        writer
                            .create_element("name")
                            .write_text_content(BytesText::new(&waypoint.display_name()))?;
                        Ok(())
                    })?;
            }
            Ok(())
        })?;
    Ok(writer.into_inner().into_inner())
}
