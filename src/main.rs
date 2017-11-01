extern crate piston_window;
extern crate opengl_graphics;

// use piston_window::{OpenGL ,PistonWindow, WindowSettings};
use piston_window::{PistonWindow, WindowSettings, EventLoop};
use opengl_graphics::{OpenGL, GlGraphics};
fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: PistonWindow = WindowSettings::new("Super Mario game", [600, 600])
        .exit_on_esc(true)
        .build()
        .unwrap();

    window.set_ups(60);
    window.set_max_fps(60);

    let mut gl = GlGraphics::new(opengl);
}
