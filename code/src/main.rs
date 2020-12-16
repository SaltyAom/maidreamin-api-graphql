#[macro_use]
extern crate lazy_static;

mod route;
mod data;
mod schema;

use std::io;
use std::sync::{ Arc, RwLock };
use hashbrown::HashMap;

use actix_web::{http, App, HttpServer, middleware, web::{ Data }};
use actix_cors::Cors;

use schema::{ create_schema, Cache };

use route::route_service;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let schema = Arc::new(create_schema());
    let graph_context = Data::new(Cache {
        cache: RwLock::new(HashMap::new())
    });

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .send_wildcard()
            .allowed_methods(vec!["POST"])
            .allowed_headers(vec![http::header::CONTENT_TYPE, http::header::ACCEPT])
            .max_age(86400);

        App::new()
            .wrap(
                middleware::Compress::default()
            )
            .wrap(cors)
            .data(schema.clone())
            .app_data(graph_context.clone())
            .configure(route_service)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
