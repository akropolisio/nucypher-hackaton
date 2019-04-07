#[derive(Debug, Serialize, Deserialize)]
pub struct MyUser {
	pub contact: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reg {
	pub contact: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegCode {
	pub code: String,
	pub contact: String,
}


pub mod gql {
	// graphql:

	use juniper::FieldResult;
	use juniper::RootNode;


	// --
	// #[derive(GraphQLEnum)]
	// enum UserState { A, B, }
	// #[derive(GraphQLEnum)]
	// enum ShareState { A, B, }

	#[derive(GraphQLObject)]
	#[graphql(description = "todo descr.")]
	struct User {
		id: String,
		contact: String,
	}

	#[derive(GraphQLInputObject)]
	#[graphql(description = "todo descr.")]
	struct NewUser {
		contact: String,
	}
	// --

	pub struct QueryRoot;

	graphql_object!(QueryRoot: () |&self| {
    field user(&executor, id: String) -> FieldResult<User> {
        Ok(User{
			  id: "42".to_owned(),
			  contact: "N/A".to_owned()
        })
    }
});

	pub struct MutationRoot;

	graphql_object!(MutationRoot: () |&self| {
    field createUser(&executor, new_user: NewUser) -> FieldResult<User> {
        Ok(User{
            id: "42".to_owned(),
				contact: new_user.contact
        })
    }
});

	pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

	pub fn create_schema() -> Schema { Schema::new(QueryRoot {}, MutationRoot {}) }

}
