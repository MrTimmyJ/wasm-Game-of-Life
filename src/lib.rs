/* mod utils;

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

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    fn alert(s: &str);
}

#[allow(unused_macros)]
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

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

    // pub fn tick(&mut self) {
    //     let _timer = Timer::new("Universe::tick");

    //     let mut next = self.cells.clone();

    //     for row in 0..self.height {
    //         for col in 0..self.width {
    //             let idx = self.get_index(row, col);
    //             let cell = self.cells[idx];
    //             let live_neighbors = self.live_neighbor_count(row, col);

    //             let next_cell = match (cell, live_neighbors) {
    //                 // Rule 1: Any live cell with fewer than two live neighbours
    //                 // dies, as if caused by underpopulation.
    //                 (Cell::Alive, x) if x < 2 => Cell::Dead,
    //                 // Rule 2: Any live cell with two or three live neighbours
    //                 // lives on to the next generation.
    //                 (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
    //                 // Rule 3: Any live cell with more than three live
    //                 // neighbours dies, as if by overpopulation.
    //                 (Cell::Alive, x) if x > 3 => Cell::Dead,
    //                 // Rule 4: Any dead cell with exactly three live neighbours
    //                 // becomes a live cell, as if by reproduction.
    //                 (Cell::Dead, 3) => Cell::Alive,
    //                 // All other cells remain in the same state.
    //                 (otherwise, _) => otherwise,
    //             };

    //             next[idx] = next_cell;
    //         }
    //     }

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

        self.cells = next;
    }

    pub fn new() -> Universe {
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
} */

// Multithreading functionality

mod utils;
extern crate web_sys;
use web_sys::console;

use std::thread;
use std::sync::Arc;

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
#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    fn alert(s: &str);
}
#[allow(unused_macros)]
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}
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

const THREAD_COUNT: u8 = 2;

pub struct UniverseSlice<'a> {
    universe: &'a Universe,
    start_x: usize,
    end_x: usize,
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

    pub fn tick_slice(slice: &UniverseSlice) {
        let _timer = Timer::new("Universe::tick");

        let mut next = slice.cells.cells.clone();

        for row in 0..slice.height {
            for col in slice.start_x..slice.end_x {
                slice.universe.get_index(row, col);
                let idx = slice.cells.get_index(row, col);
                let cell = slice.cells.cells[idx];
                let live_neighbors = slice.cells.live_neighbor_count(row, col);

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

    pub fn tick(&mut self) {
        let mut handles = vec![2];

        for i in 0..THREAD_COUNT {
            let start_x = i * (self.width/THREAD_COUNT);
            let end_x = (i+1) (self.width/THREAD_COUNT);
            let slice = Arc::new(UniverseSlice{universe:self, start_x: start_x, end_x: end_x});

            let handle = thread::spawn(move || {
                Self::tick_slice(&slice);
            });
            handles.push(handle);
        }


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

// Code for Warm Cells
// mod utils;

// use wasm_bindgen::prelude::*;
// use std::fmt;

// // #[wasm_bindgen]
// // extern "C" {
// //     fn alert(s: &str);
// // }

// // #[wasm_bindgen]
// // pub fn greet(name: &str) {
// //     alert(&format!("Hello, {}!", name));
// // }

// #[wasm_bindgen]
// #[repr(u8)]
// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub enum Cell {
//     Dead = 0,
//     Alive = 1,
//     Warm = 2,
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
//     fn get_index(&self, row: u32, column: u32) -> usize {
//         (row * self.width + column) as usize
//     }

//     fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
//         let mut count = 0;
//         for delta_row in [self.height - 1, 0, 1].iter().cloned() {
//             for delta_col in [self.width - 1, 0, 1].iter().cloned() {
//                 if delta_row == 0 && delta_col == 0 {
//                     continue;
//                 }

//                 let neighbor_row = (row + delta_row) % self.height;
//                 let neighbor_col = (column + delta_col) % self.width;
//                 let idx = self.get_index(neighbor_row, neighbor_col);
//                 if self.cells[idx] == Cell::Alive{
//                     count += 1 as u8;
//                 }
                
//             }
//         }
//         count
//     }

//     fn warm_neighbor_count(&self, row: u32, column: u32) -> u8 {
//         let mut count = 0;
//         for delta_row in [self.height - 1, 0, 1].iter().cloned() {
//             for delta_col in [self.width - 1, 0, 1].iter().cloned() {
//                 if delta_row == 0 && delta_col == 0 {
//                     continue;
//                 }

//                 let neighbor_row = (row + delta_row) % self.height;
//                 let neighbor_col = (column + delta_col) % self.width;
//                 let idx = self.get_index(neighbor_row, neighbor_col);
//                 if self.cells[idx] == Cell::Warm{
//                     count += 1 as u8;
//                 }
//             }
//         }
//         count
//     }

//     pub fn tick(&mut self) {
//         let mut next = self.cells.clone();

//         for row in 0..self.height {
//             for col in 0..self.width {
//                 let idx = self.get_index(row, col);
//                 let cell = self.cells[idx];
//                 let live_neighbors = self.live_neighbor_count(row, col);
//                 let warm_neighbors = self.warm_neighbor_count(row, col);

//                 let next_cell = match (cell, live_neighbors, warm_neighbors) {
//                     // Rule 1: Any live cell with fewer than two live neighbours
//                     // dies, as if caused by underpopulation.
//                     (Cell::Alive, x, _) if x < 2 => Cell::Warm,
//                     // Rule 2: Any live cell with two or three live neighbours
//                     // lives on to the next generation.
//                     (Cell::Alive, 2, _) | (Cell::Alive, 3, _) => Cell::Alive,
//                     // Rule 3: Any live cell with more than three live
//                     // neighbours dies, as if by overpopulation.
//                     // *Now goes to warm instead
//                     (Cell::Alive, x, _) if x > 3 => Cell::Warm,
//                     // Rule 4: Any dead cell with exactly three live neighbours
//                     // becomes a live cell, as if by reproduction.
//                     (Cell::Dead, 3, _) => Cell::Alive,
//                     // *RULE 5: Warm cells live on or die.
//                     (Cell::Warm, x, _) if x > 2 => Cell::Alive,
//                     (Cell::Warm, x, _) if x < 2 => Cell::Dead,
//                     // All other cells remain in the same state.
//                     (otherwise, _, _) => otherwise,
//                 };

//                 next[idx] = next_cell;
//             }
//         }

//         self.cells = next;
//     }

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

//     pub fn render(&self) -> String {
//         self.to_string()
//     }

//     pub fn width(&self) -> u32 {
//         self.width
//     }

//     pub fn height(&self) -> u32 {
//         self.height
//     }

//     pub fn cells(&self) -> *const Cell {
//         self.cells.as_ptr()
//     }

//     /// Set the width of the universe.
//     ///
//     /// Resets all cells to the dead state.
//     pub fn set_width(&mut self, width: u32) {
//         self.width = width;
//         self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect();
//     }

//     /// Set the height of the universe.
//     ///
//     /// Resets all cells to the dead state.
//     pub fn set_height(&mut self, height: u32) {
//         self.height = height;
//         self.cells = (0..self.width * height).map(|_i| Cell::Dead).collect();
//     }
// }