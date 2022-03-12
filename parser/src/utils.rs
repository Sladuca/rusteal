use nom::{
	IResult,
	error::ParseError,
	sequence::{delimited, pair},
	character::complete::{space0, multispace0, alpha1, alphanumeric1},
	Parser,
	InputTakeAtPosition, AsChar, combinator::recognize, branch::alt, bytes::complete::tag, multi::many0,
};

pub fn opt_s<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
  where
  F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
  delimited(
    space0,
    inner,
   space0 
  )
}


pub fn opt_ms<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
  where
  F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
  delimited(
    multispace0,
    inner,
  multispace0,
  )
}

pub fn identifier(input: &str) -> IResult<&str, &str> {
  recognize(
    pair(
      alt((alpha1, tag("_"))),
      many0(alt((alphanumeric1, tag("_"))))
    )
  )(input)
}
