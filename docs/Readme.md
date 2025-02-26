# collect_with

A utility crate for enhanced collection operations with capacity control.

[![collect-with](https://img.shields.io/crates/v/collect-with?label=collect-with)](https://crates.io/crates/collect-with)

[![Documentation](https://docs.rs/collect-with/badge.svg)](https://docs.rs/collect-with)

[![Apache-2 licensed](https://img.shields.io/crates/l/collect-with.svg)](./License)

- [Overview](#overview)
- [Features](#features)
  - [Standard Library Support](#standard-library-support)
  - [Collection Specialization](#collection-specialization)
  - [Fallible Collection](#fallible-collection)
- [Examples](#examples)
  - [Basic usage with collection](#basic-usage-with-collection)
    - [collect\_with\_capacity](#collect_with_capacity)
    - [collect\_with closure](#collect_with-closure)
    - [collect\_vec\_with](#collect_vec_with)
  - [Fallible collection (requires `try` feature)](#fallible-collection-requires-try-feature)
  - [collect indexmap (requires `indexmap` feature)](#collect-indexmap-requires-indexmap-feature)
- [About the Final Capacity Size](#about-the-final-capacity-size)
- [Traits](#traits)
  - [Core Components](#core-components)
  - [Optional Components](#optional-components)

## Overview

Provides traits for collecting iterators into collections with:

- Precise capacity management
- Fallible collection operations
- Feature-gated collection types

## Features

### Standard Library Support

- `std`:
  - Enables standard library integrations
  - When disabled, uses `alloc` crate for **no_std** environments

### Collection Specialization

- `collect_vec`:
  - Enables `CollectVector` trait for enhanced `Vec` collection
  - Provides `collect_vec_with()` and `collect_vec_with_exact()`
- `ahash`:
  - Enables `CollectAHash` trait for AHash-powered hash collections
  - Provides `collect_ahashmap_with()` and `collect_ahashset_with()`
- `indexmap`:
  - Enables `CollectIndex` trait for `IndexMap` & `IndexSet` collections
  - Provides `collect_indexmap_with()` and `collect_indexset_with()`

### Fallible Collection

- `try`: Enables fallible collection
  - `TryExtract`: Trait for item extraction with error handling,
    converting fallible types like `Option<T>` to `Result<T, ()>`.
  - `TryCollectWith` trait for error-propagating collection

## Examples

### Basic usage with collection

#### collect_with_capacity

```rust
use collect_with::CollectWithCapacity;

let numbers = (0..10).collect_with_capacity::<Vec<_>>(20);
assert_eq!(numbers.capacity(), 20);
```

#### collect_with closure

```rust
use collect_with::CollectWith;

let s = [vec!["a"], vec!["b", "c", "d"]]
  .into_iter()
  .flatten()
  .collect_with::<String>(|size| match size {
    0 => 8,
    n => n,
  });
assert_eq!(s.len(), 4);
assert_eq!(s.capacity(), 8);
```

#### collect_vec_with

```rust
use collect_with::CollectVector;

let numbers = (0..10).collect_vec_with(|hint|{
  match hint {
    0 => 12,
    n => n + 5,
  }
});
assert_eq!(numbers.capacity(), 15);
```

### Fallible collection (requires `try` feature)

```rust
use collect_with::{TryCollectWith, TryExtract};

let result = [Some(12), Some(42), Some(77)]
  .into_iter()
  .try_collect_vec_with(|u| u); // -> Result<Vec<i32>, ()>

assert_eq!(result.as_deref(), Ok(&[12, 42, 77][..]));
```

```rust
use collect_with::{TryCollectWith, TryExtract};

let result = ["42", "76", "abc"]
  .into_iter()
  .map(|x| x.parse::<i32>()) // &str -> Result<i32>
  .try_collect_with::<Vec<_>, _, _>(|u| u + 3); // -> Result<Vec<i32>, ParseIntError>

assert!(result.is_err());
```

### collect indexmap (requires `indexmap` feature)

```rust
use indexmap::IndexMap;
use collect_with::CollectIndex;

let map = ('a'..='i')
  .zip(100..=109)
  .collect_indexmap_with(|u| u + 1); // u + 1 => 9 + 1 = 10

assert_eq!(map.get(&'a'), Some(&100));
assert_eq!(map.get_index(0), Some((&'a', &100)));
assert_eq!(map.get_index(2), Some((&'c', &102)));
assert_eq!(map.capacity(), 10);
```

## About the Final Capacity Size

For example, `(0..10).collect_vec_with(|_size_bound| 2)`

1. `(0..10).size_hint()` returns `(10, Some(10))`.
2.

- lower_bound = 10 => lower
- upper_bound = Some(10) => upper
- `max(lower, upper.unwrap_or(lower))` => 10

3. _size_bound is 10.
4.

- The closure returns 2
- The final capacity is `max(_size_bound, 2)` => `max(10, 2)` = 10

5. The vector is created with `Vec::with_capacity(10)`, instead of `Vec::with_capacity(2)`.

If you need an exact capacity size, please use the `.collect_with_exact()` or `.collect_vec_with_exact()`

## Traits

### Core Components

- `ExtendWithCapacity`: A trait for collections that can be pre-allocated with specific capacity and extended with elements.
- `CollectWith`/`CollectWithCapacity`: Primary collection traits

### Optional Components

- `CollectVector` (feature = "collect_vec"): Specialized Vec collection methods
- `CollectAHash` (feature = "ahash"): AHash-based collection support
- `CollectIndex` (feature = "indexmap"): IndexMap/IndexSet collection support
- `TryExtract`/`TryCollectWith` (feature = "try")
