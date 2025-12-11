#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub(crate) struct Point2D {
    x: i64,
    y: i64,
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub(crate) struct Point3D {
    x: i64,
    y: i64,
    z: i64,
}

pub(crate) struct Map {
    map: Vec<Vec<char>>,
    x_max: i64,
    y_max: i64,
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub(crate) struct Rectangle {
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
}

impl Rectangle {
    pub(crate) fn new(x_min: i64, x_max: i64, y_min: i64, y_max: i64) -> Self {
        Rectangle {
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }

    pub(crate) fn from(p1: &Point2D, p2: &Point2D) -> Rectangle {
        let x_min = std::cmp::min(p1.x, p2.x);
        let x_max = std::cmp::max(p1.x, p2.x);
        let y_min = std::cmp::min(p1.y, p2.y);
        let y_max = std::cmp::max(p1.y, p2.y);
        Rectangle::new(x_min, x_max, y_min, y_max)
    }

    pub(crate) fn overlaps_with(self, other: &Rectangle) -> bool {
        self.x_max > other.x_min
            && other.x_max > self.x_min
            && self.y_max > other.y_min
            && other.y_max > self.y_min
    }

    pub(crate) fn area(self) -> i64 {
        let dx = (self.x_min - self.x_max).abs();
        let dy = (self.y_min - self.y_max).abs();
        (dx + 1) * (dy + 1)
    }
}

impl Point2D {
    pub fn new(x: i64, y: i64) -> Self {
        Point2D { x, y }
    }

    pub fn x(&self) -> i64 {
        self.x
    }

    pub fn y(&self) -> i64 {
        self.y
    }

    pub fn manhattan(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }

    pub fn neighbors(&self) -> Vec<Point2D> {
        let mut neighbors = vec![];
        neighbors.push(Point2D::new(self.x - 1, self.y - 1));
        neighbors.push(Point2D::new(self.x, self.y - 1));
        neighbors.push(Point2D::new(self.x + 1, self.y - 1));
        neighbors.push(Point2D::new(self.x + 1, self.y));
        neighbors.push(Point2D::new(self.x - 1, self.y));
        neighbors.push(Point2D::new(self.x - 1, self.y + 1));
        neighbors.push(Point2D::new(self.x, self.y + 1));
        neighbors.push(Point2D::new(self.x + 1, self.y + 1));
        neighbors
    }

    pub fn valid(&self, x_max: i64, y_max: i64) -> bool {
        self.x < x_max && self.y < y_max && self.x >= 0 && self.y >= 0
    }
}

impl Point3D {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Point3D { x, y, z }
    }

    pub fn x(&self) -> i64 {
        self.x
    }

    pub fn y(&self) -> i64 {
        self.y
    }

    pub fn z(&self) -> i64 {
        self.z
    }

    pub fn distance(&self, other: &Point3D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        let d = (dx * dx + dy * dy + dz * dz) as f64;
        d.sqrt()
    }
}

impl Map {
    pub fn new(map: Vec<Vec<char>>) -> Self {
        let y_max = map.len() as i64;
        let x_max = map.iter().map(|row| row.len()).max().unwrap() as i64;
        Map { map, y_max, x_max }
    }

    pub fn get(&self, p: &Point2D) -> Option<char> {
        p.valid(self.x_max, self.y_max)
            .then(|| self.map[p.y as usize][p.x as usize])
    }

    pub fn set(&mut self, p: &Point2D, character: char) {
        self.map[p.y as usize][p.x as usize] = character;
    }

    pub fn x_max(&self) -> i64 {
        self.x_max
    }

    pub fn y_max(&self) -> i64 {
        self.y_max
    }
}
