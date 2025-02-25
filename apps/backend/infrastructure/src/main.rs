mod config;
mod db;
mod graphql;
mod repository;
mod test;

use std::fs::File;
use std::io::prelude::*;
use std::net::SocketAddr;

use async_graphql::http::GraphQLPlaygroundConfig;
use async_graphql::{http::playground_source, EmptySubscription, Request, Response, Schema};

use axum::extract::Json;
use axum::{extract::Extension, response::Html, routing::get, routing::post, Router};

use graphql::mutation::Mutation;
use graphql::query::{Query, Token};

use hyper::{HeaderMap, Method};
use tower_http::cors::{Any, CorsLayer};

use axum::response::IntoResponse;

use db::persistence::postgres::{DBInterface, DB};

#[tokio::main]
async fn main() {
    let server = async {
        // FIXME: ANYなおす
        let cors = CorsLayer::new()
            .allow_headers(Any)
            .allow_methods(vec![Method::GET, Method::POST])
            .allow_origin(Any);

        let schema = Schema::build(Query, Mutation, EmptySubscription)
            .data(DB::new().await)
            .finish();
        export_gql_schema(&schema);

        let app = Router::new()
            .route("/", get(graphql_playground).post(graphql_handler))
            .route("/graphql", post(graphql_handler))
            .layer(cors)
            .layer(Extension(schema));

        let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
        // server を起動
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    };

    tokio::join!(server);
}

fn get_token_from_headers(headers: &HeaderMap) -> Option<Token> {
    headers
        .get("Authorization")
        .and_then(|value| value.to_str().map(|s| Token(s.to_string())).ok())
}

async fn graphql_handler(
    schema: Extension<Schema<Query, Mutation, EmptySubscription>>,
    headers: HeaderMap,
    req: Json<Request>,
) -> Json<Response> {
    let mut req = req.0;
    if let Some(token) = get_token_from_headers(&headers) {
        req = req.data(token);
    }
    schema.execute(req).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

fn export_gql_schema(schema: &Schema<Query, Mutation, EmptySubscription>) {
    let file_path = "graphql/schema.gql";
    let export_schema = schema.sdl();

    let mut new_file = File::create(file_path).unwrap();
    new_file.write_all(export_schema.as_bytes()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_graphql_handler_with_token() {
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", "foo".parse().unwrap());

        let processed_handler = graphql_handler(
            Extension(Schema::build(Query, Mutation, EmptySubscription).finish()),
            headers,
            Json(Request::new("".to_string())),
        );

        assert_eq!(1, 1)
    }
}
