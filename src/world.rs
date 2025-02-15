use crate::cell::Cell;
use crate::cell_matrix;

pub fn generate_walls(cell_matrix: &mut cell_matrix::CellMatrix, width: u16, height: u16) {
    cell_matrix.set_cell(0, 0, Cell::new_colorless('┏'));
    cell_matrix.set_cell(width - 1, 0, Cell::new_colorless('┓'));
    cell_matrix.set_cell(0, height - 1, Cell::new_colorless('┗'));
    cell_matrix.set_cell(width - 1, height - 1, Cell::new_colorless('┛'));

    for x in 1..width - 1 {
        cell_matrix.set_cell(x, 0, Cell::new_colorless('━'));
        cell_matrix.set_cell(x, height - 1, Cell::new_colorless('━'));
    }

    for y in 1..height - 1 {
        cell_matrix.set_cell(0, y, Cell::new_colorless('┃'));
        cell_matrix.set_cell(width - 1, y, Cell::new_colorless('┃'));
    }
}
