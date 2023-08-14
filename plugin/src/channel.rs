use nih_plug::prelude::*;

#[derive(Default, Eq, PartialEq, Copy, Clone, Debug, Enum)]
pub enum Channel {
	#[default]
	#[name = "all"]
	#[id = "all"]
	All,
	#[id = "c1"]
	#[name = "1"]
	C1,
	#[id = "c2"]
	#[name = "2"]
	C2,
	#[id = "c3"]
	#[name = "3"]
	C3,
	#[id = "c4"]
	#[name = "4"]
	C4,
	#[id = "c5"]
	#[name = "5"]
	C5,
	#[id = "c6"]
	#[name = "6"]
	C6,
	#[id = "c7"]
	#[name = "7"]
	C7,
	#[id = "c8"]
	#[name = "8"]
	C8,
	#[id = "c9"]
	#[name = "9"]
	C9,
	#[id = "c10"]
	#[name = "10"]
	C10,
	#[id = "c11"]
	#[name = "11"]
	C11,
	#[id = "c12"]
	#[name = "12"]
	C12,
	#[id = "c13"]
	#[name = "13"]
	C13,
	#[id = "c14"]
	#[name = "14"]
	C14,
	#[id = "c15"]
	#[name = "15"]
	C15,
	#[id = "c16"]
	#[name = "16"]
	C16,
}

#[cfg(feature = "gui")]
impl Channel {
	pub const VALUES: [(&'static str, Self); 17] = [
		("All", Self::All),
		("1", Self::C1),
		("2", Self::C2),
		("3", Self::C3),
		("4", Self::C4),
		("5", Self::C5),
		("6", Self::C6),
		("7", Self::C7),
		("8", Self::C8),
		("9", Self::C9),
		("10", Self::C10),
		("11", Self::C11),
		("12", Self::C12),
		("13", Self::C13),
		("14", Self::C14),
		("15", Self::C15),
		("16", Self::C16),
	];
}
