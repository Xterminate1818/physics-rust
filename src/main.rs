pub mod circle;
pub mod impulse_solver;
pub mod spawner;

use std::time::Instant;

use circle::*;
use impulse_solver::ImpulseSolver;
use raylib::prelude::*;
use spawner::Spawner;

fn main() {
  visual_sim();
  let mut sim = ImpulseSolver::new();
  for x in 0..100 {
    for y in 0..100 {
      let c = Circle::new(
        5.0,
        Vector2::new(x as f32 * 10.0, y as f32 * 10.0),
        Color::WHITE,
      );
      sim.add_circle(c);
    }
  }
  println!("\n");
  let objs = sim.count_circles();
  println!("Objects: {}", objs);
  let t = Instant::now();
  sim.do_step();
  let elapsed = t.elapsed().as_millis();
  println!("Total elapsed: {} msec", elapsed);
  let score = objs as f32 / elapsed as f32;
  println!("Score: {}", score as i32)
}

fn visual_sim() {
  let genfunc = |id: i32| -> Circle {
    let seed = id as f32 / 10.0;
    let color = Color::color_from_hsv((id) as f32, 1.0, 1.0);
    let pos = Vector2::new((id * 100 % 1000) as f32 + seed.sin() * 10.0, 10.0);
    Circle::new(5.0, pos, color)
  };

  let (mut rl, thread) = raylib::init()
    .size(1000, 1000)
    .title("Hello, World")
    .build();

  rl.set_target_fps(60);

  let mut sim = ImpulseSolver::new();
  let mut gen = Spawner::new(0.01, rl.get_time());

  while !rl.window_should_close() {
    sim.do_step();
    let generated = gen.update(rl.get_time(), genfunc);
    for c in generated {
      sim.add_circle(c);
    }
    if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
      let pos = rl.get_mouse_position();
      let color = Color::WHITE;
      sim.add_circle(Circle::new(10.0, pos, color));
    }

    let mut d: RaylibDrawHandle = rl.begin_drawing(&thread);
    d.clear_background(Color::BLACK);
    for o in &sim.buf1 {
      d.draw_circle_v(o.position, o.radius, o.color);
    }

    d.draw_fps(0, 0);
    let count = sim.count_circles().to_string();
    d.draw_text(&count, 0, 20, 20, Color::WHITE);
  }
}
