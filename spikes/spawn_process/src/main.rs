use std::str;
use tokio::process::Command;

#[tokio::main]
async fn main() {
    let mut list_dir = Command::new("ls");

    let output = list_dir
        .arg("/")
        .output()
        .await
        .expect("process failed to execute");

    println!("stderr: {:?}, stdout: {:?}",
             str::from_utf8(&output.stderr),
             str::from_utf8( &output.stdout )
    );
}
