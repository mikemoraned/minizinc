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
    /// Returns the sum of a and b
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
    async fn shapes(&self) -> Vec<Shape> {
        vec![Shape::Circle(Circle { radius: 2.5}), Shape::Square(Square { width: 10.9 })]
    }

    async fn parameters(&self, context: &Context<'_>) -> Vec<MinizincParameter> {
        // vec![MinizincParameter::Integer(MinizincIntegerParameter{ name: "foop".into()})]
        context.data_unchecked::<MinizincParameters>().list.clone()
    }
}

struct Circle {
    radius: f32,
}

#[Object]
impl Circle {
    async fn area(&self) -> f32 {
        std::f32::consts::PI * self.radius * self.radius
    }

    async fn scale(&self, s: f32) -> Shape {
        Circle { radius: self.radius * s }.into()
    }
}

struct Square {
    width: f32,
}

#[Object]
impl Square {
    async fn area(&self) -> f32 {
        self.width * self.width
    }

    async fn scale(&self, s: f32) -> Shape {
        Square { width: self.width * s }.into()
    }
}

#[derive(Union)]
enum Shape {
    Circle(Circle),
    Square(Square),
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

    println!("Playground: http://localhost:8000");

    Ok(HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
    })
        .bind("127.0.0.1:8000")?
        .run())
}
