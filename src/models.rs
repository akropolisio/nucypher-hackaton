use super::schema::users;

#[derive(Serialize, Queryable, Identifiable)]
pub struct User {
	pub id: String,
	pub contact: String,
	pub address: Option<String>,
	pub code: Option<String>,
	pub token: Option<String>,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
	pub id: &'a str,
	pub contact: &'a str,
	pub address: Option<&'a str>,
	pub code: Option<&'a str>,
	pub token: Option<&'a str>,
}

impl User {
	pub fn wait_code(&self) -> bool { self.code.is_some() }
	pub fn accepted(&self) -> bool { self.code.is_none() }
}

impl<'a> NewUser<'a> {
	pub fn wait_code(&self) -> bool { self.code.is_some() }
	pub fn accepted(&self) -> bool { self.code.is_none() }
}


//
use super::schema::shares;

#[derive(Serialize, Queryable, Identifiable)]
pub struct Share {
	pub id: String,
	pub name: String,
	pub ipfs: Option<String>,
	pub nu: Option<String>,
	pub nupk: Option<String>,
	pub nurk: Option<String>,
}


#[derive(Insertable)]
#[table_name = "shares"]
pub struct NewShare<'a> {
	pub id: &'a str,
	pub name: &'a str,
	pub ipfs: Option<&'a str>,
	pub nu: Option<&'a str>,
	pub nupk: Option<&'a str>,
	pub nurk: Option<&'a str>,
}
