connectrpc::include_generated!();

use {{proto_name}}::v1::*;

use connectrpc::{RequestContext, Response, Router as ConnectRouter, ServiceRequest, ServiceResult};
use std::sync::Arc;

struct {{proto_name | pascal_case}}Server;

#[allow(refining_impl_trait)]
impl {{proto_name | pascal_case}}Service for {{proto_name | pascal_case}}Server {
    async fn {{proto_name | snake_case}}(
        &self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, {{proto_name | pascal_case}}Request>,
    ) -> ServiceResult<{{proto_name | pascal_case}}Response> {
        let name = if request.name.is_empty() { "Guest" } else { request.name };
        let greeting = format!("Hello, {}!", name);
        Response::ok({{proto_name | pascal_case}}Response {
            greeting,
            ..Default::default()
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service = Arc::new({{proto_name | pascal_case}}Server);
    let connect_router = service.register(ConnectRouter::new());

    let app = axum::Router::new()
        .fallback_service(connect_router.into_axum_service());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server running on http://127.0.0.1:8080");
    axum::serve(listener, app).await?;
    Ok(())
}
