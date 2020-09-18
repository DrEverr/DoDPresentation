extern crate piston_window;
mod dod;

pub use dod::{animate, init, World};
use piston_window::*;

fn main() {
    let mut world = init(4, 512.0, 512.0);
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("showcase", [512; 2])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    while let Some(e) = window.next() {
        animate(&mut world);
        window.draw_2d(&e, |c, g, _| {
            clear([0.6; 4], g);
            for pos in world.positions.iter() {
                let c = c.trans(pos.0, pos.1);
                let green = [1.0, 1.0, 0.0, 1.0];
                let rect = [0.0, 0.0, 60.0, 60.0];
                ellipse(green, rect, c.transform, g);
            }
        });
    }
}
