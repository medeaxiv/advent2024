use aoc_utils::direction::Direction;

#[derive(Clone, Copy)]
pub struct Path {
    len: usize,
    steps: [Direction; Self::SIZE],
}

impl Path {
    const SIZE: usize = 8;

    pub const fn new() -> Self {
        Self {
            len: 0,
            steps: [Direction::Up; 8],
        }
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub fn as_slice(&self) -> &[Direction] {
        &self.steps[0..self.len]
    }

    pub fn movement(&mut self, direction: Direction, distance: u32) -> Result<(), PathError> {
        let new_len = self.len + distance as usize;
        if new_len >= Self::SIZE {
            return Err(PathError::MovementTooLong);
        }

        for i in self.len..new_len {
            self.steps[i] = direction;
        }

        self.len = new_len;

        Ok(())
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl Eq for Path {}

impl std::hash::Hash for Path {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state)
    }
}

impl std::ops::Deref for Path {
    type Target = [Direction];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl std::fmt::Debug for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathError {
    #[error("Movement is too long to fit in the path")]
    MovementTooLong,
}
