use std::io;
use std::io::Read;

mod gpx;
mod waypoint;
mod wpt;

fn main() -> io::Result<()> {
    let waypoints2 = vec![
        waypoint::Waypoint {
            name: "AAT054".to_string(),
            latitude: 36.276783,
            longitude: 140.145417,
            altitude: 540,
            description: "ASIO HG TO".to_string(),
        },
        waypoint::Waypoint {
            name: "ACT052".to_string(),
            latitude: 36.269475,
            longitude: 140.141306,
            altitude: 524,
            description: "COO TO".to_string(),
        },
    ];

    let gpx = gpx::formatter::format(&waypoints2).unwrap();
    let x = String::from_utf8(gpx).unwrap();
    println!("{}", x);

    Ok(())
}
