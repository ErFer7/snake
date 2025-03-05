use crate::{cells::cell::Cell, core::terminal::Terminal};

use super::vector::Vector;

pub struct CellMatrix {
    matrix: Vec<Cell>,
    print_buffer: Vec<String>,
    width: u16,
    height: u16,
}

impl CellMatrix {
    pub fn new(width: u16, height: u16) -> CellMatrix {
        let mut matrix = Vec::with_capacity((width * height) as usize);
        let mut print_buffer = Vec::new();

        for y in 0..height {
            for x in 0..width {
                let empty_cell = Cell::new_empty();

                print_buffer.push(empty_cell.to_string(&Vector::<u16>::new(x, y)));
                matrix.push(empty_cell);
            }
        }

        return CellMatrix {
            matrix,
            print_buffer,
            width,
            height,
        };
    }

    pub fn width(&self) -> u16 {
        return self.width;
    }

    pub fn height(&self) -> u16 {
        return self.height;
    }

    pub fn get_cell(&self, position: &Vector<u16>) -> Option<&Cell> {
        if !self.is_in_bounds(position) {
            return None;
        }

        return Some(&self.matrix[self.get_index(position)]);
    }

    pub fn set_cell(&mut self, position: &Vector<u16>, cell: Cell) {
        if !self.is_in_bounds(position) {
            return;
        }

        let index = self.get_index(position);

        self.print_buffer.push(cell.to_string(position));
        self.matrix[index] = cell;
    }

    pub fn write(&mut self, terminal: &mut Terminal) {
        for cell_string in &self.print_buffer {
            terminal.write(cell_string);
        }

        self.print_buffer.clear();
    }

    pub fn clear(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.set_cell(&Vector::<u16>::new(x, y), Cell::new_empty());
            }
        }
    }

    fn get_index(&self, position: &Vector<u16>) -> usize {
        return (position.y() * self.width + position.x()) as usize;
    }

    fn is_in_bounds(&self, position: &Vector<u16>) -> bool {
        return position.x() < self.width && position.y() < self.height;
    }
}
