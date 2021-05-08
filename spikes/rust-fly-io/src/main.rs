#![deny(warnings)]
extern crate dotenv;

use warp::Filter;
use std::env;
use dotenv::dotenv;
use std::{fs, io};

#[tokio::main]
async fn main() -> io::Result<()> {
    let entries = fs::read_dir(".")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    println!("dir entries: {:?}", entries);

    match fs::read_to_string("./.env") {
        Ok(contents) => print!("{}", contents),
        Err(error) => println!("{:?}", error)
    }

    dotenv().ok();
    let build_tag = env::var("BUILD_TAG");

    println!("Build tag: {:?}", build_tag);

    // Match any request and return hello world!
    let routes = warp::any().map(move || {
        format!("Hello, World! Build tag: {:?}", build_tag)
    });

    Ok(warp::serve(routes)
        // ipv6 + ipv6 any addr
        .run(([0, 0, 0, 0, 0, 0, 0, 0], 8080))
        .await)
}
