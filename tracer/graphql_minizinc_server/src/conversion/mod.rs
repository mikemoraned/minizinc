use crate::minizinc::{Model, TiExprAndId, BaseType};
use crate::graphql::{MinizincParameters, MinizincParameter, MinizincIntegerParameter};

pub fn parameters_from_model(model: &Model) -> MinizincParameters {
    MinizincParameters {
        list: model.expressions.iter().flat_map(parameter_from_expression).collect()
    }
}

fn parameter_from_expression(expression: &TiExprAndId) -> Option<MinizincParameter> {
    match expression.base_type {
        BaseType::INT => Some(MinizincParameter::Integer(MinizincIntegerParameter{ name: expression.ident.0.clone()})),
        _ => None
    }
}