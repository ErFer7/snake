use crate::cell::{Cell, CellType};

pub struct CellMatrix {
    matrix: Vec<Cell>,
    width: u16,
    height: u16,
}

impl CellMatrix {
    pub fn new(width: u16, height: u16) -> CellMatrix {
        let mut matrix = Vec::with_capacity((width * height) as usize);

        for _ in 0..height {
            for _ in 0..width {
                matrix.push(Cell::new_empty());
            }
        }

        return CellMatrix {
            matrix,
            width,
            height,
        };
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn set_cell(&mut self, x: u16, y: u16, cell: Cell) {
        let index = self.get_index(x, y);
        self.matrix[index] = cell;
    }

    pub fn get_cell(&self, x: u16, y: u16) -> Option<&Cell> {
        if !self.is_in_bounds(x, y) {
            return None;
        }

        let index = self.get_index(x, y);
        Some(&self.matrix[index])
    }

    fn get_index(&self, x: u16, y: u16) -> usize {
        return (y * self.width + x) as usize;
    }

    fn is_in_bounds(&self, x: u16, y: u16) -> bool {
        x < self.width && y < self.height
    }

    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self.get_cell(x, y).unwrap();
                cell.print(x, y);
            }
        }
    }

    pub fn clear_type(&mut self, cell_type: CellType) {
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self.get_cell(x, y).unwrap();

                if cell.cell_type() == cell_type {
                    self.set_cell(x, y, Cell::new_empty());
                }
            }
        }
    }

    pub fn clear(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.set_cell(x, y, Cell::new_empty());
            }
        }
    }
}
