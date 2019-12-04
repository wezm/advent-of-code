#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point(pub i32, pub i32);

impl Point {
    pub fn manhattan_distance(&self) -> i32 {
        self.0.abs() + self.1.abs()
    }
}
