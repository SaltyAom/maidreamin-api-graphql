#[macro_use]
extern crate lazy_static;

mod route;
mod data;
mod schema;

use std::io;
use std::sync::Arc;

use actix_web::{App, HttpServer, middleware};

use schema::create_schema;

use route::{ graphiql, graphql };

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let schema = Arc::new(create_schema());

    HttpServer::new(move || {
        App::new()
            .wrap(
                middleware::Compress::default()
            )
            .data(schema.clone())
            .service(graphql)
            .service(graphiql)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
