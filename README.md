# Tree of Placements

The `placements-tree` crate allows you to create an `n` to `k` tree of placements based on a `key` in the range `0..=n`.
This `key` will be used as the root and leaves of the tree.
So other vertices will be taken in the ranges `0..key` and `key + 1..=n`.

The structure is used to quickly respond to queries to update vertices or edges in a weighted directed graph with edges of negative weight.

## Example

```rust
use placements_tree::PlacementsTree;

fn main() {
    let ptree = PlacementsTree::new(4, 2, 0);
    let _shortest = ptree.update_vertex(1, 1);
    let _shortest = ptree.update_edge(0, 1, 2);
}
```

The `PlacementsTree::new(4, 2, 0)` creates a `4` by `2` tree of placements based on key `0`:

```
0
├── 1
│   ├── 2
│   │   └── 0
│   ├── 3
│   │   └── 0
│   └── 4
│       └── 0
├── 2
│   ├── 1
│   │   └── 0
│   ├── 3
│   │   └── 0
│   └── 4
│       └── 0
├── 3
│   ├── 1
│   │   └── 0
│   ├── 2
│   │   └── 0
│   └── 4
│       └── 0
└── 4
    ├── 1
    │   └── 0
    ├── 2
    │   └── 0
    └── 3
        └── 0
```

The `ptree.update_vertex(1, 1)` updates vertex `1` with value `1`, so the tree will be recalculated, returning the shortest distance:

```
0
├── 1
│   '-- 2
│   '   '-- 0
│   '-- 3
│   '   '-- 0
│   '-- 4
│       '-- 0
├── 2
│   ├── 1
│   │   '-- 0
│   ├── 3
│   │   └── 0
│   └── 4
│       └── 0
├── 3
│   ├── 1
│   │   '-- 0
│   ├── 2
│   │   └── 0
│   └── 4
│       └── 0
└── 4
    ├── 1
    │   '-- 0
    ├── 2
    │   └── 0
    └── 3
        └── 0
```

The `ptree.update_edge(0, 1, 2)` updates edge from `0` to `1` with value `2`, so the tree will be recalculated, returning the shortest distance:

```
0
'-- 1
│   '-- 2
│   '   '-- 0
│   '-- 3
│   '   '-- 0
│   '-- 4
│       '-- 0
├── 2
│   ├── 1
│   │   └── 0
│   ├── 3
│   │   └── 0
│   └── 4
│       └── 0
├── 3
│   ├── 1
│   │   └── 0
│   ├── 2
│   │   └── 0
│   └── 4
│       └── 0
└── 4
    ├── 1
    │   └── 0
    ├── 2
    │   └── 0
    └── 3
        └── 0
```

## Usage

```
[dependencies]
placements-tree = "0.1"
```

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
