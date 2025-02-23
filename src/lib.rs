mod common;
mod routes;

pub use common::*;
use {
	near_sdk::near,
	routes::{PathNavigation, RouteNavigator},
};

#[near(contract_state)]
#[derive(Default)]
pub struct Contract {}

#[near]
impl Contract {
	pub fn web4_get(&self, request: Web4Request) -> Web4Response {
		let navigator = RouteNavigator::new();

		Web4Response::Body {
			content_type: "text/html; charset=UTF-8".to_owned(),
			body: navigator
				.get_route_by_path(&request.path)
				.page
				.as_bytes()
				.to_owned()
				.into(),
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
