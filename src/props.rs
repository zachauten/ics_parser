use nom::{
  branch::alt,
  bytes::complete::tag,
  bytes::complete::take_till,
  bytes::complete::is_not,
  character::complete::crlf,
  sequence::delimited,
  sequence::pair,
};

// use many(alt()) to parse multiple components 

// pub fn cal_props(i: &str) -> nom::IResult<&str, &str> {

// }

// fn calscale(i: &str) -> nom::IResult<&str, &str> {

// }

// fn method() {

// }

// fn prodid(i: &str) -> nom::IResult<&str, &str> {

// }

// pub fn version(i: &str) -> nom::IResult<&str, &str> {
// }