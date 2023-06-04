mod gui;
mod curve;

use std::sync::Arc;

use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;

/// The parameters for the Functor plugin.
#[derive(Params)]
pub struct FunctorParams {
    /// The editor state.
    #[persist = "editor-state"]
    editor_state: Arc<ViziaState>,

    /// The selected beat or timestretch pattern.
    #[id = "beat"]
    pub beat: IntParam,

    /// The selected volume or gating pattern.
    #[id = "vol"]
    pub vol: IntParam,
}

impl Default for FunctorParams {
    fn default() -> Self {
        Self {
            editor_state: gui::default_state(),
            beat: IntParam::new("beat", 0, IntRange::Linear { min: 0, max: 35 }),
            vol: IntParam::new("vol", 0, IntRange::Linear { min: 0, max: 35 }),
        }
    }
}

/// The functor plugin and all of the memory it needs to allocate.
#[derive(Default)]
pub struct Functor {
    /// The parameters for the plugin.
    params: Arc<FunctorParams>,
}

impl Plugin for Functor {
    const NAME: &'static str = "Functor";
    const URL: &'static str = "https://viiii.neocities.org";
    const EMAIL: &'static str = "vi.hdz.p@gmail.com";
    const VENDOR: &'static str = "viiii";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(1),
            main_output_channels: NonZeroU32::new(1),
            ..AudioIOLayout::const_default()
        },
    ];

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        gui::create(self.params.clone(), self.params.editor_state.clone())
    }

    // Do nothing for now
    fn process(
        &mut self,
        _buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        ProcessStatus::Normal
    }
}

impl Vst3Plugin for Functor {
    const VST3_CLASS_ID: [u8; 16] = *b"fooofooofooofooo";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Tools];
}

nih_export_vst3!(Functor);
