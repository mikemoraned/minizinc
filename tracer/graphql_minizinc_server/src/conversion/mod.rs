use crate::minizinc::{Model, TiExprAndId, BaseType};
use crate::graphql::{MinizincParameters, MinizincParameter, MinizincIntegerParameter};

pub fn parameters_from_model(model: &Model) -> MinizincParameters {
    MinizincParameters {
        list: model.expressions.iter().flat_map(parameter_from_expression).collect()
    }
}

fn parameter_from_expression(expression: &TiExprAndId) -> Option<MinizincParameter> {
    match expression.base_type {
        BaseType::INT => Some(
            MinizincParameter::Integer(
                MinizincIntegerParameter{ name: expression.ident.0.clone()}
            )),
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use crate::conversion::parameter_from_expression;
    use crate::minizinc::{TiExprAndId, BaseType, Ident};
    use crate::graphql::{MinizincParameter, MinizincIntegerParameter};

    #[test]
    fn test_base_type_int() {
        assert_eq!(parameter_from_expression(
            &TiExprAndId{
                base_type: BaseType::INT,
                ident: Ident("int".into())
            }),
                   Some(MinizincParameter::Integer(
                       MinizincIntegerParameter{
                           name: "int".into()
                       })
                   )
        );
    }
}