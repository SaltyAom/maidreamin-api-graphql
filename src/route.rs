use std::sync::Arc;

use actix_web::{Error, HttpResponse, web, get, post, Result, web::ServiceConfig};
use actix_files::NamedFile;

use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

use crate::schema::{ Schema, Cache };

#[get("/")]
pub async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("static/index.html")?)
}

#[get("/vanilla.jpg")]
pub async fn cover() -> Result<NamedFile> {
    Ok(NamedFile::open("static/vanilla.jpg")?)
}

#[get("/icon.png")]
pub async fn icon() -> Result<NamedFile> {
    Ok(NamedFile::open("static/icon.png")?)
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

pub fn route_service(config: &mut ServiceConfig) {
    config
        .service(index)
        .service(icon)
        .service(cover)
        .service(graphiql)
        .service(graphql);
}