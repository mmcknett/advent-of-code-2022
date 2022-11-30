#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
    pub z: i32
}

impl Coord {
    pub fn new(x: i32, y: i32, z: i32) -> Coord {
        Coord { x, y, z }
    }

    pub fn from(coord: (i32, i32, i32)) -> Coord {
        let (x, y, z) = coord;
        Coord::new(x, y, z)
    }
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Line {
  pub s: Coord,
  pub e: Coord
}


impl Line {
  pub fn new(start: Coord, end: Coord) -> Line {
    Line {
      s: start,
      e: end
    }
  }
}
