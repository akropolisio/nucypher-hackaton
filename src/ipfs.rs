extern crate ipfs_api;

use std::io::prelude::*;
use crate::futures::Future;
use self::ipfs_api::IpfsClient;
use self::ipfs_api::response::{AddResponse, Error};


/// Creates future for publish `data` to IPFS network.
///
/// # Example
///
/// ```
/// use std::io::Cursor;
/// use actix_web::actix;
/// use ipfs_api::IpfsClient;

/// let client = IpfsClient::default();
/// let data = Cursor::new("Hello World!");
/// let req = pub_future_with(data, &client).map(|res| {
/// 	                                        println!("TODO: save ipfs hash: {}", res.hash);
/// 	                                       })
///                                         .map_err(|e| eprintln!("{}", e));

/// actix::run(|| {
/// 	req.then(|_| {
/// 		   actix::System::current().stop();
/// 		   Ok(())
/// 		  })
/// });
/// ```
pub fn pub_future<R: 'static + Read + Send>(data: R) -> (impl Future, IpfsClient) {
	let client = IpfsClient::default();
	(pub_future_with(data, &client), client)
}

pub fn pub_future_with<R: 'static + Read + Send>(data: R, client: &IpfsClient)
                                                 -> impl Future<Item = AddResponse, Error = Error> {
	let req = client.add(data);
	req
}
