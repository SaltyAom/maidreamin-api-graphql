use std::sync::Arc;

use actix_web::{Error, HttpResponse, web, get, post};

use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

use crate::schema::{ Schema, Cache };

#[get("/graphiql")]
pub async fn graphiql() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            graphiql_source("http://127.0.0.1:8080/graphql")
        )
}

#[post("/graphql")]
pub async fn graphql(
    data: web::Data<Arc<Schema>>,
    graph_context: web::Data<Cache>,
    request: web::Json<GraphQLRequest>
) -> Result<HttpResponse, Error> {
    let data = web::block(move || {
        let res = request.execute(&data, &graph_context);

        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(data))    
}
