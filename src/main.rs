pub mod circle;
pub mod constants;
pub mod impulse_solver;

use circle::*;
use constants::*;
use impulse_solver::ImpulseSolver;
use raylib::prelude::*;

fn main() {
  let (mut rl, thread) = raylib::init()
    .size(800, 800)
    .title("Hello, World")
    .build();

  rl.set_target_fps(60);

  let mut sim = ImpulseSolver::new();

  while !rl.window_should_close() {
    sim.do_step();

    let time = rl.get_time();

    if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
      let pos = rl.get_mouse_position();

      let color = Color::color_from_hsv(time.sin().abs() as f32 * 255.0, 1.0, 1.0);
      sim.add_circle(Circle::new(pos, color));
    }

    let mut d: RaylibDrawHandle = rl.begin_drawing(&thread);
    d.clear_background(Color::BLACK);
    d.draw_fps(0, 0);
    for o in &sim.objs {
      d.draw_circle_v(o.position, RADIUS, o.color);
    }
  }
}
