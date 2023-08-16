use std::{
	collections::BTreeMap,
	env,
	fmt::Write,
	fs,
	path::{
		Path,
		PathBuf,
	},
};

struct Mapping {
	name: String,
	variant: String,
	ident: String,
	from_gm: Vec<u8>,
	to_gm: Vec<u8>,
}

impl Mapping {
	fn parse(p: &Path) -> Self {
		let Ok(data) = fs::read_to_string(p) else {
			panic!("failed to read file {}", p.display());
		};

		let mut map = BTreeMap::new();
		for (i, s) in data
			.lines()
			.enumerate()
			.filter(|t| !t.1.trim().is_empty() && !t.1.trim_start().starts_with('#'))
		{
			let Some((gm, to)) = s.split_once("->") else {
				panic!(
					"the file {} contains an invalid line at line {}: {}",
					p.display(),
					i,
					s
				);
			};

			let Ok(gm) = gm.trim().parse::<u8>() else {
				panic!(
					"the file {} contains an invalid line at line {}: {}",
					p.display(),
					i,
					s
				);
			};
			let Ok(to) = to.trim().parse::<u8>() else {
				panic!(
					"the file {} contains an invalid line at line {}: {}",
					p.display(),
					i,
					s
				);
			};
			if gm > 127 || to > 127 {
				panic!(
					"the file {} contains mappings greater than 127 at line {}",
					p.display(),
					i
				);
			}
			map.insert(gm, to);
		}

		// This should be impossible
		if map.len() > 128 {
			panic!("the file {} contains more than 128 mappings", p.display());
		}

		let name = p.file_stem().unwrap().to_str().unwrap().to_string();
		let variant = name.replace(' ', "");
		let ident = name.replace(' ', "_").to_uppercase();

		let from_gm = (0..=127)
			.map(|gm| map.get(&gm).copied().unwrap_or(gm))
			.collect::<Vec<_>>();
		let mut to_gm = Vec::from_iter(0_u8..=127);
		for (gm, to) in &map {
			to_gm[*to as usize] = *gm;
		}
		Self {
			name,
			variant,
			ident,
			from_gm,
			to_gm,
		}
	}
}

fn to_rust(mappings: &[Mapping]) -> String {
	let is_clap = cfg!(feature = "clap");
	let is_nih = cfg!(feature = "nih_plug");

	let mut en_header = String::with_capacity(64);

	if is_clap {
		en_header += "#[derive(ValueEnum)]\n";
	}
	if is_nih {
		en_header += "#[derive(Enum)]\n";
	}

	let en = format!(
		"#[derive(Copy, Clone, Eq, PartialEq, Debug)]\n{en_header}pub enum Library {{\n{variants}\n}}",
		variants = mappings
			.iter()
			.fold(String::with_capacity(1024), |mut buf, m| {
				if is_nih {
				let _ = writeln!(
					buf,
					"#[name = {name:?}] #[id = {name:?}]",
					name = m.name,
				);
				}
				let _ = writeln!(buf, "{},", m.variant);
				buf
			})
	);

	let impls = format!(
		"impl Library {{
	pub const DEFAULT: Self = Self::{default};
	pub fn get_mapping(self) -> &'static Mapping {{
		match self {{
			{match_body}
		}}
	}}

	pub fn to(self, to: Self, note: u8) -> u8 {{
		self.get_mapping().to(to.get_mapping(), note)
	}}
}}
",
		default = &mappings[0].variant,
		match_body = mappings
			.iter()
			.fold(String::with_capacity(1024), |mut buf, m| {
				let _ = writeln!(buf, "Self::{} => &{},", m.variant, m.ident);
				buf
			})
	);

	let idents =  mappings.iter().fold(
		String::with_capacity(8 * 1024),
		|mut buf, m| {
			let _ = writeln!(buf, "static {ident}: Mapping = Mapping {{\nfrom_gm: {from_gm:?},\nto_gm: {to_gm:?},\n}};",
			ident = m.ident,
			to_gm = m.to_gm,
			from_gm = m.from_gm,
			);
			buf
		}
	);

	format!(
		"{en}
{impls}
{idents}\n"
	)
}

fn main() {
	println!("cargo:rerun-if-changed=../drums");
	let files = fs::read_dir("../drums")
		.expect("failed to read directory `drums`")
		.flatten()
		.filter_map(|e| {
			let p = e.path();
			if p.extension()
				.map_or(false, |ext| ext.eq_ignore_ascii_case("txt"))
			{
				Some(p)
			} else {
				None
			}
		});

	let mut mappings = Vec::with_capacity(64);
	mappings.push(Mapping {
		name: "General MIDI".into(),
		variant: "GeneralMidi".into(),
		ident: "GENERAL_MIDI".into(),
		from_gm: (0..=127).collect(),
		to_gm: (0..=127).collect(),
	});

	mappings.extend(files.map(|p| Mapping::parse(&p)));

	let mut out = PathBuf::from(env::var("OUT_DIR").unwrap());
	out.push("libraries.rs");

	if let Err(e) = fs::write(&out, to_rust(&mappings).as_bytes()) {
		panic!(
			"failed to write to the output file {}: {}",
			out.display(),
			e
		);
	}
}
