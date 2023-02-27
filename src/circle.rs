use crate::constants::*;
use raylib::prelude::*;

#[derive(Copy, Clone)]
pub struct Circle {
  pub position: Vector2,
  pub last_position: Vector2,
  pub acceleration: Vector2,
}

impl Circle {
  pub const fn new(position: Vector2) -> Self {
    Circle {
      position,
      last_position: position,
      acceleration: Vector2::new(0.0, 0.0),
    }
  }

  pub fn find_impulse(circle1: &Circle, circle2: &Circle) -> Vector2 {
    let combined = circle1.position - circle2.position;
    let length = combined.length();
    if length > DIAMETER {
      Vector2::zero()
    } else {
      combined.normalized()
    }
  }
}