#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use]
extern crate diesel;

extern crate dotenv;

use rocket::{response::content, State};
use std::fs::File;
use std::io::Write;

mod graphql;
mod models;
mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use std::sync::Mutex;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[rocket::get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[rocket::get("/graphql?<request>")]
fn get_graphql_handler(
    context: State<graphql::Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<graphql::Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

#[rocket::post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: State<graphql::Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<graphql::Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

fn main() {
    let connection = establish_connection();
    let ctx = graphql::Context {
        database: Mutex::new(connection),
    };
    let schema = graphql::Schema::new(graphql::Query, graphql::Mutation);

    let (res, _errors) =
        juniper::introspect(&schema, &ctx, juniper::IntrospectionFormat::default()).unwrap();

    let json_result = serde_json::to_string_pretty(&res);
    assert!(json_result.is_ok());
    let file_result = File::create("schema.json");
    assert!(file_result.is_ok());
    file_result
        .unwrap()
        .write_all(json_result.unwrap().as_bytes())
        .unwrap();

    rocket::ignite()
        .manage(ctx)
        .manage(schema)
        .mount(
            "/",
            rocket::routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
        .launch();
}
