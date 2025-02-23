use crate::cells::{cell_group::CellGroup, cell_matrix::CellMatrix, vector::Vector};

#[allow(dead_code)]
#[derive(Clone)]
pub enum Alignment {
    TopLeft,
    Top,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    Bottom,
    BottomRight,
}

pub struct UiElement {
    name: String,
    aligned_position: Vector<u16>,
    width: u16,
    height: u16,
    cell_group: CellGroup,
}

impl UiElement {
    pub fn new(
        name: String,
        position: Vector<i32>,
        alignment: Alignment,
        width: u16,
        height: u16,
        cell_matrix_width: u16,
        cell_matrix_height: u16,
    ) -> UiElement {
        let x_offset = match alignment {
            Alignment::TopLeft | Alignment::CenterLeft | Alignment::BottomLeft => 0,
            Alignment::Top | Alignment::Center | Alignment::Bottom => {
                (cell_matrix_width - width) / 2
            }
            Alignment::TopRight | Alignment::CenterRight | Alignment::BottomRight => {
                cell_matrix_width - width
            }
        };

        let y_offset = match alignment {
            Alignment::TopLeft | Alignment::Top | Alignment::TopRight => 0,
            Alignment::CenterLeft | Alignment::Center | Alignment::CenterRight => {
                (cell_matrix_height - height) / 2
            }
            Alignment::BottomLeft | Alignment::Bottom | Alignment::BottomRight => {
                cell_matrix_height - height
            }
        };

        UiElement {
            name,
            aligned_position: Vector::<u16>::new(
                (x_offset as i32 + position.x()) as u16,
                (y_offset as i32 + position.y()) as u16,
            ),
            width,
            height,
            cell_group: CellGroup::new(),
        }
    }

    pub fn name(&self) -> String {
        return self.name.clone();
    }

    pub fn aligned_position(&self) -> Vector<u16> {
        return self.aligned_position.clone();
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn cell_group_mut(&mut self) -> &mut CellGroup {
        return &mut self.cell_group;
    }

    pub fn render(&mut self, cell_matrix: &mut CellMatrix) {
        self.cell_group.render(cell_matrix);
    }
}
