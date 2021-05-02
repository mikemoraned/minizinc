use crate::minizinc::Model;
use crate::graphql::MinizincParameters;

pub fn parameters_from_model(_model: &Model) -> MinizincParameters {
    MinizincParameters::new()
}