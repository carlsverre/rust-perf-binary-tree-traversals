# Performance analysis of binary tree traversals

This repo specifically looks at binary tree traversals with the following properties:

- the tree nodes are stored in a HashMap, keyed by a 32 byte digest; this permits structural sharing
- the trees are always built in such a way that the left subtree of any branch is a complete and balanced binary tree, this makes certain optimizations like chasing the left branch of the tree more optimal

Three optimizations are explored in this repo - all on top of a basic preorder traversal.

1. Allocate the next-subtree stack on the stack rather than in the heap using the `heapless` crate
2. Chase the left subtree in an inner loop that pushes right subtrees until we reach a leaf
3. Chase the left subtree using an Option to track the cursor, and a stack to track right subtrees

Of the three implementations, personally I find 1 and 3 to be the most readable, however it turns out that 2 is slightly more instruction efficient.

Notably this repo does not consider cache line optimization too much. Due to the width of the digests, a maximum of 2 digests can be stored in a 64 byte cache line, and the digests are randomly distributed throughout the hashmap. Based on current profiling results (done separately) the standard library HashMap based on the hashbrown algorithm has been found to provide a solid performance trade-off for now.

## Analysis process

```shell
# ensure all of the implementations work
cargo run

# update analysis
RUSTFLAGS="-Zasm-comments" cargo asm --lib --mca-intel --everything | grep -A10 "Code Region" > mca_analysis.out
```

Compiler explorer setup: https://godbolt.org/z/b5G9bar96

A dump of mca has been committed to this repo in [./mca_analysis.out](./mca_analysis.out) - but this may differ than what you get on your machine.

## Conclusions

The simplest optimization is to switch from a heap allocated vector to a stack allocated vector using heapless. This results in a reasonable (15.1% fewer cycles) performance improvement without making the code much more complex.

The best optimization is to use heapless as well as a nested loop to chase the left subtree which gives us a roughly 29.7% cycle improvement.

Using a nested loop doesn't help very much unless the stack is on the stack. The algorithm seems to be completely overwhelmed by managing the dynamic vector which erases most of the perf improvement.

The version which uses an Option to track the cursor is less efficient than the vanilla Vec based approach so is not recommended in any case.