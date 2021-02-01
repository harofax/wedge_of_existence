/// Wow, a rectangle!
/// x1: i32
/// x2: i32
/// y1: i32
/// y2: i32
pub struct Rect {
    pub x1 : i32,
    pub x2 : i32,
    pub y1 : i32,
    pub y2 : i32
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Rect {
        Rect{x1:x, y1:y, x2:x+w, y2:y+h}
    }


    /// returns true if this rectangle overlaps with another rectangle
    pub fn intersect(&self, other:&Rect) -> bool {
        self.x1 <= other.x2 &&
            self.x2 >= other.x1 &&
                self.y1 <= other.y2 &&
                    self.y2 >= other.y1
    }

    pub fn center(&self) -> (i32, i32) {
        ((self.x1 + self.x2)/2, (self.y1 + self.y2)/2)
    }
}