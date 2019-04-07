//! Db executor actor
use ::actix::prelude::*;
use actix_web::*;
use diesel;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use uuid;

use crate::models;
use crate::schema;

/// This is db executor actor. We are going to run 3 of them in parallel.
pub struct DbExecutor(pub Pool<ConnectionManager<SqliteConnection>>);

impl Actor for DbExecutor {
	type Context = SyncContext<Self>;
}


// get

pub struct GetUser {
	pub contact: String,
}

impl Message for GetUser {
	type Result = Result<models::User, Error>;
}

impl Handler<GetUser> for DbExecutor {
	type Result = Result<models::User, Error>;

	fn handle(&mut self, msg: GetUser, _: &mut Self::Context) -> Self::Result {
		use self::schema::users::dsl::*;

		let conn: &SqliteConnection = &self.0.get().unwrap();

		let mut items = users.filter(contact.eq(&msg.contact))
		                     .load::<models::User>(conn)
		                     .map_err(|_| error::ErrorInternalServerError("Error loading account"))?;

		Ok(items.pop().unwrap())
	}
}


// reg

pub struct CreateUser {
	pub contact: String,
}

impl Message for CreateUser {
	type Result = Result<models::User, Error>;
}

impl Handler<CreateUser> for DbExecutor {
	type Result = Result<models::User, Error>;

	fn handle(&mut self, msg: CreateUser, _: &mut Self::Context) -> Self::Result {
		use self::schema::users::dsl::*;

		let uuid = format!("{}", uuid::Uuid::new_v4());
		let new_user = models::NewUser { id: &uuid,
		                                 contact: &msg.contact,
		                                 code: Some("004200"),
		                                 address: None,
		                                 token: None };

		let conn: &SqliteConnection = &self.0.get().unwrap();

		diesel::insert_into(users).values(&new_user)
		                          .execute(conn)
		                          .map_err(|_| error::ErrorInternalServerError("Error inserting account"))?;

		let mut items = users.filter(id.eq(&uuid))
		                     .load::<models::User>(conn)
		                     .map_err(|_| error::ErrorInternalServerError("Error loading account"))?;

		Ok(items.pop().unwrap())
	}
}


// reg + code

pub struct AcceptUser {
	pub contact: String,
	pub code: String,
}

impl Message for AcceptUser {
	type Result = Result<models::User, Error>;
}

impl Handler<AcceptUser> for DbExecutor {
	type Result = Result<models::User, Error>;

	fn handle(&mut self, msg: AcceptUser, _: &mut Self::Context) -> Self::Result {
		use self::schema::users::dsl::*;

		let conn: &SqliteConnection = &self.0.get().unwrap();

		let mut items = users.filter(contact.eq(&msg.contact))
		                     .load::<models::User>(conn)
		                     .map_err(|_| error::ErrorInternalServerError("Error loading account"))?;

		let mut item = items.pop().unwrap();
		if !item.wait_code() {
			return Err(error::ErrorMethodNotAllowed("No code required"));
		}
		if &msg.code != &item.code.expect("No code required") {
			// ? error::ErrorMethodNotAllowed
			Err(error::ErrorNotAcceptable("Invalid code. Try again."))
		} else {
			item.code = None;
			// diesel::update(&item).set(id.eq(&item.id))
			diesel::update(&item).set(code.eq(&item.code))
			                     .execute(conn)
			                     .map_err(|_| error::ErrorInternalServerError("Error updating account"))?;
			Ok(item)
		}
	}
}
