use nih_plug_vizia::vizia::prelude::*;
use serde::{Deserialize, Serialize};

/// A node in a curve.
#[derive(Clone, Copy, Default, Serialize, Deserialize)]
pub struct Node {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Curve {
    pub nodes: Vec<Node>,
}

impl Curve {
    pub fn iter(&self) -> std::slice::Iter<Node> {
        self.nodes.iter()
    }
}

/// Whether a preset is meant for a beat or a volume.
#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Mode {
    /// A beat or timestretching preset.
    Beat,

    /// A volume or gating preset.
    Vol,
}

/// A Functor preset. This consists of a [`Curve`] and the name of the preset.
#[derive(Clone, Lens)]
pub struct Preset {
    /// The name of the preset.
    pub name: String,

    /// The curve the beat or volume follows.
    pub curve: Curve,
}

/*
/// A [`Preset`] bundled with the [`Mode`] it's intended for.
pub struct ModePreset {
    /// The bundled preset.
    pub preset: Preset,

    /// The intended mode for the preset.
    pub mode: Mode,
}
*/
