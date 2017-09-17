//! An easy to use html-canvas-like API for 2d drawing.
//!
//! `canvas` assumes an OpenGL context is available and will make OpenGL calls to whatever context
//! is current on the present thread. `canvas` makes many assumptions about the current state of
//! the GL, so if you need to insert your own calls make sure to clean up your state when done.
//!
//! Currently provides one kind of canvas:
//!
//! - flat, a very basic canvas that simply renders commands to a target, depths are resolved
//! by render order. The canvas will not automatically clear itself and changes to the camera will
//! only be reflected in the following render commands. Nearly identical to HTML canvas.
//!

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