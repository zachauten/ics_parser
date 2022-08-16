use nom::{
    branch::permutation,
    bytes::complete::tag,
    bytes::complete::take_until,
    sequence::delimited,
    sequence::preceded,
    character::complete::crlf,
    character::complete::alphanumeric0,
    error::Error,
};
mod props;

#[derive(Debug)]
pub struct Calendar {
    version: String,
    prodid: String,
}

impl Calendar {

    pub fn new(version: String, prodid: String) -> Self {
        Self {
            version,
            prodid,
        }
    }
}

impl TryFrom<&str> for Calendar {
    type Error = String;
    
    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let (_, matched) = match delimited(start, required_cal_props, end)(text) {
            Ok(res) => res,
            Err(_) => return Err("Err parsing cal".to_owned()),
        };

        Ok(Self::new(matched.1.to_string(), matched.0.to_string()))
    }
}

fn ical(i: &str) -> nom::IResult<&str, (&str, &str)> {
    delimited(start, required_cal_props, end)(i)
}

fn start(i: &str) -> nom::IResult<&str, &str> {
    tag("BEGIN:VCALENDAR\r\n")(i)
}

fn end(i: &str) -> nom::IResult<&str, &str> {
    tag("END:VCALENDAR\r\n")(i)
}

fn required_cal_props(i: &str) -> nom::IResult<&str, (&str, &str)> {
    permutation((prodid, version))(i)
}

// https://www.rfc-editor.org/rfc/rfc5545#section-3.7.3
fn prodid(i: &str) -> nom::IResult<&str, &str> {
    delimited(tag("PRODID:"), take_until("\r\n"), crlf)(i)
}

// https://www.rfc-editor.org/rfc/rfc5545#section-3.7.4
// TODO: implement (minver ";" maxver)
fn version(i: &str) -> nom::IResult<&str, &str> {
    delimited(tag("VERSION:"), take_until("\r\n"), crlf)(i)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;
    use super::version;
    use super::Calendar;

    #[test]
    fn cal_from_str_file() {
        let filename = "tests/Calendar_Export_Short.ics";
        let content = fs::read_to_string(filename)
            .expect(&format!("error reading {}", filename));
        let cal = Calendar::try_from(content.as_str()).unwrap();
        assert_eq!("-//Apple Inc.//macOS 12.5//EN", cal.prodid);
        assert_eq!("2.0", cal.version);
    }

    #[test]
    fn cal_from_str_inline() {
        let content = "BEGIN:VCALENDAR\r\n\
            VERSION:2.0\r\n\
            PRODID:-//Apple Inc.//macOS 12.5//EN\r\n\
            END:VCALENDAR\r\n";
        let cal = Calendar::try_from(content);
        assert!(cal.is_ok());
    }

    #[test]
    fn cal_from_str_err() {
        let msg = "foo";
        let cal = Calendar::try_from(msg);
        assert_eq!("Err parsing cal", cal.unwrap_err());
    }

    #[test]
    fn test_required_cal_props_1() {
        let msg = "VERSION:2.0\r\nPRODID:-//ABC Corporation//NONSGML My Product//EN\r\n";
        let (rest, matched) = required_cal_props(msg).unwrap();
        assert_eq!("-//ABC Corporation//NONSGML My Product//EN", matched.0);
        assert_eq!("2.0", matched.1);
        assert_eq!("", rest);
    }

    #[test]
    fn test_required_cal_props_2() {
        let msg = "PRODID:-//ABC Corporation//NONSGML My Product//EN\r\nVERSION:2.0\r\n";
        let (rest, matched) = required_cal_props(msg).unwrap();
        assert_eq!("-//ABC Corporation//NONSGML My Product//EN", matched.0);
        assert_eq!("2.0", matched.1);
        assert_eq!("", rest);
    }

    #[test]
    fn test_version() {
        let msg = "VERSION:2.0\r\n";
        let (rest, matched) = version(&msg).unwrap();
        assert_eq!("2.0", matched);
        assert_eq!("", rest);
    }

    #[test]
    fn test_prodid() {
        let msg = "PRODID:-//ABC Corporation//NONSGML My Product//EN\r\n";
        let (rest, matched) = prodid(&msg).unwrap();
        assert_eq!("-//ABC Corporation//NONSGML My Product//EN".to_owned(), matched);
        assert_eq!("", rest);
    }
}
