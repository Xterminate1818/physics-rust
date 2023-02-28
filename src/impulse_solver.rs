use crate::circle::*;
use crate::constants::*;
use raylib::prelude::*;

pub struct ImpulseSolver {
  pub objs: Vec<Circle>,
  pub substeps: u32,
  bounds: Rectangle,
}

impl Default for ImpulseSolver {
  fn default() -> Self {
    Self {
      objs: vec![],
      substeps: 32,
      bounds: Rectangle {
        x: 0.0,
        y: 0.0,
        width: 800.0,
        height: 800.0,
      },
    }
  }
}

impl ImpulseSolver {
  pub fn new() -> Self {
    ImpulseSolver::default()
  }

  pub fn add_circle(&mut self, circle: Circle) {
    let pos = &circle.position;
    if pos.x >= self.bounds.x
      && pos.x <= self.bounds.x + self.bounds.width
      && pos.y >= self.bounds.y
      && pos.y <= self.bounds.y + self.bounds.height
    {
      self.objs.push(circle);
    }
  }

  pub fn count_circles(&self) -> usize {
    self.objs.len()
  }

  pub fn calculate_accelerations(&mut self, delta: f32) {
    for circle_index in 0..self.objs.len() {
      // Apply gravity
      self.objs[circle_index].acceleration += Vector2::new(0.0, 9.8);
      for other_index in circle_index..self.objs.len() {
        let circle = &self.objs[circle_index].position;
        let other = &self.objs[other_index].position;
        if circle.x < other.x - DIAMETER {
          break; // No further collisions possible
        }
        let dx = (circle.x - other.x).abs();
        let dy = (circle.y - other.y).abs();
        if dx >= DIAMETER || dy >= DIAMETER {
          continue; // Skip over obvious noncollisions
        }
        // Finally, resort to expensive calculation
        let resolution = Circle::find_impulse(
          &self.objs[circle_index],
          &self.objs[other_index],
        ) * delta;
        self.objs[circle_index].position += resolution;
        self.objs[other_index].position -= resolution;
      }
    }
  }

  pub fn resolve_velocities(&mut self, delta: f32) {
    let delta_squared = (delta).powi(2);
    for o in &mut self.objs {
      let velocity = o.position - o.last_position;
      o.last_position = o.position;
      o.position = o.position + velocity + o.acceleration * delta_squared;
      o.acceleration = Vector2::zero();
      // Apply bounds
      o.position.x = o.position.x.clamp(
        self.bounds.x + RADIUS,
        self.bounds.x + self.bounds.width - RADIUS,
      );
      o.position.y = o.position.y.clamp(
        self.bounds.y + RADIUS,
        self.bounds.y + self.bounds.height - RADIUS,
      );
    }
  }

  pub fn apply_constraints(&mut self, circle: &mut Circle) {
    if circle.position.y + RADIUS < self.bounds.height + self.bounds.y {
      circle.position.y = self.bounds.height + self.bounds.y;
    }
  }

  pub fn sort_objects(&mut self) {
    self
      .objs
      .sort_unstable_by(|a, b| a.position.x.partial_cmp(&b.position.x).unwrap())
  }

  pub fn set_substeps(&mut self, new: u32) {
    self.substeps = new;
  }

  pub fn do_substep(&mut self, delta: f32) {
    self.calculate_accelerations(delta);
    self.resolve_velocities(delta);
    self.sort_objects();
  }

  pub fn do_step(&mut self) {
    let substep_delta = 0.1 / (self.substeps as f32);
    if substep_delta == 0.0 {
      println!("0 DELTA!!!")
    }
        // Amongus :) ඞ
   for _ in 0..self.substeps {
      self.do_substep(substep_delta);
    }
  }
}
