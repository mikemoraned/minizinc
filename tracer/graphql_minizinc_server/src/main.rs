extern crate nom;

use crate::minizinc::model;
use crate::graphql::graphql_server;

mod minizinc;
mod graphql;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bytes = include_bytes!("../model.mzn");
    let model_string = String::from_utf8_lossy(bytes);
    print!("model:\n{}", &model_string);
    print!("parsed:\n{:#?}", model(&model_string));

    let server = graphql_server();

    server?.await
}
