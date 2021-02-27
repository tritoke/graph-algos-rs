# graph_algos
This repo holds Rust code for a basic graph type, as well as implementations of some common graph algorihms.

This was written to follow along with my university's Algorithms and Data Structures course.

## Example

```rust
use graph_algos::{Graph, Edge};

let mut graph: Graph<u32> = Graph::empty();

// add an edge from node 0 to node one with a weight of 5
graph.add_edge(0, Edge::new_with_weight(1, 5));

// add two more edges
graph.add_edge(0, Edge::new_with_weight(2, 2));
graph.add_edge(2, Edge::new_with_weight(1, 1));

if graph.is_edge(&0, &1) {
    println!("There is an edge from node 0 to node 1");
} else {
    println!("There is no edge from node 0 to node 1");
}
```

## Documentation
You can build the documentation with `cargo doc`.

## Tests
You can run the tests with `cargo test`.
