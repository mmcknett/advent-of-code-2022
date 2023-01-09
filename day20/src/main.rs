use std::{rc::Rc, cell::{RefCell}, fmt::Debug};

fn main() {
    // Read in the file provided as the first argument.
    let path = utils::args_iter().next().expect("Missing argument");
    let input = std::fs::read_to_string(&path).expect(&format!["Couldn't find file \"{path}\""]);

    // Parse input
    let numbers: Vec<isize> = input.split("\n").map(|n| n.parse::<isize>().unwrap()).collect();

    // Part 1
    let (ringlist, zero) = make_ringlist(&numbers);
    println!("Ring list: {:?}\nZero: {:?}", ringlist, zero);

    // Part 2
}

fn make_ringlist(numbers: &Vec<isize>) -> (RingList, Link) {
    let ringlist: RingList = numbers.iter().map(|val| Node::new(*val)).collect();
    let mut zero: Option<Link> = None;

    if numbers.len() < 2 {
        panic!("Ringlist is assuming more than two entries right now.");
    }

    // Connect first and last
    let last = ringlist.len() - 1;
    (ringlist[0].borrow_mut().prev, ringlist[0].borrow_mut().next) = (Some(ringlist[last].clone()), Some(ringlist[1].clone()));
    (ringlist[last].borrow_mut().prev, ringlist[last].borrow_mut().next) = (Some(ringlist[last - 1].clone()), Some(ringlist[0].clone()));

    if ringlist[0].borrow().val == 0 {
        zero = Some(ringlist[0].clone());
    }
    if ringlist[last].borrow().val == 0 {
        zero = Some(ringlist[last].clone());
    }

    for triple in ringlist.windows(3) {
        let (prev, curr, next) = (triple[0].clone(), &triple[1], triple[2].clone());
        curr.borrow_mut().prev = Some(prev);
        curr.borrow_mut().next = Some(next);
        if curr.borrow().val == 0 {
            zero = Some(curr.clone());
        }
    }

    return (ringlist, zero.unwrap());
}

#[derive(Clone)]
struct Node {
    val: isize,
    next: Option<Link>,
    prev: Option<Link>
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prev = self.prev.as_ref().map_or(String::from("None"), |r| r.borrow().val.to_string());
        let next = self.next.as_ref().map_or(String::from("None"), |r| r.borrow().val.to_string());
        write![f, "{{{} <- {} -> {}}}", prev, self.val, next]
    }
}

impl Node {
    fn new(val: isize) -> Link {
        let node = Self {
            val,
            next: None,
            prev: None
        };
        Rc::new(RefCell::new(node))
    }
}

type Link = Rc<RefCell<Node>>;
type RingList = Vec<Link>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_20_test() {
        assert_eq![1, 1]
    }
}