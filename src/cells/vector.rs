#[derive(Eq, Hash, PartialEq, Clone)]
pub struct VectorU16 {
    x: u16,
    y: u16,
}

impl VectorU16 {
    pub fn new(x: u16, y: u16) -> VectorU16 {
        return VectorU16 { x, y };
    }

    pub fn zero() -> VectorU16 {
        return VectorU16 { x: 0, y: 0 };
    }

    pub fn x(&self) -> u16 {
        return self.x;
    }

    pub fn y(&self) -> u16 {
        return self.y;
    }
}
