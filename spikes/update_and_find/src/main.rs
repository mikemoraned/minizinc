fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use testcontainers::{clients, images, Docker};
    use redis::{Commands, RedisResult, Iter};
    use std::collections::HashSet;

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

    #[tokio::test]
    async fn can_set_state() {
        let _ = pretty_env_logger::try_init();
        let docker = clients::Cli::default();
        let node = docker.run(images::redis::Redis::default());

        let host_port = node.get_host_port(6379).unwrap();
        let url = format!("redis://localhost:{}", host_port);

        let client = redis::Client::open(url.as_ref()).unwrap();
        let mut con = client.get_connection().unwrap();

        let initial_state : RedisResult<String> = con.hget("states", "some_id");
        assert!(initial_state.is_err());

        let expected_state = "some_state".to_string();
        con.hset::<&str, &str, &String, u8>("states", "some_id", &expected_state).unwrap();

        let current_state : String = con.hget("states", "some_id").unwrap();
        assert_eq!(expected_state, current_state);
    }

    #[tokio::test]
    async fn can_update_state() {
        let _ = pretty_env_logger::try_init();
        let docker = clients::Cli::default();
        let node = docker.run(images::redis::Redis::default());

        let host_port = node.get_host_port(6379).unwrap();
        let url = format!("redis://localhost:{}", host_port);

        let client = redis::Client::open(url.as_ref()).unwrap();
        let mut con = client.get_connection().unwrap();

        let initial_state : RedisResult<String> = con.hget("states", "some_id");
        assert!(initial_state.is_err());

        let intermediate_state = "some_other_state".to_string();
        con.hset::<&str, &str, &String, u8>("states", "some_id", &intermediate_state).unwrap();

        let mut current_state : String = con.hget("states", "some_id").unwrap();
        assert_eq!(intermediate_state, current_state);

        let expected_state = "some_state".to_string();
        con.hset::<&str, &str, &String, u8>("states", "some_id", &expected_state).unwrap();

        current_state = con.hget("states", "some_id").unwrap();
        assert_eq!(expected_state, current_state);
    }

    #[tokio::test]
    async fn can_find_all_states() {
        let _ = pretty_env_logger::try_init();
        let docker = clients::Cli::default();
        let node = docker.run(images::redis::Redis::default());

        let host_port = node.get_host_port(6379).unwrap();
        let url = format!("redis://localhost:{}", host_port);

        let client = redis::Client::open(url.as_ref()).unwrap();
        let mut con = client.get_connection().unwrap();

        con.hset::<&str, &str, &str, u8>("states", "id1", "state1").unwrap();
        con.hset::<&str, &str, &str, u8>("states", "id2", "state2").unwrap();

        let states_iter: Iter<'_, (String, String)>= con.hscan("states").unwrap();
        let expected_states : HashSet<(String, String)>
            = vec![
                ("id1".to_string(), "state1".to_string()),
                ("id2".to_string(), "state2".to_string())]
            .into_iter().collect();
        let states : HashSet<(String, String)> = states_iter.collect();
        assert_eq!(expected_states, states);
    }
}