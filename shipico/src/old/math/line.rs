use super::Point;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

pub trait AsLine: Clone + Copy {
    fn start(&self) -> Point;
    fn end(&self) -> Point;

    fn is_intersect(&self, other: impl AsLine) -> bool {
        let x1 = self.start().x;
        let y1 = self.start().y;
        let x2 = self.end().x;
        let y2 = self.end().y;

        let x3 = other.start().x;
        let y3 = other.start().y;
        let x4 = other.end().x;
        let y4 = other.end().y;

        let det = (x2 - x1) * (y4 - y3) - (x4 - x3) * (y2 - y1);
        if det == 0.0 {
            return false;
        }
        let lambda = ((y4 - y3) * (x4 - x1) + (x3 - x4) * (y4 - y1)) / det;
        let gamma = ((y1 - y2) * (x4 - x1) + (x2 - x1) * (y4 - y1)) / det;
        return (0.0 < lambda && lambda < 1.0) && (0.0 < gamma && gamma < 1.0);
    }
}

impl AsLine for Line {
    fn start(&self) -> Point {
        self.start
    }

    fn end(&self) -> Point {
        self.end
    }
}

impl AsLine for (Point, Point) {
    fn start(&self) -> Point {
        self.0
    }

    fn end(&self) -> Point {
        self.1
    }
}
