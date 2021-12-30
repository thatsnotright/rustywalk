use crate::universe::actor::Actor;
use rand::Rng;
use sdl2::pixels::Color;
use std::ops::Range;

#[derive(Debug, Clone)]
pub struct Cell {
  pub x: usize,
  pub y: usize,
  pub color: Color,
  pub is_frozen: bool,
}

impl Actor for Cell {
  fn new(is_frozen: bool, w: usize, h: usize) -> Cell {
    let mut rng = rand::thread_rng();

    return Cell {
      x: rng.gen_range(Range { start: 0, end: w }),
      y: rng.gen_range(Range { start: 0, end: h }),
      is_frozen,
      color: Color::RGB(
        rng.gen_range(Range { start: 0, end: 234 }),
        rng.gen_range(Range { start: 0, end: 234 }),
        rng.gen_range(Range { start: 0, end: 234 }),
      ),
    };
  }
}
