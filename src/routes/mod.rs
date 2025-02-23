pub mod layout;

use crate::{APP_METADATA, PageProps};

pub struct Route {
	pub page: String,
}

pub struct RouteNavigator {}

pub trait PathNavigation {
	fn get_route_by_path(&self, path: &str) -> Route;
}

impl PathNavigation for RouteNavigator {
	fn get_route_by_path(&self, path: &str) -> Route {
		match path {
			| "/" => Route {
				page: layout::view(PageProps {
					title: APP_METADATA.name.to_string(),
					subtitle: "Demo".to_string(),
					description: APP_METADATA.description.to_string(),
					children: include_str!("index.html").to_string(),
				}),
			},

			| _ => Route {
				page: layout::view(PageProps {
					title: APP_METADATA.name.to_string(),
					subtitle: "Not Found".to_string(),
					description: "No content corresponds to the requested URL.".to_string(),
					children: include_str!("404.html").to_string(),
				}),
			},
		}
	}
}

impl RouteNavigator {
	pub fn new() -> Self {
		Self {}
	}
}
