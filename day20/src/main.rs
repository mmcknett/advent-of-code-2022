use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::Debug;
use std::usize;

fn main() {
    // Read in the file provided as the first argument.
    let path = utils::args_iter().next().expect("Missing argument");
    let input = std::fs::read_to_string(&path).expect(&format!["Couldn't find file \"{path}\""]);

    // Parse input
    let numbers: Vec<isize> = input.split("\n").map(|n| n.parse::<isize>().unwrap()).collect();

    // Part 1
    let (ringlist, zero) = make_ringlist(&numbers);
    // println!("Ring list: {:?}\nZero: {:?}", ringlist, zero);

    // The ring list is now a vector that maintains references to the nodes in their original order,
    // and the nodes can be re-arranged via linked-list operations.
    for node in &ringlist {
        node.borrow_mut().move_val(ringlist.len());
    }

    let grove_coords: Vec<isize> = [1000, 2000, 3000].iter().map(|&i| zero.borrow().val_after(i)).collect();
    let sum: isize = grove_coords.iter().sum();
    println!("Part 1: Grove coordinates are {:?} -- sum: {sum}", grove_coords);

    // Part 2
    const KEY: isize = 811589153; // Shouldn't need to switch to i64 on my M1 mac.
    let numbers: Vec<isize> = numbers.iter().map(|n| n * KEY).collect();
    let (ringlist, zero) = make_ringlist(&numbers);

    for _ in 0..10 {
        for node in &ringlist {
            node.borrow_mut().move_val(ringlist.len());
        }
    }

    let grove_coords: Vec<isize> = [1000, 2000, 3000].iter().map(|&i| zero.borrow().val_after(i)).collect();
    let sum: isize = grove_coords.iter().sum();
    println!("Part 2: Grove coordinates are {:?} -- sum: {sum}", grove_coords);
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

    fn move_val(&mut self, list_len: usize) {
        // Move this node within the linked list according to its value.
        if self.val < 0 {
            self.move_left(list_len);
        } else if self.val > 0 {
            self.move_right(list_len);
        }
        // else if it's 0, no movement at all.
    }

    fn move_left(&mut self, list_len: usize) {
        let dist = self.val.abs() as usize % (list_len - 1);
        for _ in 0..dist {
            // Move left in between self and right.
            // LP <-> L <-> S <-> R
            //  ... becomes ...
            // LP <-> S <-> L <-> R
            // So update six links: 1 for left_prev and right; two for left and self.
            let left = self.prev.clone().unwrap();
            let right = self.next.clone().unwrap();
            let self_ref = left.borrow().next.clone().unwrap();
            let left_prev = left.borrow().prev.clone().unwrap();

            left_prev.borrow_mut().next = Some(self_ref.clone());
            right.borrow_mut().prev = Some(left.clone());
            left.borrow_mut().next = Some(right.clone());
            left.borrow_mut().prev = Some(self_ref.clone());
            self.next = Some(left.clone());
            self.prev = Some(left_prev.clone());
        }
    }

    fn move_right(&mut self, list_len: usize) {
        let dist = self.val.abs() as usize % (list_len - 1);
        for _ in 0..dist {
            // Move left in between self and right.
            // L <-> S <-> R <-> RN
            //  ... becomes ...
            // L <-> R <-> S <-> RN
            // So update six links: 1 for right_next and left; two for right and self.
            let left = self.prev.clone().unwrap();
            let right = self.next.clone().unwrap();
            let self_ref = right.borrow().prev.clone().unwrap();
            let right_next = right.borrow().next.clone().unwrap();

            right_next.borrow_mut().prev = Some(self_ref.clone());
            left.borrow_mut().next = Some(right.clone());
            right.borrow_mut().prev = Some(left.clone());
            right.borrow_mut().next = Some(self_ref.clone());
            self.next = Some(right_next.clone());
            self.prev = Some(right.clone());
        }
    }

    fn val_after(&self, mut i: usize) -> isize {
        if i == 0 {
            return self.val;
        }

        let mut curr = self.next.clone();
        while i > 1 {
            curr = curr.expect("There shouldn't be any missing references!").borrow().next.clone();
            i -= 1;
        }

        return curr.expect("There shouldn't be any missing references!").borrow().val;
    }
}

type Link = Rc<RefCell<Node>>;
type RingList = Vec<Link>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_20_test() {
        let v: Vec<isize> = vec![-1, 0, 3];
        let (ringlist, zero) = make_ringlist(&v);
        assert_eq!(format!("{:?}", zero), "RefCell { value: {-1 <- 0 -> 3} }");

        ringlist[0].borrow_mut().move_val(3);
        assert_eq!(format!("{:?}", ringlist[0]), "RefCell { value: {0 <- -1 -> 3} }");
        assert_eq!(format!("{:?}", zero), "RefCell { value: {3 <- 0 -> -1} }");

        ringlist[1].borrow_mut().move_val(3);
        assert_eq!(format!("{:?}", ringlist[1]), "RefCell { value: {3 <- 0 -> -1} }");
        assert_eq!(format!("{:?}", zero), "RefCell { value: {3 <- 0 -> -1} }");

        ringlist[2].borrow_mut().move_val(3);
        // [0,-1,3] -> [0,3,-1] -> [0,-1,3] -> [0,3,-1]
        assert_eq!(format!("{:?}", ringlist[2]), "RefCell { value: {0 <- 3 -> -1} }");
        assert_eq!(format!("{:?}", zero), "RefCell { value: {-1 <- 0 -> 3} }");

        assert_eq!(zero.borrow().val_after(0), 0);
        assert_eq!(zero.borrow().val_after(1), 3);
        assert_eq!(zero.borrow().val_after(2), -1);
        assert_eq!(zero.borrow().val_after(3), 0);
    }
}