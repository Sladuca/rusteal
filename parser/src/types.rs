use std::collections::HashMap;

use rusteal_ast::{
	contract::Contract,
	typing::{TypePrimitive},
	struct_def::StructDef,
};
use nom::{
	IResult,
	branch::alt,
	sequence::{preceded, delimited, separated_pair, terminated},
	multi::separated_list1,
	bytes::complete::tag,
	character::complete::alphanumeric1,
	combinator::{map_res, map},
};
use crate::parse_error::ParseError;

use super::utils::opt_space_delimited;


pub fn local_schema(input: &str) -> IResult<&str, StructDef> {
	preceded(
		tag("schema local "), 
	struct_def	
	)(input)
}

pub fn global_schema(input: &str) -> IResult<&str, StructDef> {
	preceded(
		tag("schema global "),
	struct_def
	)(input)
}

pub fn struct_def(input: &str) -> IResult<&str, StructDef> {
	delimited(
		opt_space_delimited(tag("{")), 
		struct_def_fields,
		opt_space_delimited(tag("}"))
	)(input)
}

pub fn struct_def_fields(input: &str) -> IResult<&str, StructDef> {
	let (rest, fields) = separated_list1(tag(","), struct_def_field)(input)?;
	let fields = HashMap::from_iter(fields);
	Ok((rest, StructDef { fields }))
}

pub fn struct_def_field(input: &str) -> IResult<&str, (String, TypePrimitive)> {
	separated_pair(
	map(alphanumeric1, |name: &str| name.to_string()), 
	opt_space_delimited(tag(":")), 
	terminated(
		type_primitive,
		tag(",")
		)
	)(input)
}

pub fn type_primitive(input: &str) -> IResult<&str, TypePrimitive> {
	map_res(
		alt((
			tag("Void"),
			tag("UInt64"),
			tag("Byteslice"),
			tag("Halt")
		)),
		|s: &str| {
			TypePrimitive::try_from(s).map_err(|_| ParseError::InvalidTypeName(s))
		}
	)(input)
}