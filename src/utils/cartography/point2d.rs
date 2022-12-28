use super::CompassDirection;

/// Represents a single point in two-dimensional Euclidean space.
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub struct Point2D {
    x: i64,
    y: i64,
}

impl Point2D {
    /// Creates a new 2D point.
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    /// Gets the value of the x-coordinate.
    pub fn x(&self) -> i64 {
        self.x
    }

    /// Updates the value of the x-coordinate.
    pub fn set_x(&mut self, x: i64) {
        self.x = x;
    }

    /// Gets the value of the y-coordinate.
    pub fn y(&self) -> i64 {
        self.y
    }

    /// Updates the value of the y-coordinate.
    pub fn set_y(&mut self, y: i64) {
        self.y = y;
    }

    /// Checks if another Point2D is adjacent to the current one.
    pub fn is_adjacent(&self, other: &Point2D) -> bool {
        (self.x - other.x).abs() > 1 || (self.y - other.y).abs() > 1
    }

    /// Moves the point by the specified amount in the x- and y-directions.
    pub fn move_point(&mut self, dx: i64, dy: i64) {
        self.x += dx;
        self.y += dy;
    }

    /// Returns the Point2D after the current point is moved by the specified x- and y-deltas.
    pub fn peek_move_point(&self, dx: i64, dy: i64) -> Point2D {
        Point2D::new(self.x + dx, self.y + dy)
    }

    /// Gets the eight surrounding points from the current location. Panics if integer overflow or
    /// underflow would occur.
    pub fn get_surrounding_points(&self) -> Vec<Point2D> {
        vec![
            Point2D::new(self.x, self.y - 1),     // up
            Point2D::new(self.x + 1, self.y - 1), // diag - up right
            Point2D::new(self.x + 1, self.y),     // right
            Point2D::new(self.x + 1, self.y + 1), // diag - down right
            Point2D::new(self.x, self.y + 1),     // down
            Point2D::new(self.x - 1, self.y + 1), // diag - down left
            Point2D::new(self.x - 1, self.y),     // left
            Point2D::new(self.x - 1, self.y - 1), // diag - up left
        ]
    }

    /// Gets the four points adjacent to the current location (excluding diagonals) - up, down,
    /// left and right. Panics if integer overflow or underflow would occur.
    pub fn get_adjacent_points(&self) -> Vec<Point2D> {
        vec![
            Point2D::new(self.x, self.y - 1), // up
            Point2D::new(self.x + 1, self.y), // right
            Point2D::new(self.x, self.y + 1), // down
            Point2D::new(self.x - 1, self.y), // left
        ]
    }

    /// Calculates the Manhattan distance between the current point and the other point.
    pub fn calculate_manhattan_distance(&self, other: &Point2D) -> u64 {
        (self.x - other.x).unsigned_abs() + (self.y - other.y).unsigned_abs()
    }

    /// Gets the point in the given direction from the current point.
    pub fn check_move_in_direction(&self, dirn: CompassDirection) -> Point2D {
        match dirn {
            CompassDirection::North => self.peek_move_point(0, -1),
            CompassDirection::NorthEast => self.peek_move_point(1, -1),
            CompassDirection::East => self.peek_move_point(1, 0),
            CompassDirection::SouthEast => self.peek_move_point(1, 1),
            CompassDirection::South => self.peek_move_point(0, 1),
            CompassDirection::SouthWest => self.peek_move_point(-1, 1),
            CompassDirection::West => self.peek_move_point(-1, 0),
            CompassDirection::NorthWest => self.peek_move_point(-1, -1),
        }
    }
}
