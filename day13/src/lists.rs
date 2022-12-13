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
      (L(v_l), V(r)) => L(v_l.clone()).cmp(&L(vec![V(*r)])),
      (V(l), L(v_r)) => L(vec![V(*l)]).cmp(&L(v_r.clone())),
      (L(v_l), L(v_r)) => {
        let remainder: (Vec<&LorV>, Vec<&LorV>) = v_l.iter().zip(v_r.iter())
          .skip_while(
            |(l, r)| l == r
          )
          .unzip();
        match remainder {
          (vec_l, vec_r) if vec_l.len() == 0 && vec_r.len() == 0 => {
            // Everything was equal, but if the original lists are different sizes then they compare differently.
            if v_l.len() < v_r.len() {
              Ordering::Less
            } else if v_l.len() > v_r.len() {
              Ordering::Greater
            } else {
            Ordering::Equal
            }
          },
          (_, vec_r) if vec_r.len() == 0 => Ordering::Greater,
          (vec_l, vec_r) => if vec_l.len() == 0 { Ordering::Less } else { vec_l[0].cmp(vec_r[0]) }
        }
      }
      _ => {
        panic!("Can't compare!");
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