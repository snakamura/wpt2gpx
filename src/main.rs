use std::io;
use std::io::{Read, Write};

mod gpx;
mod waypoint;
mod wpt;

fn main() -> io::Result<()> {
    let mut wpt: String = String::new();
    io::stdin().read_to_string(&mut wpt)?;

    let waypoints = wpt::parser::parse(wpt.as_str()).unwrap();

    let gpx = gpx::formatter::format(&waypoints).unwrap();
    io::stdout().write(&gpx)?;

    Ok(())
}
