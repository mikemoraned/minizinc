use crate::graphql::MinizincParameters;
use crate::minizinc::model;

pub mod minizinc;
pub mod graphql;
pub mod conversion;

pub fn parameters_from_model(model_string: &str) -> MinizincParameters {
    let (_unparsed, model) = model(&model_string).unwrap();
    print!("parsed:\n{:#?}", &model);

    let parameters: MinizincParameters = conversion::parameters_from_model(&model);
    parameters
}
