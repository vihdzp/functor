pub mod curve_view;
pub mod icon;
pub mod list;

use nih_plug::prelude::Plugin;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};

use std::sync::Arc;

use crate::curve::{Mode, Preset};
use crate::gui::list::{test_presets, FunctorList};
use crate::FunctorParams;

/// The size of the window.
const SIZE: (u32, u32) = (800, 610);

/// All the data that needs to be kept track of in the editor.
#[derive(Lens)]
struct AppData {
    /// The parameters for the plugin.
    params: Arc<FunctorParams>,

    /// The mode of the selected preset.
    mode: Mode,
    /// The index of the selected preset.
    index: usize,

    /// The presets for the beats.
    beat_presets: Vec<Preset>,
    /// The presets for the volumes.
    vol_presets: Vec<Preset>,

    dc: bool,
    // interpolation: Interpolation,
}

/// An event relating to a preset.
pub enum PresetEvent {
    /// Set a beat or volume.
    Set {
        /// Whether we're setting a beat or a volume.
        mode: Mode,
        /// The index of the beat or volume.
        index: usize,
        /// The new preset.
        preset: Preset,
    },

    /// Select a given beat or volume.
    Select {
        /// Whether we're selecting a beat or a volume.
        mode: Mode,
        /// The index of the beat.
        index: usize,
    },
}

/*

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Interpolation {
    Drop,
    Linear,
    Cubic,
    Hermite,
}

pub enum AppEvent {
    DcToggle,

    Interpolation(Interpolation),
}
*/

impl Model for AppData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        // Preset event
        event.map(|preset_event, _| match preset_event {
            PresetEvent::Set {
                mode,
                index,
                preset,
            } => {
                let old_preset = match mode {
                    Mode::Beat => self.beat_presets.get_mut(*index),
                    Mode::Vol => self.vol_presets.get_mut(*index),
                };

                if let Some(old_preset) = old_preset {
                    *old_preset = preset.clone();
                }
            }

            PresetEvent::Select { mode, index } => {
                self.mode = *mode;
                self.index = *index;
            }
        });

        // App event
        /* event.map(|app_event, _| match app_event {
            AppEvent::DcToggle => {
                self.dc = !self.dc;
            }

            AppEvent::Interpolation(interpolation) => {
                self.interpolation = *interpolation;
            }
        }) */
    }
}

pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| SIZE)
}

/*
fn interpolation_checkbox(cx: &mut Context, text: &str, interpolation: Interpolation) {
    Label::new(cx, text)
        .font_size(12.0)
        .top(Pixels(1.0))
        .right(Pixels(2.0));

    Checkbox::new(cx, AppData::interpolation.map(move |x| *x == interpolation))
        .on_toggle(move |cx| cx.emit(AppEvent::Interpolation(interpolation)))
        .background_color(Color::rgb(127, 127, 127))
        .border_radius(Pixels(9.0))
        .right(Pixels(7.0))
        .size(Pixels(18.0));
}
*/

pub(crate) fn create(
    params: Arc<FunctorParams>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn nih_plug::editor::Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        assets::register_noto_sans_light(cx);
        assets::register_noto_sans_thin(cx);

          AppData {
                params: params.clone(),
                mode: Mode::Beat,
                index: 0,
                beat_presets: test_presets(12),
                vol_presets: test_presets(12),
                dc: true,
                //interpolation: Interpolation::Hermite,
            }
            .build(cx);
        

        /*
        HStack::new(cx, |cx| {
            // Functor icon
             icon::FunctorIcon::new(cx)
            .size(Pixels(icon::IMG_SIZE as f32))
            .top(Pixels(12.5))
            .bottom(Pixels(12.5))
            .left(Pixels(15.0))
            .right(Pixels(12.5));

            // Functor label
             Label::new(cx, "Functor")
            .font_size(52.0)
            .top(Pixels(4.0))
            .on_mouse_down(|_, _| {
                // Try to open the plugin's page when clicking on the title.
                // If this fails then that's not a problem.
                let result = open::that(crate::Functor::URL);
                if cfg!(debug) && result.is_err() {
                    crate::nih_debug_assert_failure!(
                        "Failed to open web browser: {:?}",
                        result
                    );
                }
            })
            .cursor(CursorIcon::Hand); // Broken in baseview
        })
        .background_color(Color::rgb(200, 150, 255))
        .bottom(Pixels(20.0))
        .max_height(Pixels(icon::IMG_SIZE as f32)); */

        HStack::new(cx, |cx| {
            // Left area
            VStack::new(cx, |cx| {
                // Beat label
                /* Label::new(cx, "Beat")
                .font_size(20.0)
                .left(Pixels(3.0))
                .bottom(Pixels(5.0)); */

                // Beat list
                // ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                FunctorList::new(cx, Mode::Beat, AppData::beat_presets) //;
                    // })
                    .height(Pixels(140.0))
                    .bottom(Pixels(10.0));

                // Gate label
                /* Label::new(cx, "Gate")
                .font_size(20.0)
                .left(Pixels(3.0))
                .bottom(Pixels(5.0)); */

                // Gate list
                //ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                FunctorList::new(cx, Mode::Vol, AppData::vol_presets) //;
                    //   })
                    .height(Pixels(140.0))
                    .bottom(Pixels(20.0));

                // Advanced label
                /*  VStack::new(cx, |cx| {
                            Label::new(cx, "Advanced")
                                .left(Pixels(3.0))
                                .bottom(Pixels(10.0))
                                .font_size(20.0);

                            Label::new(cx, "Interpolation")
                                .font_size(14.0)
                                .left(Pixels(12.0))
                                .bottom(Pixels(2.0));

                            HStack::new(cx, |cx| {
                                interpolation_checkbox(cx, "D", Interpolation::Drop);
                                interpolation_checkbox(cx, "L", Interpolation::Linear);
                                interpolation_checkbox(cx, "C", Interpolation::Cubic);
                                interpolation_checkbox(cx, "H", Interpolation::Hermite);
                            })
                            .left(Pixels(12.0))
                            .height(Pixels(30.0));

                            HStack::new(cx, |cx| {
                                Label::new(cx, "DC offset")
                                    .font_size(14.0)
                                    .left(Pixels(12.0))
                                    .right(Pixels(2.0));

                                Checkbox::new(cx, AppData::dc)
                                    .on_toggle(|cx| cx.emit(AppEvent::DcToggle))
                                    .background_color(Color::rgb(127, 127, 127))
                                    .top(Pixels(1.0))
                                    .size(Pixels(16.0));
                            });
                        })
                        .background_color(Color::rgb(220, 220, 220))
                        .border_radius(Pixels(10.0));
                */
            })
            .left(Pixels(10.0))
            .bottom(Pixels(10.0))
            .width(Pixels(262.0));

            // Curve view
            /* curve_view::CurveView::new(
                cx,
                AppData::beat_presets,
                AppData::vol_presets,
                AppData::mode,
                AppData::index,
            )
            .left(Pixels(15.0))
            .right(Pixels(20.0))
            .size(Pixels(500.0)); */
        });
    })
}
