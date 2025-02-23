use {
	near_sdk::{json_types::Base64VecU8, near},
	std::collections::HashMap,
};

#[near(serializers = [json])]
#[derive(Debug)]
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

#[near(serializers = [json])]
#[derive(Debug)]
#[serde(untagged)]
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
