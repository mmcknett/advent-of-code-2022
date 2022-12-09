#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
    pub z: i32
}

impl Coord {
    pub fn new(x: i32, y: i32, z: i32) -> Coord {
        Coord { x, y, z }
    }

    pub fn new2(x: i32, y: i32) -> Coord {
      Coord { x, y, z: 0 }
    }

    pub fn from(coord: (i32, i32, i32)) -> Coord {
        let (x, y, z) = coord;
        Coord::new(x, y, z)
    }

    pub fn from2(coord: (i32, i32)) -> Coord {
      let (x, y) = coord;
      Coord::new2(x, y)
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
