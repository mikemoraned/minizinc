fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use testcontainers::{clients, images, Docker};
    use rusoto_dynamodb::{DynamoDbClient, ListTablesInput, DynamoDb};
    use rusoto_core::credential::StaticProvider;
    use rusoto_core::{HttpClient, Region};

    #[tokio::test]
    async fn dynamodb_local_no_tables_by_default() {
        let _ = pretty_env_logger::try_init();
        let docker = clients::Cli::default();
        let node = docker.run(images::dynamodb_local::DynamoDb::default());
        let host_port = node.get_host_port(8000).unwrap();

        let dynamodb = build_dynamodb_client(host_port);
        let list_tables_input: ListTablesInput = Default::default();

        let result = dynamodb.list_tables(list_tables_input).await;
        assert_eq!(result.is_ok(), true);
        if let Ok(output) = result {
            assert_eq!(output.table_names, Some(vec![]))
        }
    }

    fn build_dynamodb_client(host_port: u16) -> DynamoDbClient {
        let credentials_provider =
            StaticProvider::new("fakeKey".to_string(), "fakeSecret".to_string(), None, None);

        let dispatcher = HttpClient::new().expect("could not create http client");

        let region = Region::Custom {
            name: "dynamodb-local".to_string(),
            endpoint: format!("http://localhost:{}", host_port),
        };

        DynamoDbClient::new_with(dispatcher, credentials_provider, region)
    }
}