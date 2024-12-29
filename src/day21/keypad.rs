use aoc_utils::{
    direction::{Direction, Orientation},
    nalgebra, AocError,
};

use super::path::Path;

pub type Vec2 = nalgebra::Vector2<i32>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Numpad;

impl Numpad {
    pub fn path_between(a: NumpadButton, b: NumpadButton) -> Path {
        let a = a.coordinates();
        let b = b.coordinates();
        let offset = b - a;

        let horizontal = (
            Orientation::Horizontal.from_sign(offset.x),
            offset.x.unsigned_abs(),
        );

        let vertical = (
            Orientation::Vertical.from_sign(offset.y),
            offset.y.unsigned_abs(),
        );

        let mut path = Path::new();
        if a.y == 3 && b.x == 0 {
            path.movement(vertical.0, vertical.1)
                .expect("Path should be large enough");
            path.movement(horizontal.0, horizontal.1)
                .expect("Path should be large enough");
        } else if a.x == 0 && b.y == 3 {
            path.movement(horizontal.0, horizontal.1)
                .expect("Path should be large enough");
            path.movement(vertical.0, vertical.1)
                .expect("Path should be large enough");
        } else if horizontal.0 == Direction::Left {
            path.movement(horizontal.0, horizontal.1)
                .expect("Path should be large enough");
            path.movement(vertical.0, vertical.1)
                .expect("Path should be large enough");
        } else {
            /* horizontal.0 == Direction::Right */
            path.movement(vertical.0, vertical.1)
                .expect("Path should be large enough");
            path.movement(horizontal.0, horizontal.1)
                .expect("Path should be large enough");
        }

        path
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumpadButton {
    Number0,
    Number1,
    Number2,
    Number3,
    Number4,
    Number5,
    Number6,
    Number7,
    Number8,
    Number9,
    Accept,
}

impl NumpadButton {
    pub const fn coordinates(&self) -> Vec2 {
        match self {
            Self::Number0 => Vec2::new(1, 3),
            Self::Number1 => Vec2::new(0, 2),
            Self::Number2 => Vec2::new(1, 2),
            Self::Number3 => Vec2::new(2, 2),
            Self::Number4 => Vec2::new(0, 1),
            Self::Number5 => Vec2::new(1, 1),
            Self::Number6 => Vec2::new(2, 1),
            Self::Number7 => Vec2::new(0, 0),
            Self::Number8 => Vec2::new(1, 0),
            Self::Number9 => Vec2::new(2, 0),
            Self::Accept => Vec2::new(2, 3),
        }
    }

    pub const fn from_char(c: char) -> Result<Self, AocError> {
        match c {
            '0' => Ok(Self::Number0),
            '1' => Ok(Self::Number1),
            '2' => Ok(Self::Number2),
            '3' => Ok(Self::Number3),
            '4' => Ok(Self::Number4),
            '5' => Ok(Self::Number5),
            '6' => Ok(Self::Number6),
            '7' => Ok(Self::Number7),
            '8' => Ok(Self::Number8),
            '9' => Ok(Self::Number9),
            'A' => Ok(Self::Accept),
            _ => Err(AocError::InvalidInput),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dpad;

impl Dpad {
    pub fn path_between(a: DpadButton, b: DpadButton) -> Path {
        let a = a.coordinates();
        let b = b.coordinates();
        let offset = b - a;

        let horizontal = (
            Orientation::Horizontal.from_sign(offset.x),
            offset.x.unsigned_abs(),
        );

        let vertical = (
            Orientation::Vertical.from_sign(offset.y),
            offset.y.unsigned_abs(),
        );

        let mut path = Path::new();
        if b.x == 0 {
            path.movement(vertical.0, vertical.1)
                .expect("Path should be large enough");
            path.movement(horizontal.0, horizontal.1)
                .expect("Path should be large enough");
        } else if a.x == 0 {
            path.movement(horizontal.0, horizontal.1)
                .expect("Path should be large enough");
            path.movement(vertical.0, vertical.1)
                .expect("Path should be large enough");
        } else if horizontal.0 == Direction::Left {
            path.movement(horizontal.0, horizontal.1)
                .expect("Path should be large enough");
            path.movement(vertical.0, vertical.1)
                .expect("Path should be large enough");
        } else {
            /* horizontal.0 == Direction::Right */
            path.movement(vertical.0, vertical.1)
                .expect("Path should be large enough");
            path.movement(horizontal.0, horizontal.1)
                .expect("Path should be large enough");
        }

        path
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DpadButton {
    Direction(Direction),
    Accept,
}

impl DpadButton {
    pub const fn coordinates(&self) -> Vec2 {
        match self {
            Self::Direction(Direction::Left) => Vec2::new(0, 1),
            Self::Direction(Direction::Right) => Vec2::new(2, 1),
            Self::Direction(Direction::Up) => Vec2::new(1, 0),
            Self::Direction(Direction::Down) => Vec2::new(1, 1),
            Self::Accept => Vec2::new(2, 0),
        }
    }
}
