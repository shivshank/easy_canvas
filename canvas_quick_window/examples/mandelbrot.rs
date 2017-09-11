extern crate canvas;
extern crate canvas_quick_window as quick_window;

use canvas::Target;

use quick_window::Window;

fn main() {
    quick_window::create("Fractals!", 800, 600, 1.0 / 60.0, mandelbrot);
}

fn mandelbrot(window: Window) {
    let ctx = canvas::create(window, 800, 600);

    // ctx.draw(Rect(0, 0, 10, 10), style);
    ctx.use_post_process(r#"
        void mainImage(inout vec4 fragColor, in vec2 uv) {
            vec2 fragCoord = uv * resolution;
            if (length(fragCoord - resolution * 0.5) < 100.0) {
                fragColor = texture(diffuse, uv);
            } else {
                fragColor = vec4(0.0, 0.0, 0.0, 0.0);
            }
        }
    "#);

    ctx.clear((200, 150, 200, 0.25));
}

/*
// the goal:

fn main() {
    let mut window = quick_window::create("Canvas!", 800, 600, 1.0 / 60.0);
    let mut ctx = canvas::create_flat(window, 800, 600);

    // solid blue fill; paths/lines/arcs may or may not ignore fill, not sure yet
    // other styles will exist for dotted lines and other fancy things
    let solid = SolidStyle {
        fill: true,
        color: (0x33, 0x66, 0xFF).normalize(),
    };

    ctx.clear((50, 50, 50));

    // draw a 100px x 100px square at the window center along with a circle of 150px diameter
    transform.translate(400.0, 300.0);
    ctx.draw(transform * rect(-50.0, -50.0, 100.0, 100.0), style);
    ctx.draw(transform * arc(0.0, 0.0, 75.0, 0.0, 6.3), style);

    // create a new layer above ctx
    let layer = ctx.add_layer();
    // I'm thinking each layer has it's own Matrix so this will draw a circle in the top left
    // corner instead of adopting the canvas's base Matrix
    layer.draw(Circle(0.0, 0.0, ))
    // we can still draw to the original ctx which now represents the bottom-most layer
    ctx.reset_transform();
    ctx.draw(Rect(700, 500, 100, 100), style);


}
 */
