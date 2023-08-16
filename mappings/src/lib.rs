#[cfg(feature = "clap")]
use clap::ValueEnum;
#[cfg(feature = "nih_plug")]
use nih_plug::prelude::*;

include!(concat!(env!("OUT_DIR"), "/libraries.rs"));

#[derive(Copy, Clone)]
pub struct Mapping {
	pub to_gm: [u8; 128],
	pub from_gm: [u8; 128],
}

impl Library {
	#[cfg(feature = "nih_plug")]
	pub fn values() -> impl Iterator<Item = (&'static str, Self)> {
		Self::variants()
			.iter()
			.enumerate()
			.map(|(i, name)| (*name, Self::from_index(i)))
	}
}

impl Mapping {
	pub fn to(&self, to: &Self, note: u8) -> u8 {
		to.from_gm[self.to_gm[note as usize] as usize]
	}
}
