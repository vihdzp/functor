use nih_plug_vizia::vizia::{cache::BoundingBox, prelude::*, vg};

use crate::curve::{Curve, Mode, Node, Preset};

/// A list widget for the beats or volumes.
pub struct CurveView<LBeat, LVol, LMode, LIndex>
where
    LBeat: Lens<Target = Vec<Preset>>,
    LVol: Lens<Target = Vec<Preset>>,
    LMode: Lens<Target = Mode>,
    LIndex: Lens<Target = usize>,
{
    beat_lens: LBeat,
    vol_lens: LVol,
    mode_lens: LMode,
    index_lens: LIndex,
}

impl<LBeat, LVol, LMode, LIndex> CurveView<LBeat, LVol, LMode, LIndex>
where
    LBeat: Lens<Target = Vec<Preset>>,
    LVol: Lens<Target = Vec<Preset>>,
    LMode: Lens<Target = Mode>,
    LIndex: Lens<Target = usize>,
{
    /// Creates a new List view with a binding to the given lens and a template
    /// for constructing the list items
    pub fn new(
        cx: &mut Context,
        beat_lens: LBeat,
        vol_lens: LVol,
        mode_lens: LMode,
        index_lens: LIndex,
    ) -> Handle<Self>
    where
        <LBeat as Lens>::Source: Model,
        <LVol as Lens>::Source: Model,
        <LMode as Lens>::Source: Model,
        <LIndex as Lens>::Source: Model,
    {
        Self {
            beat_lens,
            vol_lens,
            mode_lens,
            index_lens,
        }
        .build(cx, |_| {})
    }

    pub fn curve(&self, cx: &mut DrawContext) -> Curve {
        match self.mode_lens.get(cx) {
            Mode::Beat => self
                .beat_lens
                .clone()
                .index(self.index_lens.get(cx))
                .then(Preset::curve)
                .get(cx),

            Mode::Vol => self
                .vol_lens
                .clone()
                .index(self.index_lens.get(cx))
                .then(Preset::curve)
                .get(cx),
        }
    }
}

pub fn draw_node(bounds: BoundingBox, canvas: &mut Canvas, node: Node) {
    // We draw rightwards and upwards.
    let x = bounds.x + bounds.w * node.x;
    let y = bounds.y + bounds.h * (1.0 - node.y);

    canvas.clear_rect(x as u32 - 5, y as u32 - 5, 10, 10, vg::Color::black());
}

impl<LBeat, LVol, LMode, LIndex> View for CurveView<LBeat, LVol, LMode, LIndex>
where
    LBeat: Lens<Target = Vec<Preset>>,
    LVol: Lens<Target = Vec<Preset>>,
    LMode: Lens<Target = Mode>,
    LIndex: Lens<Target = usize>,
{
    fn element(&self) -> Option<&'static str> {
        Some("functor-list")
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        let bounds = cx.bounds();

        for &node in self.curve(cx).iter() {
            draw_node(bounds, canvas, node);
        }

        let mut path = vg::Path::new();
        path.rect(bounds.x, bounds.y, bounds.w, bounds.h);
        canvas.stroke_path(&mut path, &vg::Paint::color(vg::Color::black()));
    }
}
