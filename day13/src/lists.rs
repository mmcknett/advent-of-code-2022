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
      (L(v_l), L(v_r)) => {
        let remainder: (Vec<&LorV>, Vec<&LorV>) = v_l.iter().zip(v_r.iter())
          .skip_while(
            |(l, r)| l == r
          )
          .unzip();

        let (vec_l, vec_r) = remainder;
        if vec_l.len() == 0 || vec_r.len() == 0 {
          // If one of the two zipped lists runs out, unzip returns empty lists for both.
          // When both unzipped lists are empty, we *may* have only run out of *one* of the lists while ignoring equal things.
          // In that case, it's the lengths of the original lists that matters.
          v_l.len().cmp(&v_r.len())
        } else {
          // We stopped in a place where two things were *not* equal, so return whether those things are greater or less.
          vec_l[0].cmp(vec_r[0])
        }
      }
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