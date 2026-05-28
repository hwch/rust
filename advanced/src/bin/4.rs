use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

struct IPoint {
    x: i32,
    y: i32,
}

impl Add<IPoint> for Point {
    type Output = Point;

    fn add(self, other: IPoint) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<Point> for IPoint {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + IPoint { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    assert_eq!(
        IPoint { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}
