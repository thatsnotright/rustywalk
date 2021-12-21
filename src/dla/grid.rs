use crate::{create_cell, Cell};
use array2d::Array2D;
use rand::Rng;
use std::cmp::{max, min};
use std::ops::Range;

pub struct Grid {
  width: u32,
  height: u32,
  pub cells: Array2D<Option<Cell>>,
  active_cells: Vec<(usize, usize)>,
}

const DIRECTION: [(i8, i8); 9] = [
  (-1, -1),
  (0, -1),
  (1, -1),
  (-1, 0),
  (0, 0),
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
        ((cell.1 as i32 + y as i32), (cell.0 as i32 + x as i32)),
        self.width,
        self.height,
      );
      println!("pos {:?}", pos);
      match &self.cells[pos] {
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

    for (x, y) in &self.active_cells {
      let mut frozen = None;
      if let Some(f) = self.is_frozen((*x, *y)) {
        frozen = Some(f.color);
      }
      if let Some(active) = &mut self.cells[(*x, *y)] {
        match frozen {
          Some(f) => {
            active.is_frozen = true;
            active.color = f.clone();
          }
          None => {
            let dir = rng.gen_range(Range { start: 0, end: 9 });
            let dir = DIRECTION[dir];
            active.x = max(0, min(self.width as i32, (active.x as i32 + dir.0 as i32))) as usize;
            active.y = max(0, min(self.height as i32, (active.y as i32 + dir.1 as i32))) as usize;
          }
        }
      }
    }
  }
}
