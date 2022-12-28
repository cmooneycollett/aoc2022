/// Represents a single point in three-dimensional Euclidean space.
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub struct Point3D {
    x: i64,
    y: i64,
    z: i64,
}

impl Point3D {
    /// Creates a new 3D point.
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
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

    /// Gets the value of the z-coordinate.
    pub fn z(&self) -> i64 {
        self.z
    }

    /// Updates the value of the z-coordinate.
    pub fn set_z(&mut self, z: i64) {
        self.z = z;
    }

    /// Moves the point by specified amounts on each axis.
    pub fn move_point(&mut self, dx: i64, dy: i64, dz: i64) {
        self.x += dx;
        self.y += dy;
        self.z += dz;
    }

    /// Returns the Point3D after the current point is moved by the specified x- and y-deltas.
    pub fn peek_move_point(&self, dx: i64, dy: i64, dz: i64) -> Point3D {
        Point3D::new(self.x + dx, self.y + dy, self.z + dz)
    }

    /// Calculates the Manhattan distance between the current point and the other point.
    pub fn calculate_manhattan_distance(&self, other: &Point3D) -> u64 {
        (self.x - other.x).unsigned_abs()
            + (self.y - other.y).unsigned_abs()
            + (self.z - other.z).unsigned_abs()
    }

    /// Gets the points adjacent to the given cube (not including diagonals).
    pub fn get_adjacent_points(&self) -> Vec<Point3D> {
        vec![
            self.peek_move_point(-1, 0, 0), // -dx
            self.peek_move_point(1, 0, 0),  // +dx
            self.peek_move_point(0, -1, 0), // -dy
            self.peek_move_point(0, 1, 0),  // +dy
            self.peek_move_point(0, 0, -1), // -dz
            self.peek_move_point(0, 0, 1),  // +dz
        ]
    }
}
