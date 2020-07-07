#[macro_use]
extern crate lazy_static;

mod data;
mod schema;

use std::io;
use std::sync::Arc;

use actix_web::{web, App, Error, HttpResponse, HttpServer, middleware};

use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

use schema::{create_schema, Schema};

async fn graphiql() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            graphiql_source("http://127.0.0.1:8080/graphql")
        )
}

async fn graphql(
    data: web::Data<Arc<Schema>>,
    request: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let data = web::block(move || {
        let res = request.execute(&data, &());

        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(data))
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let schema = std::sync::Arc::new(create_schema());

    HttpServer::new(move || {
        App::new()
            .wrap(
                middleware::Compress::default()
            )
            .data(schema.clone())
            .service(web::resource("/graphql").route(web::post().to(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
