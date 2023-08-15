#![allow(clippy::arc_with_non_send_sync)]

mod channel;
mod mappings;

use std::sync::Arc;

use nih_plug::prelude::*;
#[cfg(feature = "gui")]
use nih_plug_egui::{
	create_egui_editor,
	egui,
	EguiState,
};

use self::{
	channel::Channel,
	mappings::Library,
};

nih_export_clap!(DrumMapper);
nih_export_vst3!(DrumMapper);

#[derive(Params)]
struct Parameters {
	#[cfg_attr(feature = "gui", persist = "editor-state")]
	#[cfg(feature = "gui")]
	gui_state: Arc<EguiState>,
	#[id = "from"]
	from: EnumParam<Library>,
	#[id = "to"]
	to: EnumParam<Library>,
	#[id = "channel"]
	ch: EnumParam<Channel>,
}

impl Default for Parameters {
	fn default() -> Self {
		Self {
			#[cfg(feature = "gui")]
			gui_state: EguiState::from_size(750, 250),
			from: EnumParam::new("from", Library::DEFAULT),
			to: EnumParam::new("to", Library::DEFAULT),
			ch: EnumParam::new("channel", Channel::All),
		}
	}
}

#[derive(Default)]
struct DrumMapper {
	params: Arc<Parameters>,
}

impl Vst3Plugin for DrumMapper {
	const VST3_CLASS_ID: [u8; 16] = *b"ins__drum_mapper";
	const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
		Vst3SubCategory::Drum,
		Vst3SubCategory::Instrument,
		Vst3SubCategory::Tools,
	];
}

impl ClapPlugin for DrumMapper {
	const CLAP_DESCRIPTION: Option<&'static str> =
		Some("Maps drum tracks from/to popular drum libraries.");
	const CLAP_FEATURES: &'static [ClapFeature] = &[
		ClapFeature::Instrument,
		ClapFeature::Drum,
		ClapFeature::Utility,
	];
	const CLAP_ID: &'static str = "insomnia.drum-mapper";
	const CLAP_MANUAL_URL: Option<&'static str> = None;
	const CLAP_SUPPORT_URL: Option<&'static str> = None;
}

impl Plugin for DrumMapper {
	type BackgroundTask = ();
	type SysExMessage = ();

	const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[];
	const EMAIL: &'static str = "";
	const MIDI_INPUT: MidiConfig = MidiConfig::Basic;
	const MIDI_OUTPUT: MidiConfig = MidiConfig::Basic;
	const NAME: &'static str = "Drum Mapper";
	const SAMPLE_ACCURATE_AUTOMATION: bool = true;
	const URL: &'static str = "";
	const VENDOR: &'static str = "Insomnia";
	const VERSION: &'static str = env!("CARGO_PKG_VERSION");

	fn params(&self) -> Arc<dyn Params> {
		Arc::clone(&self.params) as Arc<dyn Params>
	}

	fn process(
		&mut self,
		_buffer: &mut Buffer,
		_aux: &mut AuxiliaryBuffers,
		context: &mut impl ProcessContext<Self>,
	) -> ProcessStatus {
		while let Some(event) = context.next_event() {
			let ch = self.params.ch.value();
			let from = self.params.from.value();
			let to = self.params.to.value();

			match event {
				NoteEvent::NoteOn {
					timing,
					voice_id,
					channel,
					note,
					velocity,
				} if ch == Channel::All || channel == ch as u8 => context.send_event(NoteEvent::NoteOn {
					timing,
					voice_id,
					channel,
					note: from.to(to, note),
					velocity,
				}),
				NoteEvent::NoteOff {
					timing,
					voice_id,
					channel,
					note,
					velocity,
				} if ch == Channel::All || channel == ch as u8 => context.send_event(NoteEvent::NoteOff {
					timing,
					voice_id,
					channel,
					note: from.to(to, note),
					velocity,
				}),
				other => context.send_event(other),
			}
		}

		ProcessStatus::Normal
	}

	#[cfg(feature = "gui")]
	fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
		let params = Arc::clone(&self.params);

		create_egui_editor(
			Arc::clone(&self.params.gui_state),
			(),
			|_, _| {},
			move |egui_ctx, setter, _state| {
				// egui_ctx.enable_accesskit();
				// egui_ctx.accesskit_node_builder(egui::accesskit_root_id())
				let mut ch = params.ch.value();
				let mut from = params.from.value();
				let mut to = params.to.value();

				egui::CentralPanel::default().show(egui_ctx, |ui| {
					// From Selection
					ui.horizontal(|ui| {
						ui.label("From");
						for (name, value) in Library::values() {
							ui.radio_value(&mut from, value, name);
						}
					});

					// To selection
					ui.horizontal(|ui| {
						ui.label("To");
						for (name, value) in Library::values() {
							ui.radio_value(&mut to, value, name);
						}
					});
					// Channel selection
					ui.horizontal(|ui| {
						ui.label("Channel");
						for (name, value) in Channel::VALUES {
							ui.radio_value(&mut ch, value, name);
						}
					});

					// Update parameters
					macro_rules! update {
						[$modified:expr, $orig:expr] => {
							if $modified != $orig.value() {
						setter.begin_set_parameter(&$orig);
						setter.set_parameter(&$orig, $modified);
						setter.end_set_parameter(&$orig);
					}
						};
					}

					update!(from, params.from);
					update!(to, params.to);
					update!(ch, params.ch);
				});
			},
		)
	}
}
