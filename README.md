# graph_algos
This repo holds Rust code for a basic graph type, as well as implementations of some common graph algorihms.

This was written to follow along with my university's Algorithms and Data Structures course.

## Example

```rust
use graph_algos::{Graph, graph};

let mut graph: Graph<u32> = graph! {
    1 => [2, 3],
    2 => [4, 6],
    3 => [5, 6],
    5 => [6],
};

assert!(graph.is_edge(&1, &2));

graph.remove_edge(&1, &2);

assert!(!graph.is_edge(&1, &2));

// assert that the second successor of nodes 2 and 3 are the same
assert_eq!(
    graph.succs(&2).map(|succs| &succs[1]),
    graph.succs(&3).map(|succs| &succs[1]),
);
```
# Building
Any of the individual algorithms can be run with `cargo run --bin (topo_sort|shortest_path|bellman_ford|dijkstra)`.

Note: Rust 1.52 is the minimum version required.

## Documentation
You can build the documentation with `cargo doc`, and then view it with `cargo doc --open`.

## Tests
You can run the tests with `cargo test`.
