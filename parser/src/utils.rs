use nom::{
	IResult,
	error::ParseError,
	sequence::delimited,
	character::complete::space0,
	Parser,
	InputTakeAtPosition, AsChar
};


pub fn opt_space_delimited<I, O, E, F>(f: F) -> impl FnMut(I) -> IResult<I, O, E>
where
	I: InputTakeAtPosition,
	E: ParseError<I>,
	F: Parser<I, O, E>,
	<I as InputTakeAtPosition>::Item: AsChar + Clone
{
	delimited(space0, f, space0)
}