# Do Not Put Examples in canvas

Instead put all examples in `canvas_quick_window`. The build script for `quick_window` needs to be fixed; currently Rust attempts to look for glfw.lib relative to `canvas` and not `quick_window`.
