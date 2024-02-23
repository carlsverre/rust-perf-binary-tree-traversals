use btt::*;
use rand::{thread_rng, Rng};

pub fn main() {
    let mut store: Store = Store::default();
    let mut rnd = thread_rng();

    macro_rules! d {
        () => {{
            let mut buf = [0u8; 32];
            rnd.fill(&mut buf);
            buf
        }};
    }

    macro_rules! b {
        ($l:expr, $r:expr) => {{
            let n = Node::Branch($l, $r);
            let digest = d!();
            store.insert(digest.clone(), n);
            digest
        }};
    }
    macro_rules! l {
        ($n:expr) => {{
            let n = Node::Leaf($n);
            let digest = d!();
            store.insert(digest.clone(), n);
            digest
        }};
    }

    let branch_2 = b!(l!(1), l!(2));
    let branch_4 = b!(branch_2, branch_2);
    let branch_8 = b!(branch_4, branch_4);
    let root = b!(branch_8, b!(branch_4, branch_2));

    let traversals: Vec<(_, Box<dyn Traverse>)> = vec![
        ("simple", Box::new(Simple)),
        ("heapless", Box::new(Heapless)),
        ("heapless_loop", Box::new(HeaplessLoop)),
        ("heapless_option", Box::new(HeaplessOption)),
        ("vec_loop", Box::new(VecLoop)),
        ("vec_option", Box::new(VecOption)),
    ];

    for traversal in traversals {
        let sum = traversal.1.traverse_sum(&store, root);
        println!("{}: {}", traversal.0, sum);
    }
}
