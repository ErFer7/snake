use crate::cells::{cell_group::CellGroup, cell_matrix::CellMatrix, vector::Vector};

#[allow(dead_code)]
#[derive(Clone)]
pub enum Orientation {
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
    position: Vector<i32>,
    aligned_position: Vector<u16>,
    anchor: Orientation,
    alignment: Orientation,
    width: u16,
    height: u16,
    cell_matrix_width: u16,
    cell_matrix_height: u16,
    cell_group: CellGroup,
}

impl UiElement {
    pub fn new(
        name: String,
        position: Vector<i32>,
        anchor: Orientation,
        alignment: Orientation,
        width: u16,
        height: u16,
        cell_matrix_width: u16,
        cell_matrix_height: u16,
    ) -> UiElement {
        let x_offset = calculated_x_offset(&anchor, &alignment, cell_matrix_width, width);
        let y_offset = calculated_y_offset(&anchor, &alignment, cell_matrix_height, height);
        let aligned_position = Vector::<u16>::new(
            (x_offset as i32 + position.x()) as u16,
            (y_offset as i32 + position.y()) as u16,
        );

        return UiElement {
            name,
            position,
            anchor,
            aligned_position,
            alignment,
            width,
            height,
            cell_matrix_width,
            cell_matrix_height,
            cell_group: CellGroup::new(),
        };
    }

    pub fn name(&self) -> String {
        return self.name.clone();
    }

    pub fn aligned_position(&self) -> Vector<u16> {
        return self.aligned_position.clone();
    }

    pub fn width(&self) -> u16 {
        return self.width;
    }

    pub fn set_width(&mut self, width: u16) {
        self.width = width;

        let x_offset = calculated_x_offset(
            &self.anchor,
            &self.alignment,
            self.cell_matrix_width,
            self.width,
        );

        self.aligned_position
            .set_x((x_offset as i32 + self.position.x()) as u16);
    }

    pub fn height(&self) -> u16 {
        return self.height;
    }

    pub fn set_height(&mut self, height: u16) {
        self.height = height;

        let y_offset = calculated_y_offset(
            &self.anchor,
            &self.alignment,
            self.cell_matrix_height,
            self.height,
        );

        self.aligned_position
            .set_y((y_offset as i32 + self.position.y()) as u16);
    }

    pub fn cell_group_mut(&mut self) -> &mut CellGroup {
        return &mut self.cell_group;
    }

    pub fn render(&mut self, cell_matrix: &mut CellMatrix) {
        self.cell_group.render(cell_matrix);
    }
}

fn calculated_x_offset(
    anchor: &Orientation,
    orientation: &Orientation,
    cell_matrix_width: u16,
    width: u16,
) -> u16 {
    let anchor_offset = match anchor {
        Orientation::TopLeft | Orientation::CenterLeft | Orientation::BottomLeft => 0,
        Orientation::Top | Orientation::Center | Orientation::Bottom => {
            cell_matrix_width.div_ceil(2)
        }
        Orientation::TopRight | Orientation::CenterRight | Orientation::BottomRight => {
            cell_matrix_width
        }
    };

    let orientation_offset = match orientation {
        Orientation::TopLeft | Orientation::CenterLeft | Orientation::BottomLeft => 0,
        Orientation::Top | Orientation::Center | Orientation::Bottom => width.div_ceil(2),
        Orientation::TopRight | Orientation::CenterRight | Orientation::BottomRight => width,
    };

    return if anchor_offset >= orientation_offset {
        anchor_offset - orientation_offset
    } else {
        0
    };
}

fn calculated_y_offset(
    anchor: &Orientation,
    orientation: &Orientation,
    cell_matrix_height: u16,
    height: u16,
) -> u16 {
    let anchor_offset = match anchor {
        Orientation::TopLeft | Orientation::Top | Orientation::TopRight => 0,
        Orientation::CenterLeft | Orientation::Center | Orientation::CenterRight => {
            cell_matrix_height.div_ceil(2)
        }
        Orientation::BottomLeft | Orientation::Bottom | Orientation::BottomRight => {
            cell_matrix_height
        }
    };

    let orientation_offset = match orientation {
        Orientation::TopLeft | Orientation::Top | Orientation::TopRight => 0,
        Orientation::CenterLeft | Orientation::Center | Orientation::CenterRight => {
            height.div_ceil(2)
        }
        Orientation::BottomLeft | Orientation::Bottom | Orientation::BottomRight => height,
    };

    return if anchor_offset >= orientation_offset {
        anchor_offset - orientation_offset
    } else {
        0
    };
}
