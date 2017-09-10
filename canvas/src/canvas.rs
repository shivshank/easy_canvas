use color::Color;
use style::Style;
use shape::{ToDrawCmd, DrawCmd};

use cgmath::Matrix4;

use std::sync::mpsc::Sender;

/// A Host provide Canvases with a place to send events.
pub trait Host {
    fn sender(&mut self) -> Option<Sender<DrawCmd>>;
}

pub trait Target {
    fn draw<D: ToDrawCmd>(&self, shape: D, style: Style);

    fn clear<C: Color>(&self, color: C);
}

pub struct Canvas {
    width: u32,
    height: u32,
    projection: Matrix4<f32>,
    tx: Sender<DrawCmd>,
}

impl Target for Canvas {
    fn draw<D: ToDrawCmd>(&self, shape: D, style: Style) {
        self.tx.send(shape.with_style(style))
            .expect("Canvas host hung up");
    }

    fn clear<C: Color>(&self, color: C) {
        self.tx.send(DrawCmd::Clear(color.normalize()))
            .expect("Canvas host hung up");
    }
}

impl Canvas {
    pub fn new<H: Host>(mut host: H, width: u32, height: u32) -> Canvas {
        Canvas {
            width,
            height,
            projection: make_projection_matrix(width, height, 1),
            tx: host.sender().expect("No sender was available from the host"),
        }
    }
}

/// Wraps Canvas::new().
///
/// Not idiomatic Rust, but it makes the library feel "easier" to me...
#[inline]
pub fn create<H: Host>(host: H, width: u32, height: u32) -> Canvas {
    Canvas::new(host, width, height)
}

fn make_projection_matrix(width: u32, height: u32, _layers: u32) -> Matrix4<f32> {
    // TODO: Add support for rendering at discrete depths
    assert_ne!(_layers, 0, "Cannot create a canvas with 0 layers");
    let (width, height) = (width as f32, height as f32);
    Matrix4::new(
        2.0 / width, 0.0, 0.0, 0.0,
        0.0, 2.0 / height, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    )
}
