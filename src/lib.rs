mod utils;

extern crate js_sys;

use wasm_bindgen::prelude::*;
use std::fmt;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// A single conway cell.
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    pub fn rand() -> Cell {
        if js_sys::Math::random() > 0.5 {
            Cell::Alive
        } else {
            Cell::Dead
        }
    }
}

#[wasm_bindgen]
/// Grid our game of life cells live on
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

// private functions used internaly
impl Universe {
    ///return the flat-Vector index of a cell on the grid.
     /// 
     /// # Arguments
     /// 
     /// * `row` - the row number of the cell in question
     /// * `column` - the column number of the cell in question
    fn get_index(&self, row: u32, column:u32) -> usize {
        (row * self.width + column) as usize
    }
    /// returns the number of 'live' neighbors of a given cell
     /// 
     /// # Arguments
     /// 
     /// * `row` - the row number of the cell in question
     /// * `column` - the column number of the cell in question
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    /// Get the dead and alive valuse of the entire universe
    pub fn get_cells(&self) ->&[Cell] {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
     /// of each cell as an array.
     /// cells : array of tuples (row, col)
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }

}

#[wasm_bindgen]
/// Javascript facing functions.
impl Universe {
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);
                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state
                    (otherwise, _) => otherwise,
                };
                next[idx] = next_cell;
            }
        }
        self.cells = next;
    }

    /// Creates a default 64/64 universe  
     /// with a pattern of live/dead cells.
    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 { Cell::Alive }
                else { Cell::Dead }
            })
            .collect();
        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn new_empty() -> Universe {
        let width: u32 = 64;
        let height: u32 = 64;

        let cells = vec![Cell::Dead; (width as usize)*(height as usize)];

        Universe {
            height,
            width,
            cells,
        }
    }

    pub fn new_rand() -> Universe {
        let height: u32 = 64;
        let width: u32 = 64;

        let cells = (0..width*height)
            .map(|_| Cell::rand() )
            .collect();

        Universe {
            height,
            width,
            cells,
        }
    }

    pub fn new_lone_glider() -> Universe {
        let height: u32 = 64;
        let width: u32 = 64;

        let mut cells: Vec<Cell> = (0..width*height)
            .map(|_| Cell::Dead )
            .collect();

        let live_cells = [
            // [row, col]
            [1, 3],
            [2, 3],
            [3, 3],
            [2, 1],
            [3, 2],
        ];

        for [row, col] in live_cells {
            cells[(row*width + col)as usize] = Cell::Alive;
        }
        
        Universe {
            height,
            width,
            cells,
        }
    }


    pub fn width(&self) -> u32 {
        self.width
    }
    /// Set the width of the universe
     /// Resets all cells to a dead state
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect();
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    /// Set the height of the universe.
     /// Resets all cells to the dead state.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height).map(|_i| Cell::Dead).collect();
    }
    
    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Universe {
    /// Output the state of the univers in a human readable format
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}
