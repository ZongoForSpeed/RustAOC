#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub(crate) struct Point2D {
    x: i32,
    y: i32,
}

pub(crate) struct Map {
    map: Vec<Vec<char>>,
    x_max: i32,
    y_max: i32,
}

impl Point2D {
    pub fn new(x: i32, y: i32) -> Self {
        Point2D { x, y }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn manhattan(&self) -> i32 {
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

    pub fn valid(&self, x_max: i32, y_max: i32) -> bool {
        self.x < x_max && self.y < y_max && self.x >= 0 && self.y >= 0
    }
}

impl Map {
    pub fn new(map: Vec<Vec<char>>) -> Self {
        let y_max = map.len() as i32;
        let x_max = map.iter().map(|row| row.len()).max().unwrap()  as i32;
        Map { map, y_max, x_max, }
    }

    pub fn get(&self, p: &Point2D) -> Option<char> {
        p.valid(self.x_max, self.y_max).then(|| self.map[p.y as usize][p.x as usize])
    }

    pub fn set(&mut self, p: &Point2D, character: char) {
        self.map[p.y as usize][p.x as usize] = character;
    }
    
    pub fn x_max(&self) -> i32 { self.x_max }
    
    pub fn y_max(&self) -> i32 { self.y_max }
}