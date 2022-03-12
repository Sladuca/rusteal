use nom::{
    branch::alt,
    bytes::complete::{escaped_transform, tag},
    character::{
        complete::char,
        complete::{none_of, one_of},
    },
    combinator::{map, map_res, recognize, value},
    multi::{many0, many1, separated_list0, separated_list1},
    sequence::{delimited, terminated},
    IResult,
};
use rusteal_ast::expression::{primitive::Primitive, Expr};

use super::parse_error::ParseError;

pub fn uint64(input: &str) -> IResult<&str, Expr> {
    map_res(
        recognize(separated_list1(char('_'), many1(one_of("0123456789")))),
        |r: &str| {
            r.replace("_", "")
                .parse()
                .map_err(|_| ParseError::InvalidNumValue(&r))
                .map(|v| Expr::Primitive(Primitive::UInt64(v)))
        },
    )(input)
}

pub fn byte_as_number(input: &str) -> IResult<&str, u8> {
    map_res(
        recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))),
        |r: &str| {
            r.replace("_", "")
                .parse::<u64>()
                .map_or(Err(ParseError::InvalidNumValue(&r)), |v| {
                    v.try_into().map_err(|_| ParseError::InvalidByteValue(&r))
                })
        },
    )(input)
}

pub fn array_byteslice(input: &str) -> IResult<&str, Expr> {
    let (rest, bytes) = delimited(
        char('['),
        separated_list0(tag(","), byte_as_number),
        char(']'),
    )(input)?;
    Ok((rest, Expr::Primitive(Primitive::Byteslice(bytes))))
}

pub fn quoted_byteslice(input: &str) -> IResult<&str, Expr> {
    map(delimited(char('"'), quoted_inner, char('"')), |r| {
        Expr::Primitive(Primitive::from(r))
    })(input)
}

pub fn quoted_inner_escape(input: &str) -> IResult<&str, String> {
    escaped_transform(none_of("\\\""), '\\', value("\"", char('\"')))(input)
}

pub fn quoted_inner(input: &str) -> IResult<&str, String> {
    alt((quoted_inner_escape, value("".to_string(), tag(""))))(input)
}

pub fn byteslice(input: &str) -> IResult<&str, Expr> {
    alt((array_byteslice, quoted_byteslice))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_uint64() {
        let (rest, res) = uint64("123").unwrap();
        assert_eq!(res, Expr::Primitive(Primitive::UInt64(123)));
        assert_eq!(rest, "");

        let (rest, res) = uint64("123_456_789").unwrap();
        assert_eq!(res, Expr::Primitive(Primitive::UInt64(123456789)));
        assert_eq!(rest, "");

        let (rest, res) = uint64("123_456_789_").unwrap();
        assert_eq!(res, Expr::Primitive(Primitive::UInt64(123456789)));
        assert_eq!(rest, "_");

        uint64("_123_456_789")
            .err()
            .expect("underscore at beginning of number should fail!");
    }

    #[test]
    fn test_parse_byteslice() {
        let (rest, res) = byteslice("\"hi\"").unwrap();
        assert_eq!(res, Expr::Primitive(Primitive::Byteslice(vec![104, 105])));
        assert_eq!(rest, "");

        let (rest, res) = byteslice("[0,2,3]").unwrap();
        assert_eq!(res, Expr::Primitive(Primitive::Byteslice(vec![0, 2, 3])));
        assert_eq!(rest, "");

        let (rest, res) = byteslice("\"\"hi").unwrap();
        assert_eq!(res, Expr::Primitive(Primitive::Byteslice(vec![])));
        assert_eq!(rest, "hi");

        let (rest, res) = byteslice("\"\\\"\"").unwrap();
        assert_eq!(res, Expr::Primitive(Primitive::Byteslice(vec![34])));
        assert_eq!(rest, "");

        byteslice("[100, 234, 356]")
            .err()
            .expect("array byteslice with value > 254 should fail!");
    }
}
