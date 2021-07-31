extern crate nom;

use graphql_minizinc_server::graphql::graphql_server;
use graphql_minizinc_server::parameters_from_model;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting");

    let bytes = include_bytes!("../../examples/all_supported_types.mzn");
    let model_string = String::from_utf8_lossy(bytes);
    print!("read model:\n{}", &model_string);

    let parameters = parameters_from_model(&model_string);
    println!("derived parameters: :{:?}", &parameters);

    let server = graphql_server(&parameters);
    println!("created server");

    println!("starting server");
    server?.await
}
