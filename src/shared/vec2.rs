use std::ops::{Add, AddAssign, Mul, Rem, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vec2 {
    pub x: i64,
    pub y: i64,
}

impl Vec2 {
    pub const ZERO: Vec2 = Vec2 { x: 0, y: 0 };

    pub const NORTH: Vec2 = Vec2 { x: 0, y: -1 };
    pub const EAST: Vec2 = Vec2 { x: 1, y: 0 };
    pub const SOUTH: Vec2 = Vec2 { x: 0, y: 1 };
    pub const WEST: Vec2 = Vec2 { x: -1, y: 0 };

    pub const CARDINAL_DIRECTIONS: [Vec2; 4] = [Vec2::NORTH, Vec2::EAST, Vec2::SOUTH, Vec2::WEST];

    pub fn dir_ordinal(&self) -> u32 {
        match self {
            Vec2 { x: 0, y: -1 } => 0,
            Vec2 { x: 1, y: 0 } => 1,
            Vec2 { x: 0, y: 1 } => 2,
            Vec2 { x: -1, y: 0 } => 3,
            _ => unimplemented!(),
        }
    }

    pub fn rotate_cw(&self) -> Vec2 {
        Vec2 {
            x: -self.y,
            y: self.x,
        }
    }

    pub fn rotate_ccw(&self) -> Vec2 {
        Vec2 {
            x: self.y,
            y: -self.x,
        }
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<Vec2> for i64 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl Rem for Vec2 {
    type Output = Vec2;

    fn rem(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x.rem_euclid(rhs.x),
            y: self.y.rem_euclid(rhs.y),
        }
    }
}
