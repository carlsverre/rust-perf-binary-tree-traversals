pub fn main() {
    let mut store: Vec<Node> = vec![];

    macro_rules! b {
        ($l:expr, $r:expr) => {{
            let n = Node::Branch($l, $r);
            store.push(n);
            (store.len() - 1) as u32
        }};
    }
    macro_rules! l {
        ($n:expr) => {{
            let n = Node::Leaf($n);
            store.push(n);
            (store.len() - 1) as u32
        }};
    }

    let branch_2 = b!(l!(1), l!(2));
    let branch_4 = b!(branch_2, branch_2);
    let branch_8 = b!(branch_4, branch_4);
    let root = b!(branch_8, b!(branch_4, branch_2));

    println!("{}", traverse_stack(&store, root))
}

use std::arch::asm;

enum Node {
    Branch(u32, u32),
    Leaf(u32),
}

#[no_mangle]
fn traverse_stack(store: &[Node], root: u32) -> u32 {
    let mut stack = heapless::Vec::<_, 32>::new();
    let _ = stack.push(root);
    let mut sum = 0;

    unsafe {
        asm!(
            "nop # LLVM-MCA-BEGIN",
            options(nomem, preserves_flags, nostack)
        );
    }
    while let Some(node) = stack.pop() {
        match store[node as usize] {
            Node::Leaf(n) => {
                sum += n;
            }
            Node::Branch(l, r) => {
                let _ = stack.push(r);
                let _ = stack.push(l);
            }
        }
    }
    unsafe {
        asm!(
            "nop # LLVM-MCA-END",
            options(nomem, preserves_flags, nostack)
        );
    }

    sum
}
