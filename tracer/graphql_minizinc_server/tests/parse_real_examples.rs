use graphql_minizinc_server::parameters_from_model;
use graphql_minizinc_server::graphql::{MinizincParameter, MinizincIntegerParameter, MinizincParameters};

#[test]
fn test_nqueens() {
    let bytes = include_bytes!("../examples/nqueens.mzn");
    let model_string = String::from_utf8_lossy(bytes);
    print!("model:\n{}", &model_string);

    let parameters = parameters_from_model(&model_string);
    assert_eq!(parameters,
               MinizincParameters {
                   list: vec![MinizincParameter::Integer(
                       MinizincIntegerParameter {
                           name: "n".into(),
                           value: None
                       })]});
}