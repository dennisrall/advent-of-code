#[derive(Debug)]
pub enum Direction {
    UP,
    DOWN,
    FORWARD,
    BACKWARD,
    UPFORWARD,
    UPBACKWARD,
    DOWNFORWARD,
    DOWNBACKWARD,
}

impl Direction {
    pub fn get_vector(&self) -> (isize, isize) {
        match self {
            Self::UP => (1, 0),
            Self::UPFORWARD => (1, 1),
            Self::FORWARD => (0, 1),
            Self::DOWNFORWARD => (-1, 1),
            Self::DOWN => (-1, 0),
            Self::DOWNBACKWARD => (-1, -1),
            Self::BACKWARD => (0, -1),
            Self::UPBACKWARD => (1, -1),
        }
    }

    pub fn variants() -> Vec<Self> {
        vec![
            Self::UP,
            Self::UPFORWARD,
            Self::FORWARD,
            Self::DOWNFORWARD,
            Self::DOWN,
            Self::DOWNBACKWARD,
            Self::BACKWARD,
            Self::UPBACKWARD,
        ]
    }
}
