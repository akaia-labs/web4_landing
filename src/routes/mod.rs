mod layout;

use std::collections::HashMap;

use crate::{APP_METADATA, PageProps};

pub struct Route {
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
				page: layout::view(PageProps {
					title: APP_METADATA.name.to_string(),
					subtitle: "Demo".to_string(),
					description: APP_METADATA.description.to_string(),
					children: include_str!("index.html").to_string(),
				}),
			})]),

			error_404: Route {
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
