use crate::models::{Duration, Timestamp};
use nom::bytes::complete::{tag, take};
use nom::character::complete::u16;
use nom::combinator::{all_consuming, complete, opt};
use nom::sequence::{preceded, terminated, tuple};
use nom::IResult;

pub fn parse_duration(input: &str) -> IResult<&str, Duration> {
    let (input, h) = opt(terminated(u16, tag("h")))(input)?;
    let (input, m) = opt(terminated(u16, tag("m")))(input)?;
    let (input, s) = opt(terminated(u16, tag("s")))(input)?;

    let h = h.unwrap_or(0) as i32;
    let m = m.unwrap_or(0) as i32;
    let s = s.unwrap_or(0) as i32;

    Ok((input, Duration::from_s(h * 3600 + m * 60 + s)))
}

pub fn parse_2_digits(input: &str) -> IResult<&str, i32> {
    let (input, n_str) = take(2usize)(input)?;
    let (_, n) = all_consuming(u16)(n_str)?;

    Ok((input, n as i32))
}

pub fn parse_timestamp(input: &str) -> IResult<&str, Timestamp> {
    let (input, h) = parse_2_digits(input)?;
    let (input, m) = preceded(tag(":"), parse_2_digits)(input)?;
    let (input, s) = opt(preceded(tag(":"), parse_2_digits))(input)?;
    let (input, d) = opt(preceded(tag(" +"), u16))(input)?;
    let d = d.unwrap_or(0) as i32;
    let s = s.unwrap_or(0);

    Ok((input, Timestamp::from_dhms(d, h, m, s)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duration() {
        assert_eq!(
            parse_duration("1h2m3s"),
            Ok(("", Duration::from_s(3600 + 120 + 3)))
        );

        assert_eq!(parse_duration("127s"), Ok(("", Duration::from_s(127))));
    }

    #[test]
    fn timestamp() {
        assert_eq!(
            parse_timestamp("01:02:03"),
            Ok(("", Timestamp::from_dhms(0, 1, 2, 3)))
        );
        assert_eq!(
            parse_timestamp("01:02"),
            Ok(("", Timestamp::from_dhms(0, 1, 2, 0)))
        );

        assert_eq!(
            parse_timestamp("01:02:03 +7"),
            Ok(("", Timestamp::from_dhms(7, 1, 2, 3)))
        );

        assert!(parse_timestamp("1:02:03").is_err());
    }
}
