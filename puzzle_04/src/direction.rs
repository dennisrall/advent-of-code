#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Direction {
    Up,
    Down,
    Forward,
    Backward,
    Upforward,
    Upbackward,
    Downforward,
    Downbackward,
}

impl Direction {
    pub fn get_vector(&self) -> (isize, isize) {
        match self {
            Self::Up => (-1, 0),
            Self::Upforward => (-1, 1),
            Self::Forward => (0, 1),
            Self::Downforward => (1, 1),
            Self::Down => (1, 0),
            Self::Downbackward => (1, -1),
            Self::Backward => (0, -1),
            Self::Upbackward => (-1, -1),
        }
    }

    pub fn variants() -> Vec<Self> {
        vec![
            Self::Up,
            Self::Upforward,
            Self::Forward,
            Self::Downforward,
            Self::Down,
            Self::Downbackward,
            Self::Backward,
            Self::Upbackward,
        ]
    }
}
