use raylib::prelude::*;

#[derive(Copy, Clone)]
pub struct Circle {
  pub radius: f32,
  pub position: Vector2,
  pub last_position: Vector2,
  pub acceleration: Vector2,
  pub color: Color,
}

impl Circle {
  pub const fn new(radius: f32, position: Vector2, color: Color) -> Self {
    Circle {
      radius,
      position,
      last_position: position,
      acceleration: Vector2::new(0.0, 0.0),
      color,
    }
  }
}
