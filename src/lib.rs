mod utils;

extern crate web_sys;
use web_sys::console;

use wasm_bindgen::prelude::*;
use std::fmt;

// #[wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);
// }

// #[wasm_bindgen]
// pub fn greet(name: &str) {
//     alert(&format!("Hello, {}!", name));
// }

// #[wasm_bindgen]
// extern "C" {
//     // Use `js_namespace` here to bind `console.log(..)` instead of just
//     // `log(..)`
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(s: &str);

//     fn alert(s: &str);
// }

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
// macro_rules! log {
//     ( $( $t:tt )* ) => {
//         web_sys::console::log_1(&format!( $( $t )* ).into());
//     }
// }

// #[allow(unused_macros)]
// macro_rules! console_log {
//     // Note that this is using the `log` function imported above during
//     // `bare_bones`
//     ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
// }

pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        console::time_with_label(name);
        Timer { name }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        console::time_end_with_label(self.name);
    }
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
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells[idx].toggle();
    }

    // fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
    //     let mut count = 0;
    //     for delta_row in [self.height - 1, 0, 1].iter().cloned() {
    //         for delta_col in [self.width - 1, 0, 1].iter().cloned() {
    //             if delta_row == 0 && delta_col == 0 {
    //                 continue;
    //             }

    //             let neighbor_row = (row + delta_row) % self.height;
    //             let neighbor_col = (column + delta_col) % self.width;
    //             let idx = self.get_index(neighbor_row, neighbor_col);
    //             count += self.cells[idx] as u8;
    //         }
    //     }
    //     count
    // }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
    
        let north = if row == 0 {
            self.height - 1
        } else {
            row - 1
        };
    
        let south = if row == self.height - 1 {
            0
        } else {
            row + 1
        };
    
        let west = if column == 0 {
            self.width - 1
        } else {
            column - 1
        };
    
        let east = if column == self.width - 1 {
            0
        } else {
            column + 1
        };
    
        let nw = self.get_index(north, west);
        count += self.cells[nw] as u8;
    
        let n = self.get_index(north, column);
        count += self.cells[n] as u8;
    
        let ne = self.get_index(north, east);
        count += self.cells[ne] as u8;
    
        let w = self.get_index(row, west);
        count += self.cells[w] as u8;
    
        let e = self.get_index(row, east);
        count += self.cells[e] as u8;
    
        let sw = self.get_index(south, west);
        count += self.cells[sw] as u8;
    
        let s = self.get_index(south, column);
        count += self.cells[s] as u8;
    
        let se = self.get_index(south, east);
        count += self.cells[se] as u8;
    
        count
    }

    pub fn tick(&mut self) {
        let _timer = Timer::new("Universe::tick");
    
        let mut next = {
            let _timer = Timer::new("allocate next cells");
            self.cells.clone()
        };
    
        {
            let _timer = Timer::new("new generation");
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
                        // All other cells remain in the same state.
                        (otherwise, _) => otherwise,
                    };
    
                    next[idx] = next_cell;
                }
            }
        }
    
        let _timer = Timer::new("free old cells");
        self.cells = next;
    }

    pub fn new() -> Universe {
        utils::set_panic_hook();

        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
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

    /// Set the width of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect();
    }

    /// Set the height of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height).map(|_i| Cell::Dead).collect();
    }
}

impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        };
    }
}

impl Universe {

    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }
}

























// Rayon Multithread Version

// mod utils;
// extern crate web_sys;

// use wasm_bindgen::prelude::*;
// use web_sys::console;
// use rayon::prelude::*;
// use std::fmt;

// pub use wasm_bindgen_rayon::init_thread_pool;

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(s: &str);
// }

// macro_rules! console_log {
//     ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
// }

// pub struct Timer<'a> {
//     name: &'a str,
// }
// impl<'a> Timer<'a> {
//     pub fn new(name: &'a str) -> Timer<'a> {
//         console::time_with_label(name);
//         Timer { name }
//     }
// }
// impl<'a> Drop for Timer<'a> {
//     fn drop(&mut self) {
//         console::time_end_with_label(self.name);
//     }
// }

// #[wasm_bindgen]
// #[repr(u8)]
// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub enum Cell {
//     Dead = 0,
//     Alive = 1,
// }

// #[wasm_bindgen]
// pub struct Universe {
//     width: u32,
//     height: u32,
//     cells: Vec<Cell>,
// }

// impl fmt::Display for Universe {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         for line in self.cells.as_slice().chunks(self.width as usize) {
//             for &cell in line {
//                 let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
//                 write!(f, "{}", symbol)?;
//             }
//             write!(f, "\n")?;
//         }
//         Ok(())
//     }
// }

// #[wasm_bindgen]
// impl Universe {
//     pub fn new() -> Universe {
//         let width = 64;
//         let height = 64;
//         let cells = (0..width * height)
//             .map(|i| {
//                 if i % 2 == 0 || i % 7 == 0 {
//                     Cell::Alive
//                 } else {
//                     Cell::Dead
//                 }
//             })
//             .collect();

//         Universe {
//             width,
//             height,
//             cells,
//         }
//     }

//     fn get_index(&self, row: u32, column: u32) -> usize {
//         (row * self.width + column) as usize
//     }

//     fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
//         let mut count = 0;

//         let north = if row == 0 { self.height - 1 } else { row - 1 };
//         let south = if row == self.height - 1 { 0 } else { row + 1 };
//         let west = if column == 0 { self.width - 1 } else { column - 1 };
//         let east = if column == self.width - 1 { 0 } else { column + 1 };

//         let neighbors = [
//             self.get_index(north, west),
//             self.get_index(north, column),
//             self.get_index(north, east),
//             self.get_index(row, west),
//             self.get_index(row, east),
//             self.get_index(south, west),
//             self.get_index(south, column),
//             self.get_index(south, east),
//         ];

//         for &idx in neighbors.iter() {
//             count += self.cells[idx] as u8;
//         }

//         count
//     }

//     pub fn tick(&mut self) {
//         let _timer = Timer::new("Universe::tick");

//         let next = (0..self.width * self.height)
//             .into_par_iter()
//             .map(|i| {
//                 let row = i / self.width;
//                 let col = i % self.width;
//                 let idx = self.get_index(row, col);
//                 let cell = self.cells[idx];
//                 let live_neighbors = self.live_neighbor_count(row, col);

//                 match (cell, live_neighbors) {
//                     (Cell::Alive, x) if x < 2 => Cell::Dead,
//                     (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
//                     (Cell::Alive, x) if x > 3 => Cell::Dead,
//                     (Cell::Dead, 3) => Cell::Alive,
//                     (otherwise, _) => otherwise,
//                 }
//             })
//             .collect();

//         self.cells = next;
//     }

//     pub fn render(&self) -> String {
//         self.to_string()
//     }

//     pub fn width(&self) -> u32 {
//         self.width
//     }

//     pub fn height(&self) -> u32 {
//         self.height
//     }

//     // Return raw pointer to cells so JS can access the memory directly
//     pub fn cells_ptr(&self) -> *const Cell {
//         self.cells.as_ptr()
//     }

//     // Return cells length for JS to know the buffer size
//     pub fn cells_len(&self) -> usize {
//         self.cells.len()
//     }

//     // Toggle a cell at (row, col)
//     pub fn toggle_cell(&mut self, row: u32, column: u32) {
//         let idx = self.get_index(row, column);
//         self.cells[idx].toggle();
//     }

//     // Set cells from a flat array of u32: [row1, col1, row2, col2, ...]
//     // Note: Accept flat arrays, not tuples
//     pub fn set_cells(&mut self, cells: &[u32]) {
//         assert!(cells.len() % 2 == 0, "cells slice length must be even");
//         for chunk in cells.chunks(2) {
//             let row = chunk[0];
//             let col = chunk[1];
//             let idx = self.get_index(row, col);
//             self.cells[idx] = Cell::Alive;
//         }
//     }
// }

// impl Cell {
//     fn toggle(&mut self) {
//         *self = match *self {
//             Cell::Dead => Cell::Alive,
//             Cell::Alive => Cell::Dead,
//         };
//     }
// }