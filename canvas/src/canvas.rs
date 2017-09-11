use color::Color;
use style::Style;
use shape::{ToDrawCmd, DrawCmd};

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
    pub fn new<H: Host>(mut host: H, _width: u32, _height: u32) -> Canvas {
        Canvas {
            tx: host.sender().expect("No sender was available from the host"),
        }
    }

    pub fn use_post_process(&self, shader: &str) {
        self.tx.send(DrawCmd::UsePostProcess(shader.to_string()))
            .expect("Canvas host hung up");
    }
}

/// Wraps Canvas::new().
///
/// Not idiomatic Rust, but it makes the library feel "easier" to me...
#[inline]
pub fn create<H: Host>(host: H, width: u32, height: u32) -> Canvas {
    Canvas::new(host, width, height)
}
