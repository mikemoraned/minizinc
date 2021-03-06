use crate::minizinc::{Model, TiExprAndId, BaseType};
use crate::graphql::{MinizincParameters, MinizincParameter, MinizincIntegerParameter, MinizincBooleanParameter, MinizincFloatParameter, MinizincStringParameter};

pub fn parameters_from_model(model: &Model) -> MinizincParameters {
    MinizincParameters {
        list: model.expressions.iter().map(parameter_from_expression).collect()
    }
}

fn parameter_from_expression(expression: &TiExprAndId) -> MinizincParameter {
    match expression.base_type {
        BaseType::BOOL =>
            MinizincParameter::Boolean(
                MinizincBooleanParameter{ name: expression.ident.0.clone(), value: None }
            ),
        BaseType::INT =>
            MinizincParameter::Integer(
                MinizincIntegerParameter{ name: expression.ident.0.clone(), value: None }
            ),
        BaseType::FLOAT =>
            MinizincParameter::Float(
                MinizincFloatParameter{ name: expression.ident.0.clone(), value: None }
            ),
        BaseType::STRING =>
            MinizincParameter::String(
                MinizincStringParameter{ name: expression.ident.0.clone(), value: None }
            ),
    }
}

#[cfg(test)]
mod tests {
    use crate::conversion::parameter_from_expression;
    use crate::minizinc::{TiExprAndId, BaseType, Ident};
    use crate::graphql::{MinizincParameter, MinizincIntegerParameter, MinizincBooleanParameter, MinizincStringParameter};

    #[test]
    fn test_base_type_bool() {
        assert_eq!(parameter_from_expression(
            &TiExprAndId{
                base_type: BaseType::BOOL,
                ident: Ident("bool".into())
            }),
                   MinizincParameter::Boolean(
                       MinizincBooleanParameter{
                           name: "bool".into(),
                           value: None
                       })
        );
    }

    #[test]
    fn test_base_type_int() {
        assert_eq!(parameter_from_expression(
            &TiExprAndId{
                base_type: BaseType::INT,
                ident: Ident("int".into())
            }),
                   MinizincParameter::Integer(
                       MinizincIntegerParameter{
                           name: "int".into(),
                           value: None
                       })
        );
    }

    #[test]
    fn test_base_type_float() {
        let converted = parameter_from_expression(
            &TiExprAndId{
                base_type: BaseType::FLOAT,
                ident: Ident("float".into())
            });
        if let MinizincParameter::Float(float_parameter) = converted {
            let expected_name : String = "float".into();
            assert_eq!(float_parameter.name, expected_name);
            assert_eq!(float_parameter.value, None);
        }
        else {
            panic!("not a Float")
        }
    }

    #[test]
    fn test_base_type_string() {
        assert_eq!(parameter_from_expression(
            &TiExprAndId{
                base_type: BaseType::STRING,
                ident: Ident("string".into())
            }),
                   MinizincParameter::String(
                       MinizincStringParameter{
                           name: "string".into(),
                           value: None
                       })
        );
    }
}