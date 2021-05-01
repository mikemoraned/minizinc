extern crate nom;

use crate::minizinc::model;
use actix_web::{guard, web, App, HttpResponse, HttpServer, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{Request, Response};
use async_graphql::extensions::ApolloTracing;
use async_graphql::*;

mod minizinc;
mod graphql;

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bytes = include_bytes!("../model.mzn");
    let model_string = String::from_utf8_lossy(bytes);
    print!("model:\n{}", &model_string);
    print!("parsed:\n{:#?}", model(&model_string));

    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .extension(ApolloTracing)
        .finish();

    println!("{}", &schema.sdl());

    println!("Playground: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
