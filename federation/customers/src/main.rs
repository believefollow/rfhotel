use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::Extension;
use axum::handler::get;
use axum::response::{self, IntoResponse};
use axum::{AddExtensionLayer, Router, Server};
use customers::{QueryRoot, Customers, CustomersSchema};

async fn graphql_handler(
    schema: Extension<CustomersSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    response::Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(Customers::new())
        .finish();

    let app = Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .layer(AddExtensionLayer::new(schema));

    println!("Playground: http://localhost:4001");

    Server::bind(&"0.0.0.0:4001".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
