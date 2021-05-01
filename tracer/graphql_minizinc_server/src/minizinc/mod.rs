use core::result::Result::Ok;
use nom::branch::{alt, permutation};
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, line_ending, space0, space1};
use nom::combinator::recognize;
use nom::error::{context, VerboseError};
use nom::multi::{many0, many1};
use nom::sequence::{pair, terminated, tuple};
use nom::IResult;

#[derive(Debug, PartialEq, Eq)]
pub enum BaseType {
    BOOL,
    INT,
    FLOAT,
    STRING
}

#[derive(Debug, PartialEq, Eq)]
pub struct Ident(String);

#[derive(Debug, PartialEq, Eq)]
pub struct TiExprAndId {
    base_type: BaseType,
    ident: Ident
}

#[derive(Debug, PartialEq, Eq)]
pub struct Model {
    expressions: Vec<TiExprAndId>
}

type Res<T, U> = IResult<T, U, VerboseError<T>>;

fn base_type(input: &str) -> Res<&str, BaseType> {
    let mut parser = context(
        "base_type",alt((tag("bool"), tag("int"), tag("float"), tag("string"))));
    let (next_input, name) = parser(input)?;
    Ok((next_input, match name {
        "bool"   => BaseType::BOOL,
        "int"    => BaseType::INT,
        "float"  => BaseType::FLOAT,
        "string" => BaseType::STRING,
        _ => panic!()
    }))
}

fn ident(input: &str) -> Res<&str, Ident> {
    let mut parser =
        context("ident",
                recognize( many1(alt((alphanumeric1, tag("_"))))));
    // TODO: handle full: [A-Za-z][A-Za-z0-9_]* | ’[^’\xa\xd\x0]*’
    let (next_input, name) = parser(input)?;
    Ok((next_input, Ident(name.to_string())))
}

fn ti_expr_and_id(input: &str) -> Res<&str, TiExprAndId> {
    let mut parser =
        context("ti_expr_and_id",
                tuple(( base_type, space0, tag(":"), space1, ident )));
    parser(input).map(|(next_input, res)| {
        let base_type = res.0;
        let ident = res.4;
        (next_input, TiExprAndId { base_type, ident })
    })
}

pub fn model(input: &str) -> Res<&str, Model> {
    let separator = pair(tag(";"), permutation((space0, many0(line_ending))));
    let mut parser =
        context("model",
                many0(terminated(ti_expr_and_id, separator))
        );
    parser(input).map(|(next_input, expressions)| {
        (next_input, Model { expressions })
    })
}

#[cfg(test)]
mod tests {
    use nom::{
        Err as NomErr,
        error::{ErrorKind, VerboseError, VerboseErrorKind},
    };
    use crate::minizinc::{BaseType, Ident, ti_expr_and_id, TiExprAndId, model, Model, base_type, ident};

    #[test]
    fn test_base_type_bool() {
        assert_eq!(base_type("bool"), Ok(("", BaseType::BOOL)));
    }

    #[test]
    fn test_base_type_int() {
        assert_eq!(base_type("int"), Ok(("", BaseType::INT)));
    }

    #[test]
    fn test_base_type_float() {
        assert_eq!(base_type("float"), Ok(("", BaseType::FLOAT)));
    }

    #[test]
    fn test_base_type_string() {
        assert_eq!(base_type("string"), Ok(("", BaseType::STRING)));
    }

    #[test]
    fn test_base_type_error() {
        assert_eq!(base_type("shazbat"), Err(NomErr::Error(VerboseError {
            errors: vec![
                ("shazbat", VerboseErrorKind::Nom(ErrorKind::Tag)),
                ("shazbat", VerboseErrorKind::Nom(ErrorKind::Alt)),
                ("shazbat", VerboseErrorKind::Context("base_type")),
            ]
        })));
    }

    #[test]
    fn test_ident_simple() {
        assert_eq!(ident("some"), Ok(("", Ident("some".to_string()))));
    }

    #[test]
    fn test_ident_simple_with_underscore() {
        assert_eq!(ident("some_ident"), Ok(("", Ident("some_ident".to_string()))));
    }

    #[test]
    fn test_ident_error_empty_string() {
        assert_eq!(ident(""), Err(NomErr::Error(VerboseError {
            errors: vec![
                ("", VerboseErrorKind::Nom(ErrorKind::Tag)),
                ("", VerboseErrorKind::Nom(ErrorKind::Alt)),
                ("", VerboseErrorKind::Nom(ErrorKind::Many1)),
                ("", VerboseErrorKind::Context("ident")),
            ]
        })));
    }

    #[test]
    fn test_ti_expr_and_id() {
        assert_eq!(ti_expr_and_id("int: foop"), Ok(("", TiExprAndId {
            base_type: BaseType::INT,
            ident: Ident("foop".to_string())
        })));
    }

    #[test]
    fn test_model_single_ti_expr_and_id() {
        assert_eq!(model("int: foop;"), Ok(("", Model {
            expressions: vec![
                TiExprAndId {
                    base_type: BaseType::INT,
                    ident: Ident("foop".to_string())
                }]
        })));
    }

    #[test]
    fn test_model_multi_ti_expr_and_id_on_same_line() {
        assert_eq!(model("int: foop; float: farp;"), Ok(("", Model {
            expressions: vec![
                TiExprAndId {
                    base_type: BaseType::INT,
                    ident: Ident("foop".to_string())
                },
                TiExprAndId {
                    base_type: BaseType::FLOAT,
                    ident: Ident("farp".to_string())
                }]
        })));
    }

    #[test]
    fn test_model_multi_ti_expr_and_id_over_multiple_lines() {
        assert_eq!(model("int: foop;\n\nfloat: farp;"), Ok(("", Model {
            expressions: vec![
                TiExprAndId {
                    base_type: BaseType::INT,
                    ident: Ident("foop".to_string())
                },
                TiExprAndId {
                    base_type: BaseType::FLOAT,
                    ident: Ident("farp".to_string())
                }]
        })));
    }
}
