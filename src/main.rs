use actix_cors::Cors;
use actix_web::{get, middleware::Logger, route, web, web::Data, App, HttpServer, Responder};
use actix_web_lab::respond::Html;
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use std::{
    cell::Cell,
    sync::atomic::{AtomicUsize, Ordering},
    sync::Mutex,
};
use surrealdb::{Datastore, Session};

mod app;
mod star_wars;

use self::app::{AppState, AppStateGlobal};
use self::star_wars::{QueryRoot, StarWars, StarWarsSchema};

static SERVER_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// GraphQL endpoint
#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(schema: web::Data<StarWarsSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

/// graphql playground UI
#[get("/playground")]
async fn graphql_playground() -> impl Responder {
    Html(playground_source(
        GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"),
    ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Note: web::Data created outside HttpServer::new closure
    let data = web::Data::new(AppStateGlobal {
        counter: Mutex::new(0),
        datastore: Datastore::new("tikv://127.0.0.1:2379").await.unwrap(),
        session: Session::for_kv().with_ns("test").with_db("test"),
    });

    let ds = Datastore::new("tikv://127.0.0.1:2379").await.unwrap();

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(StarWars::new())
        // TODO:
        // .data(Data::new(AppState {
        //     server_id: SERVER_COUNTER.fetch_add(1, Ordering::SeqCst),
        //     request_count: Cell::new(0),
        // }))
        // .data(data.clone())
        .data(ds)
        .finish();

    log::info!("starting HTTP server on port 8282");
    log::info!("graphql playground: http://localhost:8282/playground");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            // .app_data(Data::new(AppState {
            //     server_id: SERVER_COUNTER.fetch_add(1, Ordering::SeqCst),
            //     request_count: Cell::new(0),
            //     // filter,
            // }))
            // // global data: don't wrap it in data::new() it's already wrapped above
            // .app_data(data.clone())
            // services
            .service(graphql)
            .service(graphql_playground)
            .wrap(Cors::permissive())
            .wrap(Logger::default())
    })
    .workers(2)
    .bind(("0.0.0.0", 8282))?
    .run()
    .await
}
