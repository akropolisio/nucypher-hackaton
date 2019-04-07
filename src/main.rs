//! Diesel does not support tokio, so we have to run it in separate threads.
//! Actix supports sync actors by default, so we going to create sync actor
//! that use diesel. Technically sync actors are worker style actors, multiple
//! of them can run in parallel and process messages from same queue.
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate futures;
extern crate r2d2;
extern crate uuid;
extern crate bytes;
// extern crate json;

#[macro_use]
extern crate juniper;


use juniper::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use bytes::BytesMut;
use actix::prelude::*;
use actix_web::{http, middleware, server, App, AsyncResponder, FutureResponse, HttpResponse, Path, Error, HttpRequest, State, HttpMessage, error, Json};

use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use futures::{future, Future, Stream};

mod db;
mod models;
mod schema;
mod api;
mod ipfs;

use api::*;
use db::{CreateUser, AcceptUser, GetUser, DbExecutor};


const MAX_SIZE: usize = 4096; // mb. max payload size is 256k = 262_144 ?


/// State with DbExecutor address
struct AppState<'a> {
	db: Addr<DbExecutor>,
	gql: std::sync::Arc<juniper::RootNode<'a, gql::QueryRoot, gql::MutationRoot>>,
}


fn get_state((name, state): (Path<String>, State<AppState>)) -> FutureResponse<HttpResponse> {
	// send async `GetUser` message to a `DbExecutor`
	state.db
	     .send(GetUser { contact: name.into_inner() })
	     .from_err()
	     .and_then(|res| {
		     match res {
			     Ok(user) => Ok(HttpResponse::Ok().json(user)),
		       Err(_) => Ok(HttpResponse::InternalServerError().into()),
		     }
		    })
	     .responder()
}

fn reg_init((item, state): (Json<Reg>, State<AppState>)) -> impl Future<Item = HttpResponse, Error = Error> {
	// TODO: validate contact?
	state.db
	     .send(CreateUser { contact: item.into_inner().contact })
	     .from_err()
	     .and_then(|res| {
		     match res {
			     Ok(user) => Ok(HttpResponse::Ok().json(user)),
		       Err(_) => Ok(HttpResponse::InternalServerError().into()),
		     }
		    })
}

fn reg_code((item, state): (Json<RegCode>, State<AppState>)) -> impl Future<Item = HttpResponse, Error = Error> {
	let data = item.into_inner();
	state.db
	     .send(AcceptUser { contact: data.contact,
	                        code: data.code })
	     .from_err()
	     .and_then(|res| {
		     match res {
			     Ok(user) => Ok(HttpResponse::Ok().json(user)),
		       Err(_) => Ok(HttpResponse::InternalServerError().into()),
		     }
		    })
}

fn main() {
	std::env::set_var("RUST_LOG", "actix_web=info");
	env_logger::init();

	// TODO: get from .ENV:
	let listen_ip = "127.0.0.1:8080";

	let sys = actix::System::new("nu-contact-crossshare");

	// Start 3 db executor actors
	let manager = ConnectionManager::<SqliteConnection>::new(// TODO: get from .ENV:
	                                                         "test.db");
	let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");

	let addr = SyncArbiter::start(3, move || DbExecutor(pool.clone()));

	// Create Juniper schema
	let schema = std::sync::Arc::new(gql::create_schema());
	// use actix_web::client as web;
	// Start http server
	server::new(move || {
		App::with_state(AppState { db: addr.clone(), gql:schema.clone() })
		      // enable logger
		      .middleware(middleware::Logger::default())

		      // graphql:
		      .resource("/graphql", |r| r.method(http::Method::POST).with_async(graphql))
		      .resource("/graphiql", |r| r.method(http::Method::GET).with(graphiql))

		      // raw:
		      .resource("/state/{contact}", |r| r.method(http::Method::GET).with(get_state))
		      .resource("/register/init", |r| {
		          r.method(http::Method::POST)
		              .with_async_config(reg_init, |(json_cfg, )| {
		                  json_cfg.0.limit(MAX_SIZE);
		              })
		      })
		      .resource("/register/code", |r| {
		          r.method(http::Method::POST)
		              .with_async_config(reg_code, |(json_cfg, )| {
		                  json_cfg.0.limit(MAX_SIZE);
		              })
		      })
	}).bind(listen_ip)
	.unwrap()
	.start();
	println!("Started http server: {}", listen_ip);
	let _ = sys.run();
}


fn graphql((data, state): (Json<GraphQLRequest>, State<AppState<'static>>))
           -> impl Future<Item = HttpResponse, Error = Error> {
	futures::lazy(move || {
		let res = data.execute(&state.gql, &());
		if res.is_ok() {
			Ok(HttpResponse::Ok().json(res))
		} else {
			Ok(HttpResponse::NotFound().body(serde_json::to_string(&res)?).into())
		}
	})
}

fn graphiql<'t>((..): (Json<GraphQLRequest>, State<AppState<'t>>)) -> HttpResponse {
	let html = graphiql_source("http://127.0.0.1:8080/graphql");
	HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html)
}
