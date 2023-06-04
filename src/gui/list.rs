use std::marker::PhantomData;

use crate::curve::{Curve, Mode, Node, Preset};
use nih_plug_vizia::vizia::prelude::*;

use super::PresetEvent;

/// Some dummy presets to test the layout.
pub(crate) fn test_presets(n: usize) -> Vec<Preset> {
    (0..n)
        .map(|i| Preset {
            name: format!("Test {}", i),
            curve: Curve {
                nodes: vec![
                    Node { x: 0.0, y: 0.0 },
                    Node {
                        x: i as f32 / 50.0,
                        y: i as f32 / 50.0,
                    },
                ],
            },
        })
        .collect()
}

/// The buttons for selecting a preset.
pub struct FunctorButton<L>
where
    L: Lens<Target = Vec<Preset>>,
{
    p: PhantomData<L>,
}

impl<L> Default for FunctorButton<L>
where
    L: Lens<Target = Vec<Preset>>,
{
    fn default() -> Self {
        Self { p: PhantomData }
    }
}

impl<L: 'static + Lens<Target = Vec<Preset>>> View for FunctorButton<L> {
    fn element(&self) -> Option<&'static str> {
        Some("functor-button")
    }
}

pub fn functor_button<L: Lens<Target = Vec<Preset>>>(
    cx: &mut Context,
    mode: Mode,
    lens: L,
    index: usize,
) -> Handle<Button>
where
    <L as Lens>::Source: Model,
{
    let ptr = lens.clone().index(index);

    Button::new(
        cx,
        move |cx| {
            cx.emit(PresetEvent::Select { mode, index });
        },
        |cx| {
            Label::new(cx, ptr.then(Preset::name))
                .width(Pixels(58.0))
                .font_size(12.0)
        },
    )
}

/// A list widget for the beats or volumes.
pub struct FunctorList<L>
where
    L: Lens<Target = Vec<Preset>>,
{
    p: PhantomData<L>,
}

impl<L> Default for FunctorList<L>
where
    L: Lens<Target = Vec<Preset>>,
{
    fn default() -> Self {
        Self { p: PhantomData }
    }
}

impl<L: 'static + Lens<Target = Vec<Preset>>> View for FunctorList<L> {
    fn element(&self) -> Option<&'static str> {
        Some("functor-list")
    }
}

impl<L: 'static + Lens<Target = Vec<Preset>>> FunctorList<L> {
    /// Creates a new [`FunctorList] view with a binding to the given lens and a
    /// template for constructing the list items.
    pub fn new(cx: &mut Context, mode: Mode, lens: L) -> Handle<Self>
    where
        <L as Lens>::Source: Model,
    {
        FunctorList::default().build(cx, move |cx| {
            // Bind to the list length.
            Binding::new(
                cx,
                lens.clone().map(|lst| lst.len()),
                move |cx, list_len| {
                    // If the number of list items is different to the number of
                    // children of the ListView then remove and rebuild all the
                    // children.
                    let list_len = list_len.get_fallible(cx).unwrap_or_default();

                    for row in 0..(list_len / 3) {
                        HStack::new(cx, |cx| {
                            for col in 0..3 {
                                functor_button(cx, mode, lens.clone(), 3 * row + col);
                            }
                        })
                        .max_height(Pixels(35.0));
                    }

                    HStack::new(cx, |cx| {
                        for col in 0..(list_len % 3) {
                            functor_button(cx, mode, lens.clone(), 3 * (list_len / 3) + col);
                        }
                    })
                    .max_height(Pixels(35.0));
                },
            );
        })
    }
}
