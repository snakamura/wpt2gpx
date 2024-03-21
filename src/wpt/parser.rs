use super::super::waypoint::Waypoint;
use nom::bytes::complete::tag;
use nom::character::complete::{
    alphanumeric1, char, line_ending, not_line_ending, one_of, space1, u16,
};
use nom::combinator::{all_consuming, map};
use nom::error::Error;
use nom::multi::many0;
use nom::sequence::{preceded, tuple};
use nom::Err;
use nom::IResult;

pub fn parse(input: &str) -> Result<Vec<Waypoint<&str>>, Err<Error<&str>>> {
    match all_consuming(waypoints)(input) {
        Ok((_, waypoints)) => Ok(waypoints),
        Err(e) => Err(e),
    }
}

fn waypoints(input: &str) -> IResult<&str, Vec<Waypoint<&str>>> {
    preceded(header, many0(waypoint))(input)
}

fn header(input: &str) -> IResult<&str, ()> {
    map(tuple((tag("$FormatGEO"), line_ending)), |_| ())(input)
}

fn waypoint(input: &str) -> IResult<&str, Waypoint<&str>> {
    map(
        tuple((
            name,
            space1,
            latitude,
            space1,
            longitude,
            space1,
            altitude,
            space1,
            description,
            line_ending,
        )),
        |(name, _, latitude, _, longitude, _, altitude, _, description, _)| Waypoint {
            name: name,
            latitude,
            longitude,
            altitude,
            description: description,
        },
    )(input)
}

fn name(input: &str) -> IResult<&str, &str> {
    alphanumeric1(input)
}

fn latitude(input: &str) -> IResult<&str, f64> {
    map(tuple((one_of("NS"), space1, degree)), |(ns, _, degree)| {
        let b = if ns == 'N' { 1f64 } else { -1f64 };
        b * degree
    })(input)
}

fn longitude(input: &str) -> IResult<&str, f64> {
    map(tuple((one_of("EW"), space1, degree)), |(ns, _, degree)| {
        let b = if ns == 'E' { 1f64 } else { -1f64 };
        b * degree
    })(input)
}

fn degree(input: &str) -> IResult<&str, f64> {
    map(
        tuple((u16, space1, u16, space1, u16, char('.'), u16)),
        |(degrees, _, minutes, _, seconds, _, milliseconds)| {
            f64::from(degrees)
                + f64::from(minutes) / 60f64
                + (f64::from(seconds) + f64::from(milliseconds) / 100f64) / 60f64 / 60f64
        },
    )(input)
}

fn altitude(input: &str) -> IResult<&str, u16> {
    u16(input)
}

fn description(input: &str) -> IResult<&str, &str> {
    not_line_ending(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use nom::error::{Error, ErrorKind};
    use nom::Err;

    #[test]
    fn test_parse() {
        let waypoints = parse(
            r#"$FormatGEO
AAT054    N 36 16 36.42    E 140 08 43.50   540  ASIO HG TO
ACT052    N 36 16 10.11    E 140 08 28.70   524  COO TO
"#,
        )
        .unwrap();
        assert_eq!(waypoints.len(), 2);

        assert_eq!(
            parse(
                r#"$FormatGEO
AAT054    N 36 16 36.42    E 140 08 43.50   540  ASIO HG TO
ACT052    N 36 16 10.11    E 140 08 28.70   524  COO TO
X"#,
            ),
            Err(Err::Error(Error::new("X", ErrorKind::Eof)))
        );
    }

    #[test]
    fn test_waypoints() {
        assert_eq!(waypoints("$FormatGEO\n"), Ok(("", vec![])));
        let (_, waypoints) = waypoints(
            r#"$FormatGEO
AAT054    N 36 16 36.42    E 140 08 43.50   540  ASIO HG TO
ACT052    N 36 16 10.11    E 140 08 28.70   524  COO TO
"#,
        )
        .unwrap();
        assert_eq!(waypoints.len(), 2);
    }

    #[test]
    fn test_header() {
        assert_eq!(header("$FormatGEO\nAAT054"), Ok(("AAT054", ())));
        assert_eq!(
            header("$FormatGEO"),
            Err(Err::Error(Error::new("", ErrorKind::CrLf)))
        );
        assert_eq!(
            header("$FormatGEOX\nAAT054"),
            Err(Err::Error(Error::new("X\nAAT054", ErrorKind::CrLf)))
        );
    }

    #[test]
    fn test_waypoint() {
        let (
            _,
            Waypoint {
                name,
                latitude,
                longitude,
                altitude,
                description,
            },
        ) = waypoint("AAT054    N 36 16 36.42    E 140 08 43.50   540  ASIO HG TO\n").unwrap();
        assert_eq!(name, "AAT054");
        assert_approx_eq!(latitude, 36.276783);
        assert_approx_eq!(longitude, 140.145417);
        assert_eq!(altitude, 540);
        assert_eq!(description, "ASIO HG TO");

        assert_eq!(
            waypoint("AAT054    N 36 16 36.42    E 140 08 43.50   XXX  ASIO HG TO\n"),
            Err(Err::Error(Error::new(
                "XXX  ASIO HG TO\n",
                ErrorKind::Digit
            )))
        )
    }

    #[test]
    fn test_name() {
        assert_eq!(name("AAT054 "), Ok((" ", "AAT054")));
    }

    #[test]
    fn test_latitude() {
        assert_approx_eq!(latitude("N 36 16 36.42").unwrap().1, 36.276783);
        assert_approx_eq!(latitude("S 36 16 36.42").unwrap().1, -36.276783);
        assert_eq!(
            latitude("N 36 16 36"),
            Err(Err::Error(Error::new("", ErrorKind::Char)))
        );
    }

    #[test]
    fn test_longitude() {
        assert_approx_eq!(longitude("E 140 08 43.50").unwrap().1, 140.145417);
        assert_approx_eq!(longitude("W 140 08 43.50").unwrap().1, -140.145417);
        assert_eq!(
            longitude("140 08 43.50"),
            Err(Err::Error(Error::new("140 08 43.50", ErrorKind::OneOf)))
        );
    }

    #[test]
    fn test_degree() {
        assert_approx_eq!(degree("140 08 43.50").unwrap().1, 140.145417);
        assert_approx_eq!(degree("36 16 36.42").unwrap().1, 36.276783);
        assert_eq!(
            degree("E 140 08 43.50"),
            Err(Err::Error(Error::new("E 140 08 43.50", ErrorKind::Digit)))
        );
    }

    #[test]
    fn test_altitude() {
        assert_eq!(altitude("540 "), Ok((" ", 540)));
        assert_eq!(altitude("1280"), Ok(("", 1280)));
        assert_eq!(
            altitude("A"),
            Err(Err::Error(Error::new("A", ErrorKind::Digit)))
        );
    }

    #[test]
    fn test_description() {
        assert_eq!(
            description("ASIO HG TO\nACT052"),
            Ok(("\nACT052", "ASIO HG TO"))
        );
        assert_eq!(description("ASIO HG TO"), Ok(("", "ASIO HG TO")));
    }
}
