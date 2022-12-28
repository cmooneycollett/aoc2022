use super::Point2D;

/// Used to represent the minimum and maximum x- and y-values for a two-dimensional region.
pub struct MinMax2D {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
}

impl MinMax2D {
    pub fn new(min_x: i64, max_x: i64, min_y: i64, max_y: i64) -> Self {
        Self {
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }

    /// Checks if the given 2D point within the bounds of the minmax (inclusive).
    pub fn contains_point(&self, point: &Point2D) -> bool {
        self.min_x <= point.x()
            && self.max_x >= point.x()
            && self.min_y <= point.y()
            && self.max_y >= point.y()
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
}
