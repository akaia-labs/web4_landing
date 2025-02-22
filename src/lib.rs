mod common;
mod routes;

pub use common::*;
use {
	near_sdk::{
		borsh::{BorshDeserialize, BorshSerialize},
		json_types::Base64VecU8,
		near_bindgen,
		serde::{Deserialize, Serialize},
	},
	routes::Routes,
	std::collections::HashMap,
};

const ROUTES: Routes = Routes::new();

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {}

#[near_bindgen]
impl Contract {
	pub fn web4_get(&self, request: Web4Request) -> Web4Response {
		if let Some(route) = ROUTES.by_path.get(&request.path) {
			Web4Response::Body {
				content_type: "text/html; charset=UTF-8".to_owned(),
				body:         route.page.as_bytes().to_owned().into(),
			}
		} else {
			Web4Response::Body {
				content_type: "text/html; charset=UTF-8".to_owned(),
				body:         ROUTES.error_404.page.into_bytes().into(),
			}
		}
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		let result = 2 + 2;
		assert_eq!(result, 4);
	}
}
