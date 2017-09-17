//! An easy to use 2D drawing library for Rust.
//!
//! `easy_canvas` is completely independent from any drawing or windowing library. 
//!
//! A simple canvas implementation can be created via [`easy_canvas::create`][create] by providing
//! a [`Host`][host]. The `quick_window` library is recomended to get started quickly.
//! 
//! [create]: ./canvas/fn.create.html
//! [host]: ./host/trait.Host.html

extern crate cgmath;
#[cfg(feature = "drawing")]
extern crate gl;

pub mod color;
pub mod style;
pub mod shape;
pub mod transform;
pub mod canvas;
pub mod host;
#[cfg(feature = "drawing")]
pub mod drawing;

pub use color::*;
pub use style::*;
pub use shape::*;
pub use transform::*;
pub use canvas::*;
pub use host::*;

pub use cgmath::{Angle, Deg, Rad};