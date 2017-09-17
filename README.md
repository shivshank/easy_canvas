# Easy Canvas
Easy Canvas is a rust library designed to make it easy to prototype ideas that need a 2d drawing API. Most other libraries are powerful yet cumbersome to use, especially for toying around and making simple examples.

Easy Canvas is inspired by the HTML5 Canvas API. Notably, it uses similar primitives (Rect, Arc, Path)

# Event Handling

Currently `quick_window` does not provide any way to handle events.

The front end window is however 100% separate from the canvas. If you want to roll your own event loop quickly, you can use 

# Easy Canvas is a work in progress.

The API is currently unstable and I am actively looking for feedback!

`quick_window` currently uses GLFW. A GLFW precompiled lib file is provided for Windows users but Linux and Mac users will need to build GLFW themselves and override the build script.

I did add support for glutin, but it seemed to add significantly to start up time and memory usage, so at the moment I am not bothering with it. It also does not expose any kind of "wait for events or until timeout" sleep function like GLFW (GLFW uses WaitEvent on windows and presumably its analogue on other platforms).

Methods in `easy_canvas::Target` are most likely stable, while `easy_canvas::Canvas` methods are subject to change.

`quick_window` currently waits for events but also runs every so often to poll for drawing commands. I don't think there's any good reason that this behavior is exactly necessary. The ideal behavior is probably "tick no more often than refresh_rate seconds, sleep if there are no events or drawing commands."

But for the niche the library is targeting this seems like overkill. A simple polling event loop would suffice, but it feels rude to waste CPU cycles.

TODO:
- Make Target generic over DrawCmd
- Move DrawCmd behind a feature flag, only expose shape API by default.

- Add support for (easy) event handling.
- Clean up drawing API in `easy_canvas` (don't look at it, it's gross!).