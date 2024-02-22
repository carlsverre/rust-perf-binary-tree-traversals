pub fn main() {
    let mut store: std::vec::Vec<Node> = vec![];

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
    let mut cursor = Some(root);
    let mut sum = 0;

    unsafe {
        asm!(
            "nop # LLVM-MCA-BEGIN",
            options(nomem, preserves_flags, nostack)
        );
    }
    while let Some(current) = cursor {
        match store[current as usize] {
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
    unsafe {
        asm!(
            "nop # LLVM-MCA-END",
            options(nomem, preserves_flags, nostack)
        );
    }

    sum
}
