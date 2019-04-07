table! {
		users (id) {
				id -> Text,
				contact -> Text,
				address -> Nullable<Text>,
				code -> Nullable<Text>,
				token -> Nullable<Text>,
		}
}

table! {
		shares (id) {
				id -> Text,
				name -> Text,
				ipfs -> Nullable<Text>,
				nu -> Nullable<Text>,
				nupk -> Nullable<Text>,
				nurk -> Nullable<Text>,
		}
}
