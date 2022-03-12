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
	combinator::{map_res, map, opt},
};
use crate::parse_error::ParseError;

use super::utils::{opt_s, identifier, opt_ms};


pub fn local_schema(input: &str) -> IResult<&str, StructDef> {
	preceded(
		tag("schema local"), 
	struct_def	
	)(input)
}

pub fn global_schema(input: &str) -> IResult<&str, StructDef> {
	preceded(
		tag("schema global"),
	struct_def
	)(input)
}

pub fn struct_def(input: &str) -> IResult<&str, StructDef> {
	delimited(
		opt_ms(tag("{")), 
		struct_def_fields,
		opt_ms(tag("}"))
	)(input)
}

pub fn struct_def_fields(input: &str) -> IResult<&str, StructDef> {
	let (rest, fields) = terminated(separated_list1(tag(","), opt_ms(struct_def_field)), opt(tag(",")))(input)?;
	let fields = HashMap::from_iter(fields);
	Ok((rest, StructDef { fields }))
}

pub fn struct_def_field(input: &str) -> IResult<&str, (String, TypePrimitive)> {
	separated_pair(
	map(identifier, |name: &str| name.to_string()), 
	opt_s(tag(":")), 
	type_primitive,
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_primitive() {
		let (rest, res) = type_primitive("Void").unwrap();
		assert_eq!(res, TypePrimitive::Void);
		assert_eq!(rest, "");

		let (rest, res) = type_primitive("UInt64").unwrap();
		assert_eq!(res, TypePrimitive::UInt64);
		assert_eq!(rest, "");

		let (rest, res) = type_primitive("Byteslice").unwrap();
		assert_eq!(res, TypePrimitive::Byteslice);
		assert_eq!(rest, "");

		let (rest, res) = type_primitive("Halt").unwrap();
		assert_eq!(res, TypePrimitive::Halt);
		assert_eq!(rest, "");

		type_primitive("void").err().expect("primitive types names should be case-sensitive");
		type_primitive(" Void").err().expect("primitive types name should include optional whitespace");
		type_primitive("AHalt").err().expect("should not take prefix");
	}

	#[test]
	fn test_struct_def_field() {
		let (rest, (name, ty)) = struct_def_field("foo: Void").unwrap();
		assert_eq!(name, "foo");
		assert_eq!(ty, TypePrimitive::Void);
		assert_eq!(rest, "");

		let (rest, (name, ty)) = struct_def_field("length: UInt64,").unwrap();
		assert_eq!(name, "length");
		assert_eq!(ty, TypePrimitive::UInt64);
		assert_eq!(rest, ",");

		let (rest, (name, ty)) = struct_def_field("best_husbandos: Byteslice").unwrap();
		assert_eq!(name, "best_husbandos");
		assert_eq!(ty, TypePrimitive::Byteslice);
		assert_eq!(rest, "");

		let (rest, (name, ty)) = struct_def_field("its_time_to_st0p: Halt").unwrap();
		assert_eq!(name, "its_time_to_st0p");
		assert_eq!(ty, TypePrimitive::Halt);
		assert_eq!(rest, "");

		struct_def_field("foo:,").err().expect("field should contain type");
		struct_def_field("foo: ,").err().expect("field should contain type");
		struct_def_field("foo: MeaningOfLife,").err().expect("field type should be one of the defined primitives");
	}

	#[test]
	pub fn test_struct_def_fields() {
		let (rest, fields) = struct_def_fields("foo: Void, bar: UInt64, baz: Byteslice, qux: Halt").unwrap();
		let fields = fields.as_ref();
		assert_eq!(fields.len(), 4);
		assert_eq!(fields.get("foo"), Some(&TypePrimitive::Void));
		assert_eq!(fields.get("bar"), Some(&TypePrimitive::UInt64));
		assert_eq!(fields.get("baz"), Some(&TypePrimitive::Byteslice));
		assert_eq!(fields.get("qux"), Some(&TypePrimitive::Halt));
		assert_eq!(rest, "");

		let (rest, fields) = struct_def_fields("foo: Void, bar: UInt64, baz: Byteslice, qux: Halt,").unwrap();
		let fields = fields.as_ref();
		assert_eq!(fields.len(), 4);
		assert_eq!(fields.get("foo"), Some(&TypePrimitive::Void));
		assert_eq!(fields.get("bar"), Some(&TypePrimitive::UInt64));
		assert_eq!(fields.get("baz"), Some(&TypePrimitive::Byteslice));
		assert_eq!(fields.get("qux"), Some(&TypePrimitive::Halt));
		assert_eq!(rest, "");

		let (rest, fields) = struct_def_fields("foo: Void,\n bar: UInt64,\n baz: Byteslice,\n qux: Halt").unwrap();
		let fields = fields.as_ref();
		assert_eq!(fields.len(), 4);
		assert_eq!(fields.get("foo"), Some(&TypePrimitive::Void));
		assert_eq!(fields.get("bar"), Some(&TypePrimitive::UInt64));
		assert_eq!(fields.get("baz"), Some(&TypePrimitive::Byteslice));
		assert_eq!(fields.get("qux"), Some(&TypePrimitive::Halt));
		assert_eq!(rest, "");

		let (rest, fields) = struct_def_fields("\tfoo: Void,\n\tbar: UInt64,\n\tbaz: Byteslice,\n\tqux: Halt").unwrap();
		let fields = fields.as_ref();
		assert_eq!(fields.len(), 4);
		assert_eq!(fields.get("foo"), Some(&TypePrimitive::Void));
		assert_eq!(fields.get("bar"), Some(&TypePrimitive::UInt64));
		assert_eq!(fields.get("baz"), Some(&TypePrimitive::Byteslice));
		assert_eq!(fields.get("qux"), Some(&TypePrimitive::Halt));
		assert_eq!(rest, "");

		let (rest, fields) = struct_def_fields("\tfoo: Void,\r\n\tbar: UInt64,\r\n\tbaz: Byteslice,\r\n\tqux: Halt").unwrap();
		let fields = fields.as_ref();
		assert_eq!(fields.len(), 4);
		assert_eq!(fields.get("foo"), Some(&TypePrimitive::Void));
		assert_eq!(fields.get("bar"), Some(&TypePrimitive::UInt64));
		assert_eq!(fields.get("baz"), Some(&TypePrimitive::Byteslice));
		assert_eq!(fields.get("qux"), Some(&TypePrimitive::Halt));
		assert_eq!(rest, "");

		let (rest, fields) = struct_def_fields("\tfoo: Void,\r\n\t bar: UInt64,\t\r\n  \tbaz: Byteslice,\t  \t\r\nqux: Halt").unwrap();
		let fields = fields.as_ref();
		assert_eq!(fields.len(), 4);
		assert_eq!(fields.get("foo"), Some(&TypePrimitive::Void));
		assert_eq!(fields.get("bar"), Some(&TypePrimitive::UInt64));
		assert_eq!(fields.get("baz"), Some(&TypePrimitive::Byteslice));
		assert_eq!(fields.get("qux"), Some(&TypePrimitive::Halt));
		assert_eq!(rest, "");

		let (rest, fields) = struct_def_fields("foo: Void; bar: UInt64; baz: Byteslice; qux: Halt;").unwrap();
		let fields = fields.as_ref();
		assert_eq!(fields.len(), 1);
		assert_eq!(rest, "; bar: UInt64; baz: Byteslice; qux: Halt;");


		let (rest, fields) = struct_def_fields("\tfoo: Void\n\tbar: UInt64\n\tbaz: Byteslice\n\tqux: Halt").unwrap();
		let fields = fields.as_ref();
		assert_eq!(fields.len(), 1);
		assert_eq!(rest, "bar: UInt64\n\tbaz: Byteslice\n\tqux: Halt");
	}

	#[test]
	fn test_struct_def() {
		let (rest, def) = struct_def("{ foo: Void, bar: UInt64, baz: Byteslice, qux: Halt }").unwrap();
		assert_eq!(def.fields.len(), 4);
		assert_eq!(def.fields.get("foo"), Some(&TypePrimitive::Void));
		assert_eq!(def.fields.get("bar"), Some(&TypePrimitive::UInt64));
		assert_eq!(def.fields.get("baz"), Some(&TypePrimitive::Byteslice));
		assert_eq!(def.fields.get("qux"), Some(&TypePrimitive::Halt));
		assert_eq!(rest, "");

		let (rest, def) = struct_def("{\n\tfoo: Void,\n\t bar: UInt64,\n\t baz: Byteslice,\n\t qux: Halt\n}").unwrap();
		assert_eq!(def.fields.len(), 4);
		assert_eq!(def.fields.get("foo"), Some(&TypePrimitive::Void));
		assert_eq!(def.fields.get("bar"), Some(&TypePrimitive::UInt64));
		assert_eq!(def.fields.get("baz"), Some(&TypePrimitive::Byteslice));
		assert_eq!(def.fields.get("qux"), Some(&TypePrimitive::Halt));
		assert_eq!(rest, "");

		struct_def("{\n\tfoo: Void,\n\t bar: UInt64,\n\t baz: Byteslice,\n\t qux: Halt\n").err().expect("should fail on missing closing brace");
		struct_def("\n\tfoo: Void,\n\t bar: UInt64,\n\t baz: Byteslice,\n\t qux: Halt\n}").err().expect("should fail on missing opening brace");
		struct_def("\n\tfoo: Void,\n\t bar: UInt64,\n\t baz: Byteslice,\n\t qux: Halt\n").err().expect("should fail on no braces");
	}

	#[test]
	fn test_local_schema() {
		let (rest, def) = local_schema("schema local { foo: Void, bar: UInt64, baz: Byteslice, qux: Halt }").unwrap();
		assert_eq!(def.fields.len(), 4);
		assert_eq!(def.fields.get("foo"), Some(&TypePrimitive::Void));
		assert_eq!(def.fields.get("bar"), Some(&TypePrimitive::UInt64));
		assert_eq!(def.fields.get("baz"), Some(&TypePrimitive::Byteslice));
		assert_eq!(def.fields.get("qux"), Some(&TypePrimitive::Halt));
		assert_eq!(rest, "");

		let (rest, def) = local_schema("schema local\n{ foo: Void, bar: UInt64, baz: Byteslice, qux: Halt }").unwrap();
		assert_eq!(def.fields.len(), 4);
		assert_eq!(def.fields.get("foo"), Some(&TypePrimitive::Void));
		assert_eq!(def.fields.get("bar"), Some(&TypePrimitive::UInt64));
		assert_eq!(def.fields.get("baz"), Some(&TypePrimitive::Byteslice));
		assert_eq!(def.fields.get("qux"), Some(&TypePrimitive::Halt));
		assert_eq!(rest, "");

		let (rest, def) = local_schema("schema local{ foo: Void, bar: UInt64, baz: Byteslice, qux: Halt }").unwrap();
		assert_eq!(def.fields.len(), 4);
		assert_eq!(def.fields.get("foo"), Some(&TypePrimitive::Void));
		assert_eq!(def.fields.get("bar"), Some(&TypePrimitive::UInt64));
		assert_eq!(def.fields.get("baz"), Some(&TypePrimitive::Byteslice));
		assert_eq!(def.fields.get("qux"), Some(&TypePrimitive::Halt));
		assert_eq!(rest, "");

		local_schema("schema global { foo: Void, bar: UInt64, baz: Byteslice, qux: Halt }").err().expect("should fail if given global schema");
		local_schema("schema { foo: Void, bar: UInt64, baz: Byteslice, qux: Halt }").err().expect("should fail if local keyword not given");
		local_schema("{ foo: Void, bar: UInt64, baz: Byteslice, qux: Halt }").err().expect("should fail on just struct def");
	}


}