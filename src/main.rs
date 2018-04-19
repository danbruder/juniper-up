extern crate iron;
extern crate juniper;
extern crate juniper_iron;
extern crate logger;
extern crate mount;
extern crate serde;

use std::env;

use mount::Mount;
use logger::Logger;
use iron::prelude::*;
use juniper::EmptyMutation;
use juniper_iron::{GraphQLHandler, GraphiQLHandler};
use juniper::tests::model::Database;

fn context_factory(_: &mut Request) -> Database {
    Database::new()
}

fn main() {
    let mut mount = Mount::new();

    let graphql_endpoint = GraphQLHandler::new(
        context_factory,
        Database::new(),
        EmptyMutation::<Database>::new(),
    );
    let graphiql_endpoint = GraphiQLHandler::new("/staging/graphql");

    mount.mount("/", graphiql_endpoint);
    mount.mount("/graphql", graphql_endpoint);

    let (logger_before, logger_after) = Logger::new(None);

    let mut chain = Chain::new(mount);
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    let port = env::var("PORT").unwrap();
    let host = format!("localhost:{}", port);
    println!("GraphQL server started on {}", host);
    Iron::new(chain).http(host).unwrap();
}
