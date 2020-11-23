use std::sync::Arc;

use actix_web::{Error, HttpResponse, web, get, post};

use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

use crate::schema::{ Schema, Cache };

const INDEX_RESPONSE: &'static str = r#"
    <!DOCTYPE HTML>
    <html lang="en">
        <head>
            <title>Dreamin GraphQL</title>
            <meta name="viewport" content="width=device-width, initial-scale=1">
        </head>
        <body>
            <h1>Dreamin GraphQL</h1>
            <ul>
                <li>
                    <a href="/graphiql">/graphiql (GET) - GraphQL Playground</a>
                </li>
                <li>
                    <a href="/graphql">/graphql (POST) - Request Endpoint</a>
                </li>
            </ul>
        </body>
    </html>
"#;

#[get("/")]
pub async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(INDEX_RESPONSE)
}

#[get("/graphiql")]
pub async fn graphiql() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            graphiql_source("/graphql", None)
        )
}

#[post("/graphql")]
pub async fn graphql(
    data: web::Data<Arc<Schema>>,
    graph_context: web::Data<Cache>,
    request: web::Json<GraphQLRequest>
) -> Result<HttpResponse, Error> {
    let res = request.execute(&data, &graph_context).await;
    
    Ok(HttpResponse::Ok()
        .json(res)
    )
}
