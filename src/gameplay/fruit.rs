use rand::{rng, Rng};

use crate::cells::{
    cell::{Cell, CellType},
    cell_group::CellGroup,
    cell_matrix::CellMatrix,
    color::Color,
    vector::Vector,
};

pub struct Fruit {
    cell_group: CellGroup,
}

impl Fruit {
    pub fn new(
        cell_matrix: &CellMatrix,
        gameplay_area_origin: &Vector<u16>,
        gameplay_area_extension: &Vector<u16>,
    ) -> Fruit {
        let position;
        let mut cell_group = CellGroup::new();

        loop {
            let mut rng = rng();
            let x = rng.random_range(gameplay_area_origin.x()..gameplay_area_extension.x());
            let y = rng.random_range(gameplay_area_origin.y()..gameplay_area_extension.y());

            let possible_position = Vector::<u16>::new(x, y);

            if cell_matrix
                .get_cell(&possible_position)
                .unwrap()
                .cell_type()
                == CellType::Empty
            {
                position = possible_position;
                cell_group.set_cell(
                    position.clone(),
                    Cell::new(
                        '■',
                        Color::Black.to_rgb(),
                        Color::Red.to_rgb(),
                        CellType::Fruit,
                    ),
                );
                break;
            }
        }

        return Fruit { cell_group };
    }

    pub fn none() -> Fruit {
        return Fruit {
            cell_group: CellGroup::new(),
        };
    }

    pub fn render(&mut self, cell_matrix: &mut CellMatrix) {
        self.cell_group.render(cell_matrix);
    }
}
