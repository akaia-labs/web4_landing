pub mod error_404;
pub mod page;

use page::default as index;

pub struct Route {
	pub path: String,
	pub body: String,
}

pub struct Routes {
	pub index:     Route,
	pub error_404: Route,
}

impl Routes {
	pub fn new() -> Routes {
		Routes {
			index: Route {
				path: "/".to_string(),
				body: index(),
			},

			error_404: Route {
				path: "/404".to_string(),
				body: error_404(),
			},
		}
	}
}
