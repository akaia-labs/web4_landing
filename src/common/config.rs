pub struct AppMetadata {
	pub name: &'static str,
	pub description: &'static str,
	pub favicon_url: &'static str,
}

pub const APP_METADATA: AppMetadata = AppMetadata {
	name: "Akaia",
	description: "An R&D e/acc community pushing the boundaries of self-sovereign and sustainable web.",
	favicon_url: "https://ipfs.near.social/ipfs/bafkreig5ck3loqtlizliqlfnzyv2qayqf3eqafnael57ur7ny7duon3ak4",
};
