include!(concat!(env!("OUT_DIR"), "/libraries.rs"));

impl Library {
	pub fn values() -> impl Iterator<Item = (&'static str, Self)> {
		Self::variants()
			.iter()
			.enumerate()
			.map(|(i, name)| (*name, Self::from_index(i)))
	}
}