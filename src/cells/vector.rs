use num_traits::PrimInt;

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct Vector<T>
where
    T: PrimInt,
{
    x: T,
    y: T,
}

impl<T> Vector<T>
where
    T: PrimInt,
{
    pub fn new(x: T, y: T) -> Vector<T> {
        return Vector { x, y };
    }

    pub fn zero() -> Vector<T> {
        return Vector {
            x: T::zero(),
            y: T::zero(),
        };
    }

    pub fn x(&self) -> T {
        return self.x;
    }

    pub fn set_x(&mut self, x: T) {
        self.x = x;
    }

    pub fn y(&self) -> T {
        return self.y;
    }

    pub fn set_y(&mut self, y: T) {
        self.y = y;
    }
}
