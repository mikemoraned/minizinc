fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use testcontainers::{clients, images, Docker};
    use redis::Commands;

    #[tokio::test]
    async fn no_states_by_default() {
        let _ = pretty_env_logger::try_init();
        let docker = clients::Cli::default();
        let node = docker.run(images::redis::Redis::default());

        let host_port = node.get_host_port(6379).unwrap();
        let url = format!("redis://localhost:{}", host_port);

        let client = redis::Client::open(url.as_ref()).unwrap();
        let mut con = client.get_connection().unwrap();

        let size: u128 = con.hlen("states").unwrap();
        assert_eq!(0, size);
    }
}