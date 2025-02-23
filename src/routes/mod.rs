mod layout;

use std::collections::HashMap;

pub struct Route {
	pub path: String,
	pub page: String,
}

pub struct Routes {
	pub by_path: HashMap<String, Route>,
	pub error_404: Route,
}

impl Routes {
	pub fn new() -> Routes {
		Routes {
			by_path: HashMap::from([("/".to_string(), Route {
				path: "/".to_string(),
				page: layout::view(include_str!("index.html").to_string()),
			})]),

			error_404: Route {
				path: "/404".to_string(),
				page: layout::view(include_str!("404.html").to_string()),
			},
		}
	}
}
