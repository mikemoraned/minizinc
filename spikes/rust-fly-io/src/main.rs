#![deny(warnings)]

use warp::Filter;
use std::{fs, io};

#[tokio::main]
async fn main() -> io::Result<()> {
    let entries = fs::read_dir(".")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    println!("dir entries: {:?}", entries);

    let build_tag = match fs::read_to_string("./build_tag") {
        Ok(contents) => contents,
        Err(error) => format!("{:?}", error)
    };

    println!("build_tag: {:?}", build_tag);

    // Match any request and return hello world!
    let routes = warp::any().map(move || {
        format!("Hello, World! Build tag: {:?}", build_tag)
    });

    Ok(warp::serve(routes)
        // ipv6 + ipv6 any addr
        .run(([0, 0, 0, 0, 0, 0, 0, 0], 8080))
        .await)
}
