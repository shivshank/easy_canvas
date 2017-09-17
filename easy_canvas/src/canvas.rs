use color::Color;
use style::Style;
use shape::{ToDrawCmd, DrawCmd};
use transform::Transform;
use host::Host;

use std::sync::mpsc::Sender;

/// A generic drawing target.
pub trait Target {
    fn draw<D: ToDrawCmd>(&self, Transform, Style, shape: D);

    fn clear<C: Color>(&self, color: C);

    fn with_state<F: FnMut(ImplicitTarget<Self>)>(&self, Transform, Style, cb: F);

    /// Implies an implementaiton defined default style.
    ///
    /// Users are encouraged to use the draw_with_style function on the implict target.
    ///
    /// Implementors are encouraged to use fill black.
    fn with_transform<F: FnMut(ImplicitTarget<Self>)>(&self, Transform, cb: F);

    /// Implies identity transformation.
    ///
    /// Users are encouraged to use the draw_with_transform function on the implict target.
    fn with_style<F: FnMut(ImplicitTarget<Self>)>(&self, Style, cb: F);
}

/// Stores a transform and style as implicit constants for draw calls.
pub struct ImplicitTarget<'p, T: Target + 'p> {
    parent: &'p T,
    transform: Transform,
    style: Style
}

impl<'p, T: Target> ImplicitTarget<'p, T> {
    pub fn from_state(parent: &T, transform: Transform, style: Style) -> ImplicitTarget<T> {
        ImplicitTarget {
            parent, transform, style
        }
    }
}

impl<'p, T: Target> ImplicitTarget<'p, T> {
    pub fn draw<D: ToDrawCmd>(&self, shape: D) {
        self.parent.draw(self.transform, self.style, shape);
    }

    pub fn draw_with_style<D: ToDrawCmd>(&self, style: Style, shape: D) {
        self.parent.draw(self.transform, style, shape);
    }

    pub fn draw_with_transform<D: ToDrawCmd>(&self, transform: Transform, shape: D) {
        self.parent.draw(transform, self.style, shape);
    }

    /// Identical to Target::clear.
    ///
    /// It would not be unreasonable to shadow the Target variable in the with_state callback,
    /// so we might as well mirror the clear method in the implicit target.
    pub fn clear<C: Color>(&self, color: C) {
        self.parent.clear(color);
    }
}

pub struct Canvas {
    tx: Sender<DrawCmd>,
}

impl Target for Canvas {
    fn draw<D: ToDrawCmd>(&self, transform: Transform, style: Style, shape: D) {
        self.tx.send(shape.with_state(transform, style))
            .expect("Canvas host hung up");
    }

    fn clear<C: Color>(&self, color: C) {
        self.tx.send(DrawCmd::Clear(color.normalize()))
            .expect("Canvas host hung up");
    }

    fn with_state<F: FnMut(ImplicitTarget<Self>)>(&self, transform: Transform, 
            style: Style, mut cb: F) {
        let t = ImplicitTarget::from_state(self, transform, style);
        cb(t);
    }

    fn with_style<F: FnMut(ImplicitTarget<Self>)>(&self, style: Style, cb: F) {
        self.with_state(Transform::identity(), style, cb);
    }

    fn with_transform<F: FnMut(ImplicitTarget<Self>)>(&self, transform: Transform, cb: F) {
        self.with_state(transform, Style::fill((0, 0, 0)), cb);
    }
}

impl Canvas {
    pub fn new<H: Host>(host: &mut H, _width: u32, _height: u32) -> Canvas {
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
/// Not idiomatic Rust, but I think it's a nice method to have for this library ;)
#[inline]
pub fn create<H: Host>(host: &mut H, width: u32, height: u32) -> Canvas {
    Canvas::new(host, width, height)
}
