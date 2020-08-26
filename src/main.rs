#[macro_use]
extern crate lazy_static;

mod route;
mod data;
mod schema;

use std::io;
use std::sync::{ Arc, Mutex };
use std::collections::HashMap;

use actix_web::{App, HttpServer, middleware, web::{ Data }};

use schema::{ create_schema, Cache };

use route::{ graphiql, graphql };

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let schema = Arc::new(create_schema());
    let graph_context = Data::new(Cache {
        cache: Mutex::new(HashMap::new())
    });

    HttpServer::new(move || {
        App::new()
            .wrap(
                middleware::Compress::default()
            )
            .data(schema.clone())
            .app_data(graph_context.clone())
            .service(graphql)
            .service(graphiql)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
