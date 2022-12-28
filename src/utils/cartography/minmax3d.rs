use super::Point3D;

/// Used to record the minimum and maximum axis values amongst the observed cubes.
pub struct MinMax3D {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
    min_z: i64,
    max_z: i64,
}

impl MinMax3D {
    pub fn new(min_x: i64, max_x: i64, min_y: i64, max_y: i64, min_z: i64, max_z: i64) -> Self {
        Self {
            min_x,
            max_x,
            min_y,
            max_y,
            min_z,
            max_z,
        }
    }

    /// Checks if the given 3D-point is within the bounds (inclusive) of the 3D-minmax.
    pub fn contains_point(&self, loc: &Point3D) -> bool {
        self.min_x <= loc.x()
            && self.max_x >= loc.x()
            && self.min_y <= loc.y()
            && self.max_y >= loc.y()
            && self.min_z <= loc.z()
            && self.max_z >= loc.z()
    }

    /// Gets the value of the min_x field.
    pub fn min_x(&self) -> i64 {
        self.min_x
    }

    /// Gets the value of the max_x field.
    pub fn max_x(&self) -> i64 {
        self.max_x
    }

    /// Gets the value of the min_y field.
    pub fn min_y(&self) -> i64 {
        self.min_y
    }

    /// Gets the value of the max_y field.
    pub fn max_y(&self) -> i64 {
        self.max_y
    }

    /// Gets the value of the min_z field.
    pub fn min_z(&self) -> i64 {
        self.min_z
    }

    /// Gets the value of the max_z field.
    pub fn max_z(&self) -> i64 {
        self.max_z
    }
}
