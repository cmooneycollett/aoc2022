/// Represents the eight compass directions including the cardinal and inter-cardinal directions.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CompassDirection {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}
