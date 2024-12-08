# Resources
**x86_64 instruction reference**:
https://www.felixcloutier.com/x86/

**Intel intrinsics guide**:
https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html

# Day 1

## Part 1

This is just parsing integers out of a byte stream, sorting the two lists,
and subtracting the pairs. The naive solution is straightforward.

**Naive timing**: ~9us

For the fast solution we'll parse integers using SIMD.
Implementation is inspired by this article:
http://0x80.pl/articles/simd-parsing-int-sequences.html

We know more about out inputs than that article, so we can skip validation.
