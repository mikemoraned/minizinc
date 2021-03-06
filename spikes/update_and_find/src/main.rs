use std::collections::HashSet;
use std::env;
use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use std::time::{SystemTime, UNIX_EPOCH};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let redis_url = env::var("FLY_REDIS_CACHE_URL").unwrap();
    println!("Using {}", redis_url);
    let redis_client = redis::Client::open(redis_url).unwrap();
    let client = Client::new(redis_client);
    for (id, state) in client.find_all_states() {
        println!("{}: {}", id, state);
    }

    HttpServer::new(move || {
        App::new()
            .data(client.clone())
            .route("/", web::get().to(list))
            .route("/update", web::post().to(update))
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

async fn list(client: web::Data<Client>) -> impl Responder {
    client.find_all_states()
        .iter()
        .map(|(id, state)| format!("{}: {}", id, state))
        .collect::<Vec<String>>()
        .join("\n")
}

async fn update(client: web::Data<Client>) -> impl Responder {
    let system_time = SystemTime::now();
    let since_the_epoch = system_time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let current_time_millis = since_the_epoch.as_millis();
    let arbitrary_id = format!("id{}", current_time_millis % 1000);
    let arbitrary_state = format!("state{}", current_time_millis % 7);
    client.set_state(&arbitrary_id, &arbitrary_state);
    HttpResponse::Ok().body(format!("{}: {}", arbitrary_id, arbitrary_state))
}

#[derive(Clone)]
struct Client {
    redis_client: redis::Client
}

impl Client {
    fn new(redis_client: redis::Client) -> Self {
        Client { redis_client }
    }

    fn set_state(&self, id: &str, state: &str) {
        use redis::Commands;
        let mut con = self.redis_client.get_connection().unwrap();

        con.hset::<&str, &str, &str, u8>("states", id, state).unwrap();
    }

    fn find_state(&self, id: &str) -> Option<String> {
        use redis::Commands;
        let mut con = self.redis_client.get_connection().unwrap();

        match con.hget("states", id)  {
            Ok(state) => Some(state),
            Err(_) => None
        }
    }

    fn find_all_states(&self) -> HashSet<(String, String)> {
        use redis::{Commands, Iter};
        let mut con = self.redis_client.get_connection().unwrap();

        let states_iter: Iter<'_, (String, String)>= con.hscan("states").unwrap();
        states_iter.collect()
    }
}

#[cfg(test)]
mod tests {
    use testcontainers::{clients, images, Docker};
    use std::collections::HashSet;
    use crate::Client;

    #[tokio::test]
    async fn no_states_by_default() {
        let _ = pretty_env_logger::try_init();
        let docker = clients::Cli::default();
        let node = docker.run(images::redis::Redis::default());

        let host_port = node.get_host_port(6379).unwrap();
        let url = format!("redis://localhost:{}", host_port);

        let redis_client = redis::Client::open(url.as_ref()).unwrap();
        let client = Client::new(redis_client);
        let states = client.find_all_states();
        assert_eq!(0, states.len());
    }

    #[tokio::test]
    async fn can_set_state() {
        let _ = pretty_env_logger::try_init();
        let docker = clients::Cli::default();
        let node = docker.run(images::redis::Redis::default());

        let host_port = node.get_host_port(6379).unwrap();
        let url = format!("redis://localhost:{}", host_port);

        let redis_client = redis::Client::open(url.as_ref()).unwrap();
        let client = Client::new(redis_client);

        let initial_state  = client.find_state(&"some_id".to_string());
        assert_eq!(None, initial_state);

        let expected_state = "some_state".to_string();
        client.set_state(&"some_id".to_string(), &expected_state);

        let current_state = client.find_state(&"some_id".to_string());
        assert_eq!(Some(expected_state), current_state);
    }

    #[tokio::test]
    async fn can_update_state() {
        let _ = pretty_env_logger::try_init();
        let docker = clients::Cli::default();
        let node = docker.run(images::redis::Redis::default());

        let host_port = node.get_host_port(6379).unwrap();
        let url = format!("redis://localhost:{}", host_port);

        let redis_client = redis::Client::open(url.as_ref()).unwrap();
        let client = Client::new(redis_client);

        let initial_state  = client.find_state(&"some_id".to_string());
        assert_eq!(None, initial_state);

        let intermediate_state = "some_other_state".to_string();
        client.set_state(&"some_id".to_string(), &intermediate_state);

        let mut current_state = client.find_state(&"some_id".to_string()).unwrap();
        assert_eq!(intermediate_state, current_state);

        let expected_state = "some_state".to_string();
        client.set_state(&"some_id".to_string(), &expected_state);

        current_state = client.find_state(&"some_id".to_string()).unwrap();
        assert_eq!(expected_state, current_state);
    }

    #[tokio::test]
    async fn can_find_all_states() {
        let _ = pretty_env_logger::try_init();
        let docker = clients::Cli::default();
        let node = docker.run(images::redis::Redis::default());

        let host_port = node.get_host_port(6379).unwrap();
        let url = format!("redis://localhost:{}", host_port);

        let redis_client = redis::Client::open(url.as_ref()).unwrap();
        let client = Client::new(redis_client);

        client.set_state(&"id1".to_string(), &"state1".to_string());
        client.set_state(&"id2".to_string(), &"state2".to_string());

        let expected_states : HashSet<(String, String)>
            = vec![
                ("id1".to_string(), "state1".to_string()),
                ("id2".to_string(), "state2".to_string())]
            .into_iter().collect();
        let states = client.find_all_states();
        assert_eq!(expected_states, states);
    }
}