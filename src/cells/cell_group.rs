use std::collections::HashMap;

use super::{
    cell::{Cell, CellType},
    cell_matrix::CellMatrix,
    vector::VectorU16,
};

pub struct CellGroup {
    cells: HashMap<VectorU16, Cell>,
}

impl CellGroup {
    pub fn new() -> CellGroup {
        return CellGroup {
            cells: HashMap::new(),
        };
    }

    pub fn set_cell(&mut self, position: VectorU16, cell: Cell) {
        self.cells.insert(position, cell);
    }

    pub fn render(&mut self, cell_matrix: &mut CellMatrix) {
        for (vector, cell) in &self.cells {
            cell_matrix.set_cell(vector.clone(), cell.clone());
        }

        self.strip_empty();
    }

    fn strip_empty(&mut self) {
        self.cells
            .retain(|_, cell| cell.cell_type() != CellType::Empty);
    }
}
