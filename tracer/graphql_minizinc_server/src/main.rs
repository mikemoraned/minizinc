extern crate nom;

use crate::minizinc::model;
use crate::graphql::{graphql_server, MinizincParameters};

mod minizinc;
mod graphql;
mod conversion;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bytes = include_bytes!("../model.mzn");
    let model_string = String::from_utf8_lossy(bytes);
    print!("model:\n{}", &model_string);

    let (_unparsed, model) = model(&model_string).unwrap();
    print!("parsed:\n{:#?}", &model);

    let parameters : MinizincParameters = conversion::parameters_from_model(&model);

    let server = graphql_server(&parameters);

    server?.await
}
