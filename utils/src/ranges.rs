#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Range {
  start: u32,
  end: u32
}

impl Range {
  pub fn new(start: u32, end: u32) -> Range {
    Range {
      start,
      end
    }
  }

  pub fn fully_contains(&self, other: &Range) -> bool {
    other.end <= self.end && other.start >= self.start
  }

  pub fn overlaps(&self, other: &Range) -> bool {
    other.end >= self.start && other.start <= self.start ||
    other.start <= self.end && other.start >= self.start
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn contains() {
    assert!(Range::new(2,8).fully_contains(&Range::new(3, 7)));
    assert!(Range::new(4,6).fully_contains(&Range::new(6, 6)));
    assert!(!Range::new(4,5).fully_contains(&Range::new(6, 6)));
    assert!(!Range::new(4,6).fully_contains(&Range::new(5, 7)));
  }

  #[test]
  fn overlaps() {
    assert!(Range::new(3,5).overlaps(&Range::new(4,6)));
    assert!(Range::new(4,6).overlaps(&Range::new(3,5)));
    assert!(Range::new(2,6).overlaps(&Range::new(3,5)));
    assert!(Range::new(3,5).overlaps(&Range::new(2,6)));
    assert!(Range::new(3,5).overlaps(&Range::new(3,5)));

    assert!(Range::new(5,7).overlaps(&Range::new(7,9)));
    assert!(Range::new(2,8).overlaps(&Range::new(3,7)));
    assert!(Range::new(6,6).overlaps(&Range::new(4,6)));
    assert!(Range::new(2,6).overlaps(&Range::new(4,8)));

    assert!(!Range::new(3,5).overlaps(&Range::new(7,9)));
    assert!(!Range::new(7,9).overlaps(&Range::new(3,5)));
    assert!(!Range::new(2,4).overlaps(&Range::new(6,8)));
    assert!(!Range::new(2,3).overlaps(&Range::new(4,5)));
  }
}