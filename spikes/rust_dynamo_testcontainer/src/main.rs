fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use testcontainers::{clients, images, Docker};
    use rusoto_dynamodb::{DynamoDbClient, ListTablesInput, DynamoDb, CreateTableInput, KeySchemaElement, AttributeDefinition, ProvisionedThroughput};
    use rusoto_core::credential::StaticProvider;
    use rusoto_core::{HttpClient, Region};

    #[tokio::test]
    async fn dynamodb_local_no_tables_by_default() {
        let _ = pretty_env_logger::try_init();
        let docker = clients::Cli::default();
        let node = docker.run(images::dynamodb_local::DynamoDb::default());
        let host_port = node.get_host_port(8000).unwrap();

        let dynamodb = build_dynamodb_client(host_port);
        assert_has_tables_named(dynamodb, vec![]).await
    }

    #[tokio::test]
    async fn dynamodb_local_can_create_table() {
        let _ = pretty_env_logger::try_init();
        let docker = clients::Cli::default();
        let node = docker.run(images::dynamodb_local::DynamoDb::default());
        let host_port = node.get_host_port(8000).unwrap();

        let dynamodb = build_dynamodb_client(host_port);

        let create_tables_input = CreateTableInput {
            table_name: "books".to_string(),
            key_schema: vec![KeySchemaElement {
                key_type: "HASH".to_string(),
                attribute_name: "title".to_string(),
            }],
            attribute_definitions: vec![AttributeDefinition {
                attribute_name: "title".to_string(),
                attribute_type: "S".to_string(),
            }],
            provisioned_throughput: Some(ProvisionedThroughput {
                read_capacity_units: 5,
                write_capacity_units: 5,
            }),
            ..Default::default()
        };

        let result = dynamodb.create_table(create_tables_input).await;
        assert_eq!(result.is_ok(), true);

        assert_has_tables_named(dynamodb, vec!["books".to_string()]).await
    }

    async fn assert_has_tables_named(dynamodb: DynamoDbClient, expected_table_names: Vec<String>) {
        let list_tables_input: ListTablesInput = Default::default();
        let result = dynamodb.list_tables(list_tables_input).await;
        assert_eq!(result.is_ok(), true);
        if let Ok(output) = result {
            assert_eq!(output.table_names, Some(expected_table_names))
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