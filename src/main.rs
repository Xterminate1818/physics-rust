pub mod bounds;
pub mod impulse_solver;
pub mod verlet_object;

use impulse_solver::ImpulseSolver;
use raylib::prelude::*;
use verlet_object::*;

fn main() {
  launch_app();
}

fn launch_app() {
  let (mut rl, thread) = raylib::init()
    .size(500, 500)
    .title("Game")
    .build();

  rl.set_target_fps(60);

  let mut sim = ImpulseSolver::new();

  while !rl.window_should_close() {
    sim.do_step();
    if rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
      let pos = rl.get_mouse_position();
      let color =
        Color::color_from_hsv(sim.count_circles() as f32 * 2.0, 1.0, 1.0);
      let obj = VerletObject::new(pos, color);
      if !sim.would_collide(&obj) {
        sim.add_circle(VerletObject::new(pos, color));
      }
    }

    let mut d: RaylibDrawHandle = rl.begin_drawing(&thread);
    d.clear_background(Color::BLACK);
    for o in &sim.objects {
      d.draw_circle_v(o.position, RADIUS, o.color);
    }
    d.draw_circle_lines(250, 250, 250.0, Color::WHITE);

    // d.draw_fps(0, 0);
    // let count = sim.count_circles().to_string();
    // d.draw_text(&count, 0, 20, 20, Color::WHITE);
  }
}
