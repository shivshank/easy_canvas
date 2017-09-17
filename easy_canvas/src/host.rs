use shape::{DrawCmd};

use std::sync::mpsc::Sender;

/// A Host provides Canvases with a place to send events.
pub trait Host {
    /// Get a sender/transmitter that accepts DrawCmds.
    ///
    /// TODO: Should we change this?
    /// Can fail if the host chooses to only allow one transmitter.
    fn sender(&mut self) -> Option<Sender<DrawCmd>>;
}
