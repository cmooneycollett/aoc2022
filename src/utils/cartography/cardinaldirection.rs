/// Represents the cardinal directions on a map.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CardinalDirection {
    North,
    East,
    South,
    West,
}

impl CardinalDirection {
    /// Determines the cardinal direction resulting from rotating from the current direction by 90
    /// degrees in the clockwise direction.
    pub fn rotate90_clockwise(&self) -> CardinalDirection {
        match self {
            CardinalDirection::North => CardinalDirection::East,
            CardinalDirection::East => CardinalDirection::South,
            CardinalDirection::South => CardinalDirection::West,
            CardinalDirection::West => CardinalDirection::North,
        }
    }

    /// Determines the cardinal direction resulting from rotating from the current direction by 90
    /// degrees in the counter-clockwise direction.
    pub fn rotate90_counterclockwise(&self) -> CardinalDirection {
        match self {
            CardinalDirection::North => CardinalDirection::West,
            CardinalDirection::East => CardinalDirection::North,
            CardinalDirection::South => CardinalDirection::East,
            CardinalDirection::West => CardinalDirection::South,
        }
    }
}
