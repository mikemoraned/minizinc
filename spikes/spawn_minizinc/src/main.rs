use std::str;
use tokio::process::Command;
use actix_web::{web, App, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("starting up");

    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(minizinc))
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

async fn minizinc() -> impl Responder {
    let mut list_dir = Command::new("minizinc");

    let output = list_dir
        .arg("-D n=8")
        .arg("--verbose")
        .arg("nqueens.mzn")
        .output()
        .await
        .expect("process failed to execute");

    format!("stderr: {:?}, stdout: {:?}",
            str::from_utf8(&output.stderr),
            str::from_utf8( &output.stdout )
    )
}
