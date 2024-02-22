# Performance analysis of binary tree traversals

```
# run one or more versions
for name in heapless option loop; do echo $name; RUSTFLAGS="-Zasm-comments" cargo asm --bin $name traverse --mca-intel 2>/dev/null | head -n14; done

# dump asm
RUSTFLAGS="-Zasm-comments" cargo asm --bin loop travers

# compare mca
for name in heapless option loop; do echo $name; RUSTFLAGS="-Zasm-comments" cargo asm --bin $name traverse --mca-intel 2>/dev/null | head -n14; done
```

Compiler explorer setup: https://godbolt.org/z/b5G9bar96