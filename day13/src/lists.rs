use std::cmp::Ordering;
use std::cmp::Eq;

#[derive(Debug, Clone)]
pub enum LorV {
  L(Vec<LorV>),
  V(i32)
}

use LorV::*;

impl Ord for LorV {
  fn cmp(&self, other: &Self) -> Ordering {
    match (self, other) {
      (V(l), V(r)) => l.cmp(r),
      (L(_), V(r)) => self.cmp(&L(vec![V(*r)])),  // right becomes a list (x -> [x]), then re-compare
      (V(l), L(_)) => L(vec![V(*l)]).cmp(&other), // left becomes a list (x -> [x]), then re-compare
      (L(v_l), L(v_r)) => v_l.cmp(&v_r)
    }
  }
}

impl PartialOrd for LorV {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
      Some(self.cmp(other))
  }
}

impl PartialEq for LorV {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (V(l), V(r)) => l == r,
      (lhs, rhs) => lhs.cmp(&rhs) == Ordering::Equal
    }
  }
}

impl Eq for LorV {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn compare_test() {
    use LorV::*;
    assert![   V(1) <= V(1)   ];
    assert![   V(1) <= V(2)   ];
    assert![ !(V(2) <= V(1))  ];

    assert![   V(1) <= L(vec![V(1)])  ];
    assert![   L(vec![V(1)]) <= V(1)  ];
    assert![   V(1) <= L(vec![V(2)])  ];
    assert![   L(vec![V(1)]) <= V(2)  ];
    assert![ !(V(2) <= L(vec![V(1)])) ];
    assert![ !(L(vec![V(2)]) <= V(1)) ];
    
    assert![
      L(vec![L(vec![V(4),V(4)]),V(4),V(4)]) <= L(vec![L(vec![V(4),V(4)]),V(4),V(4), V(4)])
    ];
  }

  #[test]
  fn test_zipping() {
    // Zipping didn't work like I thought; if you zip 2, 2, 2 and 2, 2, then skip while they're equal,
    // you can't unzip to get back the last 2 on the left side. You get the residuals back if something *isn't*
    // equal, though.
    let left = vec![2, 2, 2];
    let right = vec![2, 3];

    let res: (Vec<i32>, Vec<i32>) = left.iter().zip(right.iter()).skip_while(|(l, r)| l == r).unzip();
    assert_eq![res.0.len(), 1];
  }
}