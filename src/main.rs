use raylib::prelude::*;
pub mod geometry;
use geometry as gm;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(600, 600)
        .title("Hello, World")
        .build();

    let mut curs = gm::Circle::new(0_f64, 0_f64, 15_f64);
     
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
         
        d.clear_background(Color::BLACK);
    }
}
