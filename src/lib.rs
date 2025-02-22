mod common;
mod routes;

pub use common::*;
use {near_sdk::near, routes::Routes};

#[near(contract_state)]
#[derive(Default)]
pub struct Contract {}

#[near]
impl Contract {
	pub fn web4_get(&self, request: Web4Request) -> Web4Response {
		let routes = Routes::new();

		if let Some(route) = routes.by_path.get(&request.path) {
			Web4Response::Body {
				content_type: "text/html; charset=UTF-8".to_owned(),
				body:         route.page.as_bytes().to_owned().into(),
			}
		} else {
			Web4Response::Body {
				content_type: "text/html; charset=UTF-8".to_owned(),
				body:         routes.error_404.page.into_bytes().into(),
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
