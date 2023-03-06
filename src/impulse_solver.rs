use crate::bounds::{Boundable, Circle};
use crate::verlet_object::*;
use raylib::prelude::*;
use rayon::prelude::*;

pub struct ImpulseSolver {
  pub objects: Vec<VerletObject>,
  pub substeps: u32,
  pub bounds: Box<dyn Boundable>,
  pub timestep: f32,
  pub current_time: f32,
  pub response_mod: f32,
  pub gravity: Vector2,
}

impl Default for ImpulseSolver {
  fn default() -> Self {
    Self {
      objects: vec![],
      substeps: 8,
      // bounds: Box::new(Rectangle {
      //   x: 0.0,
      //   y: 0.0,
      //   width: 1000.0,
      //   height: 1000.0,
      // }),
      bounds: Box::new(Circle {
        x: 250.0,
        y: 250.0,
        radius: 250.0,
      }),
      timestep: 1.0 / 60.0,
      current_time: 0.0,
      response_mod: 0.5,
      gravity: Vector2::new(0.0, 1000.0),
    }
  }
}

impl ImpulseSolver {
  pub fn new() -> Self {
    ImpulseSolver::default()
  }

  pub fn add_circle(&mut self, mut circle: VerletObject) {
    circle = self.bounds.restrict(circle);
    circle.last_position = circle.position;
    self.objects.push(circle);
  }

  pub fn count_circles(&self) -> usize {
    self.objects.len()
  }

  pub fn would_collide(&self, test: &VerletObject) -> bool {
    let test = self.bounds.restrict(*test);
    for other in &self.objects {
      if (test.position.x - other.position.x).powi(2)
        + (test.position.y - other.position.y).powi(2)
        < DIAMETER_SQUARED
      {
        return true;
      }
    }
    false
  }

  pub fn find_collisions(&mut self) {
    for circle_index in 0..self.objects.len() {
      // Apply gravity
      self.objects[circle_index].acceleration += self.gravity;
      for other_index in circle_index..self.objects.len() {
        let circle = &self.objects[circle_index].position;
        let other = &self.objects[other_index].position;
        let minimum_distance = DIAMETER;
        if circle.x < other.x - minimum_distance {
          break; // No further collisions possible
        }
        let dy = (circle.y - other.y).abs();
        if dy >= minimum_distance {
          continue; // Skip over obvious noncollisions
        }
        let combined = *circle - *other;
        let distance_squared = combined.length_sqr();
        if distance_squared >= DIAMETER_SQUARED || distance_squared == 0.0 {
          continue;
        }
        // Finally, resort to expensive calculation
        let distance = distance_squared.sqrt();
        let normalized = combined.scale_by(1.0 / distance);
        let delta = 0.5 * self.response_mod * (distance - minimum_distance);
        self.objects[circle_index].position -= normalized * delta * 0.5;
        self.objects[other_index].position += normalized * delta * 0.5;
      }
    }
  }

  pub fn move_objects(&mut self, delta: f32) {
    for o in &mut self.objects {
      o.update(delta);
      *o = self.bounds.restrict(*o);
    }
  }

  fn sort_objects(&mut self) {
    self.objects.par_sort_unstable_by(|a, b| {
      a.position.x.partial_cmp(&b.position.x).unwrap()
    })
  }

  fn do_substep(&mut self, delta: f32) {
    self.find_collisions();
    self.move_objects(delta);
    self.sort_objects();
  }

  pub fn do_step(&mut self) {
    let substep_delta = self.timestep / (self.substeps as f32);
    for _ in 0..self.substeps {
      self.do_substep(substep_delta);
    }
    self.current_time += self.timestep;
  }
}
