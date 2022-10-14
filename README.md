# nostbeep

[![Rust](https://github.com/luke-lorenzini/binary-heap/actions/workflows/rust.yml/badge.svg)](https://github.com/luke-lorenzini/binary-heap/actions/workflows/rust.yml)

A no_std implementation of a binary heap. More specifically, a max heap.

## Simple example

```rust
use nostbeep::MaxHeap;
let val1 = 17;
let val2 = -5;
let val3 = 100;
let mut my_heap = MaxHeap::new();
assert_eq!(0, my_heap.len());

my_heap.push(val1);
my_heap.push(val2);
my_heap.push(val3);

my_heap.pop();
my_heap.pop();
assert_eq!(Some(val2), my_heap.pop());
```
