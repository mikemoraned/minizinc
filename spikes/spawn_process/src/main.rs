use std::str;
use tokio::process::Command;
use actix_web::{web, App, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(list))
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

async fn list() -> impl Responder {
    let mut list_dir = Command::new("ls");

    let output = list_dir
        .arg("/")
        .output()
        .await
        .expect("process failed to execute");

    format!("stderr: {:?}, stdout: {:?}",
             str::from_utf8(&output.stderr),
             str::from_utf8( &output.stdout )
    )
}
