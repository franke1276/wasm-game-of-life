extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;

use cfg_if::cfg_if;
use std::fmt;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}
#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}
#[wasm_bindgen]
pub struct Universe {
    run: bool,
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}
impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
#[wasm_bindgen]
impl Universe {
    fn new(w: u32, h: u32) -> Universe {
        Universe {
            width: w,
            height: h,
            run: false,
            cells: vec![Cell::Dead; (w * h) as usize],
        }
    }
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
    pub fn reset(&mut self) {
        self.cells = (0..self.width * self.height).map(|_i| Cell::Dead).collect();
        self.run = false;
    }
    pub fn create() -> Universe {
        let width = 64;
        let height = 64;
        // let cells = (0..width * height)
        //     .map(|i| {
        //         if i % 2 == 0 || i % 7 == 0 {
        //             Cell::Alive
        //         } else {
        //             Cell::Dead
        //         }
        //     }).collect();
        let cells = (0..width * height).map(|_i| Cell::Dead).collect();
        Universe {
            width,
            height,
            run: false,
            cells,
        }
    }
    pub fn render(&self) -> String {
        self.to_string()
    }
    pub fn stop(&mut self) {
        self.run = false;
    }
    pub fn generate_pattern(&mut self) {
        self.stop();
        self.cells = (0..self.width * self.height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            }).collect();
    }
    pub fn toggle_start_stop(&mut self) {
        self.run = !self.run;
    }
    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells[idx] = if self.cells[idx] == Cell::Alive {
            Cell::Dead
        } else {
            Cell::Alive
        }
    }
    fn set(&mut self, row: u32, column: u32, value: Cell) -> Result<(), String> {
        if row > self.height - 1 || column > self.width - 1 {
            Err(format!("row or column out of range {} {}", row, column))
        } else {
            let index = self.get_index(row, column);
            self.cells[index] = value;
            Ok(())
        }
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count: u8 = 0;
        for r in [self.height - 1, 0, 1].iter().cloned() {
            for c in [self.width - 1, 0, 1].iter().cloned() {
                if r == 0 && c == 0 {
                    continue;
                }
                let row_ = (row + r) % self.height();
                let column_ = (column + c) % self.width();
                let index = self.get_index(row_, column_);
                count += self.cells[index] as u8;
            }
        }
        count
    }
    pub fn tick(&mut self) {
        if self.run {
            let mut next = self.cells.clone();
            for r in 0..self.height {
                for c in 0..self.width {
                    let idx = self.get_index(r, c);
                    let new_cell = match (self.cells[idx], self.live_neighbor_count(r, c)) {
                        (Cell::Alive, i) if i < 2 => Cell::Dead,
                        (Cell::Alive, i) if i >= 2 && i <= 3 => Cell::Alive,
                        (Cell::Alive, i) if i > 3 => Cell::Dead,
                        (Cell::Dead, i) if i == 3 => Cell::Alive,
                        (other, _) => other,
                    };
                    next[idx] = new_cell;
                }
            }
            self.cells = next;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_test() {
        /*
          0 1 2 3
        0 . . . . 
        1 . O . . 
        2 . O . . 
        3 . O . . 
        */
        let mut universe = Universe::new(4, 4);
        universe.set(1, 1, Cell::Alive).unwrap();
        universe.set(2, 1, Cell::Alive).unwrap();
        universe.set(3, 1, Cell::Alive).unwrap();
        assert_eq!(2, universe.live_neighbor_count(0, 0));
        assert_eq!(2, universe.live_neighbor_count(0, 1));
        assert_eq!(2, universe.live_neighbor_count(0, 2));
        assert_eq!(0, universe.live_neighbor_count(0, 3));
        assert_eq!(3, universe.live_neighbor_count(2, 2));
        assert_eq!(2, universe.live_neighbor_count(3, 0));

        universe.tick();
        /*
          0 1 2 3
        0 . . . . 
        1 . O . . 
        2 . O O . 
        3 . O . . 
        */
        assert_eq!(Cell::Alive, universe.cells[universe.get_index(2, 2)]);
        assert_eq!(Cell::Alive, universe.cells[universe.get_index(1, 1)]);
        assert_eq!(Cell::Alive, universe.cells[universe.get_index(2, 1)]);
        assert_eq!(Cell::Alive, universe.cells[universe.get_index(3, 1)]);
    }
}
