use std::cell::RefCell;

use nih_plug_vizia::vizia;
use vg::imgref::Img;
use vg::rgb::FromSlice;
use vizia::image;
use vizia::prelude::*;
use vizia::vg;

/// The size for the Functor icon.
pub const IMG_SIZE: usize = 50;

/// The Functor icon.
pub struct FunctorIcon {
    image: RefCell<Option<vg::ImageId>>,
}

impl FunctorIcon {
    /// Builds the Functor icon.
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {
            image: RefCell::new(None),
        }
        .build(cx, |_| {})
    }
}

/// Returns the Functor logo, which is embedded in the executable.
fn logo() -> image::ImageResult<image::RgbaImage> {
    let mut reader = image::io::Reader::new(std::io::Cursor::new(include_bytes!(
        "../../resources/logo.png"
    )));
    reader.set_format(image::ImageFormat::Png);
    reader.decode().map(|img| img.into_rgba8())
}

impl View for FunctorIcon {
    fn element(&self) -> Option<&'static str> {
        Some("functor-icon")
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        let bounds = cx.bounds();

        let image_id = if self.image.borrow().is_none() {
            let logo = logo().expect("logo failed to load");
            let image = Img::new(logo.as_rgba(), IMG_SIZE, IMG_SIZE);
            let image_id = canvas
                .create_image(image, vg::ImageFlags::empty())
                .expect("image couldn't be loaded");

            *self.image.borrow_mut() = Some(image_id);
            image_id
        } else {
            self.image.borrow().unwrap()
        };

        let mut path = vg::Path::new();
        path.rect(bounds.x, bounds.y, bounds.w, bounds.h);
        let paint = vg::Paint::image(image_id, bounds.x, bounds.y, bounds.w, bounds.h, 0.0, 1.0);
        canvas.fill_path(&mut path, &paint);
    }
}
