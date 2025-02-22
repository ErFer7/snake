use crate::cells::cell::Cell;
use crate::cells::cell_group::CellGroup;
use crate::cells::cell_matrix::CellMatrix;
use crate::cells::vector::VectorU16;

pub struct Wall {
    cell_group: CellGroup,
}

impl Wall {
    pub fn new(width: u16, height: u16) -> Wall {
        let mut cell_group = CellGroup::new();

        cell_group.set_cell(VectorU16::zero(), Cell::new_colorless('┏'));
        cell_group.set_cell(VectorU16::new(width - 1, 0), Cell::new_colorless('┓'));
        cell_group.set_cell(VectorU16::new(0, height - 1), Cell::new_colorless('┗'));
        cell_group.set_cell(
            VectorU16::new(width - 1, height - 1),
            Cell::new_colorless('┛'),
        );

        for x in 1..width - 1 {
            cell_group.set_cell(VectorU16::new(x, 0), Cell::new_colorless('━'));
            cell_group.set_cell(VectorU16::new(x, height - 1), Cell::new_colorless('━'));
        }

        for y in 1..height - 1 {
            cell_group.set_cell(VectorU16::new(0, y), Cell::new_colorless('┃'));
            cell_group.set_cell(VectorU16::new(width - 1, y), Cell::new_colorless('┃'));
        }

        return Wall { cell_group };
    }

    pub fn render(&mut self, cell_matrix: &mut CellMatrix) {
        self.cell_group.render(cell_matrix);
    }
}
