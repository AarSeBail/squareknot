# Storage
- [X] Adjacency List
- [X] Adjacency Matrix
- [ ] Laplacian Matrix

# Traversal
- [X] Depth First Iterator
  - [X] Full Traversal
- [X] Breadth First Iterator
  - [X] Full Traversal
  - [X] Recording Traversal
- [X] Components and Component Counts

# Combinators
- [ ] Revamp Combinator System
  - Brainstorming: Want functions f:GraphLike -> GraphLike
  - Do not want the functions to be GraphLike
  - Want the functions to be applicable to both GraphLike and AbstractGraph
- [ ] Create `ExactVertexSize` trait
  - Adds extra methods to allow implementations to inform consumers of the number of labels used
- [ ] Create `ExactEdgeSize` trait
  - Same as `ExactVertexSize`
- [ ] Create `UnGraphLike` trait
  - Acts as an invariant to let users know that implementations are intended to be undirected
- [ ] Make `squareknot_io` use `GraphLike` and additional traits when applicable

# Graph Metadata
- [ ] Design Metadata System

# Pathing
- [X] Shortest Path - Undirected
- [ ] Eulerian Cycle/Circuit
- [ ] All Pairs Shortest Paths Iterator - Unweighted
- [ ] K-Shortest Disjoint Paths
- [ ] Dijkstra's Algorithm
- [ ] Johnson's Algorithm
- [ ] Best Path Approximation w/ Simulated Annealing

# Network Flow
- [ ] Edmonds-Karp Algorithm

# Algebra
- [ ] Isomorphism Testing and Construction
- [ ] Automorphism Algorithms

# Cliques
- [ ] Bron-Kerbosch Algorithm

# Planarity
- [ ] Rotational Systems
- [ ] Hopcroft-Tarjan
- [ ] Boyer-Myrvold

# Coloring
- [ ] Greedy Coloring Algorithms
- [ ] Other Coloring Algorithms

# Other
- [ ] Union Find
- [ ] Kruskal's Algorithm

# Topology
- [ ] Gluing Maps
- [ ] Cell Complex
  - [ ] Simplicial Complex

# Topological Embeddings
- [ ] Rotational Systems
- [ ] Exponential Embedding Algorithm
- [ ] Polynomial Embedding Algorithm

# Utilities
- [ ] Actual `GraphFormat` Error Types
- [X] Implement Modified DIMACS Writer
- [ ] Implement Real DIMACS Formatting
- [X] Vertex removal is slow, add an `InducedSubgraph` type
  - Replace `rem_vertex` entirely? Probably, since testing for vertex membership causes slowdowns elsewhere.
  - This was done as `VertexFilter`
- [ ] Augment Graphs with Auxilliary Data Structures
  - [ ] Q: How to do this w/ zero cost?
  - [ ] Q: Use this to abstract away degrees?
- [ ] Better Ordering Abstraction
- [ ] Abstract Partial Mutability
- [ ] Documentation
- [ ] Feature flags for auxilliary data, weighted graphs, etc.
  - [ ] Additional feature flags?
- [X] Integration Testing
- [X] Having `neighbor_iter` instead of `neighbors` would be faster for adjacency matrices, due to no heap allocation.
  - [X] Q: How to do the types for this? Probably use associated types... A: Just use ``impl Iterator<Item = usize>``.
- [ ] Conversion b/w Storage Types
- [ ] AI Generated Logo
- [ ] Add `README.md` to each crate

# Quandaries
- [ ] The new neighbor functions do not support .rev and other `DoubleEndedIterator` operations. This messes with the expected order of BFS. In all fairness, neighborhoods might best be described as unordered unless an order is induced.

# Documentation
- [ ] Further clarify the differences between `Storage` and `AbstractGraph`
- [ ] Write better documentation