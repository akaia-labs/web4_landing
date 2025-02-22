pub mod error_404;
pub mod page;

use {page::default as index, std::collections::HashMap};

pub struct Route {
	pub path: String,
	pub page: String,
}

pub struct Routes {
	pub by_path:   HashMap<String, Route>,
	pub error_404: Route,
}

impl Routes {
	pub fn new() -> Routes {
		Routes {
			by_path: HashMap::from([("/".to_string(), Route {
				path: "/".to_string(),
				page: index(),
			})]),

			error_404: Route {
				path: "/404".to_string(),
				page: error_404(),
			},
		}
	}
}
