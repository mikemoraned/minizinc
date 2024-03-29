use actix_web::{guard, web, App, HttpResponse, HttpServer, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{Request, Response};
use async_graphql::extensions::ApolloTracing;
use async_graphql::*;
use actix_web::dev::Server;

struct Query;
type LocalSchema = Schema<Query, EmptyMutation, EmptySubscription>;

#[Object]
impl Query {
    async fn parameters(&self, context: &Context<'_>) -> Vec<MinizincParameter> {
        context.data_unchecked::<MinizincParameters>().list.clone()
    }
}

#[derive(SimpleObject, Clone, Debug, PartialEq, Eq)]
pub struct MinizincBooleanParameter {
    pub name: String,
    pub value: Option<bool>
}

#[derive(SimpleObject, Clone, Debug, PartialEq, Eq)]
pub struct MinizincIntegerParameter {
    pub name: String,
    pub value: Option<u32> // TODO: guessing a minizinc int is u32
}

#[derive(SimpleObject, Clone, Debug, PartialEq)]
pub struct MinizincFloatParameter {
    pub name: String,
    pub value: Option<f32> // TODO: guessing a minizinc int is f32
}

#[derive(SimpleObject, Clone, Debug, PartialEq, Eq)]
pub struct MinizincStringParameter {
    pub name: String,
    pub value: Option<String>
}

#[derive(Union, Clone, Debug, PartialEq)]
pub enum MinizincParameter {
    Boolean(MinizincBooleanParameter),
    Integer(MinizincIntegerParameter),
    Float(MinizincFloatParameter),
    String(MinizincStringParameter)
}

#[derive(Clone, Debug, PartialEq)]
pub struct MinizincParameters {
    pub list: Vec<MinizincParameter>
}

async fn index(schema: web::Data<LocalSchema>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
}

async fn index_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
        )))
}

pub fn graphql_server(parameters: &MinizincParameters) -> Result<Server, std::io::Error> {
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(parameters.clone())
        .extension(ApolloTracing)
        .finish();

    println!("{}", &schema.sdl());

    println!("Playground: http://localhost:8080");

    Ok(HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
    })
        .bind("0.0.0.0:8080")?
        .run())
}
