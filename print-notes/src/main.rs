use std::{
	env,
	fs,
};

use anyhow::{
	anyhow,
	Result,
};
use midly::{
	MidiMessage,
	TrackEventKind,
};

fn main() -> Result<()> {
	for f in env::args().skip(1) {
		let data = fs::read(&f)?;
		let (_, tracks) = midly::parse(&data).map_err(|e| anyhow!("error parsing {f}: {e}"))?;
		let tracks = tracks
			.collect_tracks()
			.map_err(|e| anyhow!("failed to parse {f}: {e}"))?;
		for (i, track) in tracks.into_iter().enumerate() {
			println!("# track {i} from {f}:");
			for e in track {
				if let TrackEventKind::Midi { channel, message } = e.kind {
					match message {
						MidiMessage::NoteOn { key, .. } => println!("noteon {key} ch{channel}"),
						MidiMessage::NoteOff { key, .. } => println!("noteoff {key} ch{channel}"),
						MidiMessage::Aftertouch { key, .. } => {
							println!("aftertouch {key} ch{channel}")
						}
						_ => (),
					}
				}
			}
		}
	}

	Ok(())
}
