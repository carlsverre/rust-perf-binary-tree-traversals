use std::arch::asm;
use std::collections::HashMap;

macro_rules! mca {
    (begin $name:literal) => {
        mca!("BEGIN", $name)
    };
    (end $name:literal) => {
        mca!("END", $name)
    };
    ($cmd:literal, $name:literal) => {
        unsafe {
            asm!(
                concat!("nop # LLVM-MCA-", $cmd, " ", $name),
                options(nomem, preserves_flags, nostack)
            );
        }
    };
}

pub type Digest = [u8; 32];

pub enum Node {
    Branch(Digest, Digest),
    Leaf(u32),
}

pub type Store = HashMap<Digest, Node>;

pub trait Traverse {
    fn traverse_sum(&self, store: &Store, root: Digest) -> u32;
}

pub struct Simple;

impl Traverse for Simple {
    fn traverse_sum(&self, store: &Store, root: Digest) -> u32 {
        let mut stack = Vec::with_capacity(32);
        stack.push(root);
        let mut sum = 0;

        mca!(begin "SIMPLE");
        while let Some(node) = stack.pop() {
            match store[&node] {
                Node::Leaf(n) => {
                    sum += n;
                }
                Node::Branch(l, r) => {
                    stack.push(r);
                    stack.push(l);
                }
            }
        }
        mca!(end "SIMPLE");

        sum
    }
}

pub struct Heapless;

impl Traverse for Heapless {
    fn traverse_sum(&self, store: &Store, root: Digest) -> u32 {
        let mut stack = heapless::Vec::<_, 32>::new();
        let _ = stack.push(root);
        let mut sum = 0;

        mca!(begin "HEAPLESS");
        while let Some(node) = stack.pop() {
            match store[&node] {
                Node::Leaf(n) => {
                    sum += n;
                }
                Node::Branch(l, r) => {
                    let _ = stack.push(r);
                    let _ = stack.push(l);
                }
            }
        }
        mca!(end "HEAPLESS");

        sum
    }
}

pub struct HeaplessLoop;

impl Traverse for HeaplessLoop {
    fn traverse_sum(&self, store: &Store, root: Digest) -> u32 {
        let mut stack = heapless::Vec::<_, 32>::new();
        let _ = stack.push(root);
        let mut sum = 0;

        mca!(begin "HEAPLESS_LOOP");
        while let Some(mut node) = stack.pop() {
            loop {
                match store[&node] {
                    Node::Leaf(n) => {
                        sum += n;
                        break;
                    }
                    Node::Branch(l, r) => {
                        let _ = stack.push(r);
                        node = l;
                    }
                }
            }
        }
        mca!(end "HEAPLESS_LOOP");

        sum
    }
}

pub struct HeaplessOption;

impl Traverse for HeaplessOption {
    fn traverse_sum(&self, store: &Store, root: Digest) -> u32 {
        let mut stack = heapless::Vec::<_, 32>::new();
        let mut cursor = Some(root);
        let mut sum = 0;

        mca!(begin "HEAPLESS_OPTION");
        while let Some(current) = cursor {
            match store[&current] {
                Node::Leaf(n) => {
                    sum += n;
                    // jump to the next right-subtree if any
                    cursor = stack.pop();
                }
                Node::Branch(l, r) => {
                    // push the right-subtree to the stack to visit it later
                    let _ = stack.push(r);
                    // chase the left child
                    cursor = Some(l);
                }
            }
        }
        mca!(end "HEAPLESS_OPTION");

        sum
    }
}

pub struct VecLoop;

impl Traverse for VecLoop {
    fn traverse_sum(&self, store: &Store, root: Digest) -> u32 {
        let mut stack = Vec::with_capacity(32);
        stack.push(root);
        let mut sum = 0;

        mca!(begin "VEC_LOOP");
        while let Some(mut node) = stack.pop() {
            loop {
                match store[&node] {
                    Node::Leaf(n) => {
                        sum += n;
                        break;
                    }
                    Node::Branch(l, r) => {
                        stack.push(r);
                        node = l;
                    }
                }
            }
        }
        mca!(end "VEC_LOOP");

        sum
    }
}

pub struct VecOption;

impl Traverse for VecOption {
    fn traverse_sum(&self, store: &Store, root: Digest) -> u32 {
        let mut stack = Vec::with_capacity(32);
        let mut cursor = Some(root);
        let mut sum = 0;

        mca!(begin "VEC_OPTION");
        while let Some(current) = cursor {
            match store[&current] {
                Node::Leaf(n) => {
                    sum += n;
                    // jump to the next right-subtree if any
                    cursor = stack.pop();
                }
                Node::Branch(l, r) => {
                    // push the right-subtree to the stack to visit it later
                    stack.push(r);
                    // chase the left child
                    cursor = Some(l);
                }
            }
        }
        mca!(end "VEC_OPTION");

        sum
    }
}
