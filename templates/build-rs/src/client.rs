connectrpc::include_generated!();

use {{proto_name}}::v1::*;

use connectrpc::client::{ClientConfig, HttpClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let http = HttpClient::plaintext();
    let config = ClientConfig::new("http://127.0.0.1:8080".parse()?);

    let client = {{proto_name | pascal_case}}ServiceClient::new(http, config);

    let request = {{proto_name | pascal_case}}Request {
        name: "Developer".into(),
        ..Default::default()
    };

    println!("Sending request to {{proto_name | pascal_case}}Service with name: {}", request.name);
    let response = client.{{proto_name | snake_case}}(request).await?;

    let greeting = response.view().greeting;
    println!("Received response: {}", greeting);

    Ok(())
}
