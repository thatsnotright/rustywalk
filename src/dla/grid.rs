use crate::{create_cell, Cell};
use array2d::Array2D;
use rand::Rng;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::ops::Range;

pub struct Grid {
    width: u32,
    height: u32,
    desired_active: u16,
    pub cells: Array2D<Option<Cell>>,
    pub active_cells: HashMap<(usize, usize), Cell>,
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
    let (x, y) = pos;
    let mut new_pos = (x as usize, y as usize);
    if x < 0 {
        new_pos.0 = (w as i32 - 1) as usize;
    }
    if x >= w as i32 {
        new_pos.0 = 0;
    }
    if y < 0 {
        new_pos.1 = (h as i32 - 1) as usize;
    }
    if y >= h as i32 {
        new_pos.1 = 0;
    }
    return new_pos;
}
impl Grid {
    pub fn new(width: u32, height: u32, count: u16) -> Grid {
        let mut new_grid = Grid {
            width,
            height,
            desired_active: count,
            cells: Array2D::filled_with(None, width as usize, height as usize),
            active_cells: HashMap::with_capacity(count as usize),
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
        if !is_frozen {
            self.active_cells.insert((x, y), cell);
        } else {
            self.cells[(x, y)] = Some(cell);
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
        let mut cells = std::mem::replace(&mut self.active_cells, HashMap::new());
        cells.drain_filter(|k, active_cell| {
            let mut frozen = None;
            if let Some(f) = self.is_frozen(*k) {
                frozen = Some(f.color);
            }
            match frozen {
                Some(f) => {
                    active_cell.is_frozen = true;
                    active_cell.color = f.clone();
                    self.cells[*k] = Some(active_cell.to_owned());
                    return true;
                }
                None => {
                    let dir_idx = rng.gen_range(Range { start: 0, end: 8 });
                    let dir = DIRECTION[dir_idx];
                    let new_pos = clip(
                        (
                            (active_cell.x as i32 + dir.0 as i32),
                            (active_cell.y as i32 + dir.1 as i32),
                        ),
                        self.width,
                        self.height,
                    );
                    active_cell.x = new_pos.0;
                    active_cell.y = new_pos.1;
                    return false;
                }
            }
        });
        for _i in 0..(self.desired_active - (self.active_cells.len() as u16)) {
            self.add_cell(false);
        }
    }
}
