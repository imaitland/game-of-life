mod utils;
use wasm_bindgen::prelude::*;
use std::fmt;
use js_sys; // for js Math.random()
use fixedbitset::FixedBitSet;
use web_sys; // for js console.log

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

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

    pub fn get_cells(&self) -> &FixedBitSet {
        &self.cells
    }

    pub fn set_cells(&mut self, cells: &[(u32,u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row,col);
            self.cells.set(idx, true);
        }

    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == 1 { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {

    pub fn width(&self) -> u32 { 
        self.width
    }

    pub fn height(&self) -> u32 { 
        self.height
    }

    pub fn set_width (&mut self, w: u32) {
        self.width = w;
    }
    pub fn set_height (&mut self, w: u32) {
        self.width = w;
    }

    pub fn reset_universe(&mut self) {
        let size = (self.width * self.height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);
        for i in 0..size {
            cells.set(i, false);
        }
        self.cells = cells
    }
    // To pass a pointer to the start of the bits to JavaScript, you can convert the FixedBitSet to a slice and then convert the slice to a pointer:
    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    pub fn toggle_cell(&mut self, row: u32, col: u32) {

        let idx = self.get_index(row, col);
        let currentState = self.cells[idx];

        match currentState {
            true => self.cells.set(idx, false),
            false => self.cells.set(idx, true)
        }
    }

    // Initializes a new universe, with random (50% total) living cells 
    pub fn new(width: u32, height: u32) -> Universe {
        utils::set_panic_hook();
        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);
        for i in 0..size {
            if js_sys::Math::random() < 0.5 {
                cells.set(i, true);
            } else {
                cells.set(i, false);
            }
            
        }

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                // Example using our macro of js console.log that we get through the web_sys crate.

                /*
                log!(
                    "Cell [{}, {}] is initially {:?} and has {} live neighbors.", 
                    row,
                    col,
                    cell,
                    live_neighbors
                );
                */

                next.set(idx, match (cell, live_neighbors) {
                    (true, x) if x < 2 => false,
                    (true, 2) | (true, 3) => true,
                    (true, x) if x > 3 => false,
                    (false, 3) => true,
                    (otherwise, _) => otherwise
                });
            }
        }

        self.cells = next;
    }
}