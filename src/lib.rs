pub mod routes;

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

pub const ROUTES: Routes = Routes::new();

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {}

#[near_bindgen]
impl Contract {
	pub fn web4_get(&self, request: Web4Request) -> Web4Response {
		if request.path == ROUTES.index.path {
			Web4Response::Body {
				content_type: "text/html; charset=UTF-8".to_owned(),
				body:         ROUTES.index.body.as_bytes().to_owned().into(),
			}
		} else {
			Web4Response::Body {
				content_type: "text/html; charset=UTF-8".to_owned(),
				body:         ROUTES.error_404.body.into_bytes().into(),
			}
		}

		// } else {
		// 	Web4Response::Body {
		// 		content_type: "text/html; charset=UTF-8".to_owned(),

		// 		body: format!("<h1>Some page</h1><pre>{:#?}</pre>", request)
		// 			.into_bytes()
		// 			.into(),
		// 	}
		// }
	}
}

#[derive(Debug, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Web4Request {
	#[serde(rename = "accountId")]
	pub account_id: Option<String>,
	pub path:       String,
	#[serde(default)]
	pub params:     HashMap<String, String>,
	#[serde(default)]
	pub query:      HashMap<String, Vec<String>>,
	pub preloads:   Option<HashMap<String, Web4Response>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde", untagged)]
pub enum Web4Response {
	Body {
		#[serde(rename = "contentType")]
		content_type: String,
		body:         Base64VecU8,
	},

	BodyUrl {
		#[serde(rename = "bodyUrl")]
		body_url: String,
	},

	PreloadUrls {
		#[serde(rename = "preloadUrls")]
		preload_urls: Vec<String>,
	},
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		let result = 2 + 2;
		assert_eq!(result, 4);
	}
}
