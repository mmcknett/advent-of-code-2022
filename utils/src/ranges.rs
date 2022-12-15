use num::traits::One;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Range<T> {
  pub start: T,
  pub end: T
}

impl<T> Range<T>
where
  T: Ord + 
     PartialOrd + 
     num::One + 
     Copy +
     std::ops::Add<Output = T> +
     std::ops::Sub<Output = T>
{
  pub fn new(start: T, end: T) -> Range<T> {
    Range {
      start,
      end
    }
  }

  pub fn fully_contains(&self, other: &Range<T>) -> bool {
    other.end <= self.end && other.start >= self.start
  }

  pub fn overlaps(&self, other: &Range<T>) -> bool {
    other.end >= self.start && other.start <= self.start ||
    other.start <= self.end && other.start >= self.start
  }

  pub fn adjacent(&self, other: &Range<T>) -> bool
  {
    (other.end + One::one()) == self.start ||
    (self.end + One::one()) == other.start
  }

  pub fn combine(&self, other: &Range<T>) -> Option<Range<T>> {
    if self.overlaps(other) || self.adjacent(other) {
        let combined = Range::new(std::cmp::min(self.start, other.start), std::cmp::max(self.end, other.end));
        return Some(combined);
    }
    None
}

  pub fn size(&self) -> T {
    self.end - self.start
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

    assert!(!Range::new(2,6).overlaps(&Range::new(7,8)));
    assert!(!Range::new(3,5).overlaps(&Range::new(7,9)));
    assert!(!Range::new(7,9).overlaps(&Range::new(3,5)));
    assert!(!Range::new(2,4).overlaps(&Range::new(6,8)));
    assert!(!Range::new(2,3).overlaps(&Range::new(4,5)));
  }

  #[test]
  fn adjacent() {
    assert!(Range::new(3, 5).adjacent(&Range::new(6,7)));
    assert!(Range::new(6, 7).adjacent(&Range::new(3, 5)));

    assert!(!Range::new(6, 7).adjacent(&Range::new(3, 4)));
    assert!(!Range::new(6, 8).adjacent(&Range::new(3, 7)));
  }

  #[test]
  fn size() {
    assert_eq!(Range::new(-2, 24).size(), 26);
  }
}