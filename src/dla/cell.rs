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

pub fn create_cell(w: u32, h: u32, is_frozen: bool) -> Cell {
  let mut rng = rand::thread_rng();

  Cell {
    x: rng.gen_range(Range {
      start: 0,
      end: w as usize,
    }),
    y: rng.gen_range(Range {
      start: 0,
      end: h as usize,
    }),
    is_frozen,
    color: Color::RGB(
      rng.gen_range(Range { start: 0, end: 234 }),
      rng.gen_range(Range { start: 0, end: 234 }),
      rng.gen_range(Range { start: 0, end: 234 }),
    ),
  }
}
