# Day 20

## Part 1 Solution discussion
There's a couple wrinkles to this problem that will make using an array as a ring-buffer challenging. The biggest is matching indices of the original array to indices of the mixed array, since there are duplicate items in the puzzle input.

It might be simpler to create a doubly-linked list and keep the original list as references to the nodes. A cursory perusal of the existing linked list crates makes it seem like getting access to the nodes and referencing them might not be possible; they're abstracted away.

I'll start by creating my own doubly-linked-list Node class. This will also give me the opportunity to link the nodes in a ring.
- Ideally, I'll use weak references to make the links; I have to look up how to do that w/ Rust.
- Presumably these Nodes will need to refer to each other via Boxes, and I'm not sure how that works yet.

### Rust heap containers
I started by having a look at some Linked list and doubly linked list example implementations. I thought [Roka's blog](https://rtoch.com/posts/rust-doubly-linked-list/) had a great starting point. After reading the [`cell`](https://doc.rust-lang.org/std/cell/index.html) and [`boxed`](https://doc.rust-lang.org/std/boxed/index.html) module docs, I felt like I had a pretty good intro, but [this user forum answer](https://users.rust-lang.org/t/confused-between-box-rc-cell-arc/10946/2) really drove it home for me.

>Box<T> is for single ownership.
>Rc<T> is for multiple ownership.
>Arc<T> is for multiple ownership, but threadsafe.
>Cell<T> is for "interior mutability" for Copy types; that is, when you need to mutate something behind a &T.

So `Box` is like C++ `unique_ptr` and `Rc` is a ref-counted smart pointer like `shared_ptr`. Since you can't actually have multiple mutable references in Rust *at all*, there is `Cell` and `RefCell` available to let your ref-counted memory be mutable from multiple places. However, they're not thread safe, which is why the docs don't use them with `Arc` (atomic refcounts).

For today's data structure, I think I'll need to use RefCells to refer to the Nodes of the doubly linked list I create.