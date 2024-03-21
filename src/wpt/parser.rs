use crate::waypoint::Waypoint;
use nom::bytes::complete::tag;
use nom::character::complete::{
    alphanumeric1, char, line_ending, not_line_ending, one_of, space1, u16,
};
use nom::combinator::{all_consuming, map};
use nom::multi::many0;
use nom::sequence::{preceded, tuple};
use nom::IResult;

pub fn parse(input: &str) -> Result<Vec<Waypoint<String>>, String> {
    match all_consuming(waypoints)(input) {
        Ok((_, waypoints)) => Ok(waypoints),
        Err(e) => Err(e.to_string()),
    }
}

fn waypoints(input: &str) -> IResult<&str, Vec<Waypoint<String>>> {
    preceded(header, many0(waypoint))(input)
}

fn header(input: &str) -> IResult<&str, ()> {
    map(tuple((tag("$FormatGEO"), line_ending)), |_| ())(input)
}

fn waypoint(input: &str) -> IResult<&str, Waypoint<String>> {
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
            name: name.to_string(),
            latitude,
            longitude,
            altitude,
            description: description.to_string(),
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
