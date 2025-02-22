use {
	near_sdk::{
		borsh::{BorshDeserialize, BorshSerialize},
		json_types::Base64VecU8,
		serde::{Deserialize, Serialize},
	},
	std::collections::HashMap,
};

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
