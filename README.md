# Performance analysis of binary tree traversals

This repo specifically looks at binary tree traversals with the following properties:

- the tree nodes are stored in a HashMap, keyed by a 32 byte digest; this permits structural sharing
- the trees are always built in such a way that the left subtree of any branch is a complete and balanced binary tree, this makes certain optimizations like chasing the left branch of the tree more optimal

Four optimizations are explored in this repo - all on top of a basic preorder traversal.

1. Allocate the next-subtree stack on the stack rather than in the heap using the `heapless` crate
1. Allocate the next-subtree stack on the heap using the `coca` crate to produce a fixed capacity vector
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

The simplest optimization is to switch from a heap allocated vector to either heapless or coca. This results in a solid (15.1% or 25% fewer cycles respectively) performance improvement without making the code much more complex.

The best optimization is to use heapless or coca as well as a nested loop to chase the left subtree which gives us a 29.7% or 39.8% cycle improvement over the simple vector approach.

By switching away from Vec to either heapless or coca we gain the primary benefit of removing implicit resize behavior. This is safe to do because we can predict the maximum size of the stack based on the size of the tree by exploiting knowledge of the tree's underlying structure and the traversal algorithm.

Coca has the further advantage over heapless in that it supports specifying the capacity at runtime. This does require a global allocator however so may not be the best choice in all applications.

The version which uses an Option to track the cursor is less efficient than the vanilla Vec based approach so is not recommended in any case.