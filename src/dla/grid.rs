use crate::{create_cell, Cell};
use array2d::Array2D;
use rand::Rng;
use std::cmp::{max, min};
use std::ops::Range;

pub struct Grid {
  width: u32,
  height: u32,
  desired_active: u16,
  pub cells: Array2D<Option<Cell>>,
  pub active_cells: Vec<(usize, usize)>,
}

const DIRECTION: [(i8, i8); 8] = [
  (-1, -1),
  (0, -1),
  (1, -1),
  (-1, 0),
  (1, 0),
  (-1, 1),
  (0, 1),
  (1, 1),
];
fn clip(pos: (i32, i32), w: u32, h: u32) -> (usize, usize) {
  return (
    max(0, min(w as i32 - 1, pos.0)) as usize,
    max(0, min(h as i32 - 1, pos.1)) as usize,
  );
}
impl Grid {
  pub fn new(width: u32, height: u32, count: u16) -> Grid {
    let mut new_grid = Grid {
      width,
      height,
      desired_active: count,
      cells: Array2D::filled_with(None, width as usize, height as usize),
      active_cells: Vec::new(),
    };
    for _i in 0..count {
      new_grid.add_cell(false);
    }
    for _i in 0..count / 2 {
      new_grid.add_cell(true);
    }
    return new_grid;
  }
  fn add_cell(&mut self, is_frozen: bool) {
    let cell = create_cell(self.width, self.height, is_frozen);
    let x = cell.x as usize;
    let y = cell.y as usize;
    self.cells[(x, y)] = Some(cell);
    if !is_frozen {
      self.active_cells.push((x, y));
    }
  }

  fn is_frozen(&self, cell: (usize, usize)) -> Option<&Cell> {
    let mut result = None;
    for dir in DIRECTION {
      let (x, y) = dir;
      let pos = clip(
        ((cell.0 as i32 + x as i32), (cell.1 as i32 + y as i32)),
        self.width,
        self.height,
      );
      match self.cells[pos].as_ref() {
        Some(r) => {
          result = Some(r);
        }
        None => {}
      }
    }
    return result;
  }

  pub fn cycle(&mut self) {
    let mut rng = rand::thread_rng();
    let mut next_active = Vec::new();
    let mut add_count = 0;
    for (x, y) in &self.active_cells {
      let mut frozen = None;
      if let Some(f) = self.is_frozen((*x, *y)) {
        println!("frozen at {:?}", (*x, *y));
        frozen = Some(f.color);
      }
      if let Some(active) = &mut self.cells[(*x, *y)] {
        match frozen {
          Some(f) => {
            active.is_frozen = true;
            active.color = f.clone();
            add_count += 1;
          }
          None => {
            let dir_idx = rng.gen_range(Range { start: 0, end: 8 });
            let dir = DIRECTION[dir_idx];
            let new_pos = clip(
              (
                (active.x as i32 + dir.0 as i32),
                (active.y as i32 + dir.1 as i32),
              ),
              self.width,
              self.height,
            );
            active.x = new_pos.0;
            active.y = new_pos.1;
            next_active.push((active.x, active.y));
            self.cells[(new_pos.0, new_pos.1)] = Some(active.clone());
            self.cells[(*x, *y)] = None;
          }
        }
      }
    }
    self.active_cells = next_active;
    for _i in 0..(self.desired_active - (self.active_cells.len() as u16)) {
      self.add_cell(false);
    }
  }
}
