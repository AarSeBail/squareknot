# Algorithms
- [X] Depth First Iterator
- [X] Breadth First Iterator
- [ ] Greedy Coloring Algorithms
- [X] Union Find
- [ ] Components and Component Counts
- [ ] Eulerian Cycle/Circuit
- [X] All Pairs Shortest Paths Iterator - Unweighted
- [ ] Adjacency Matrix
- [ ] Laplacian Matrix
- [ ] Bron-Kerbosch Algorithm
- [ ] Isomorphism Testing and Construction
- [ ] Coloring Algorithms
- [ ] K-Shortest Disjoint Paths
- [ ] Boyer-Myrvold Algorithm
- [ ] Automorphism Algorithms
## Directed/Weighted Graphs
- [ ] Dijkstra's Algorithm
- [ ] Johnson's Algorithm
- [ ] Kruskal's Algorithm
- [ ] Edmonds-Karp Algorithm
## Topological Structures
- [ ] Gluing maps of polygons
- [ ] More general structures?

# Utilities
- [ ] Actual `GraphFormat` Error Types
- [ ] Implement Modified DIMACS Writer
- [ ] Implement Real DIMACS Formatting
- [ ] Augment Graphs with Auxilliary Data Structures
  - [ ] Q: How to do this w/ zero cost?
  - [ ] Q: Use this to abstract away degrees?
- [ ] Better Ordering Abstraction
- [ ] Abstract Partial Mutability
- [ ] Documentation
- [ ] Feature flags for auxilliary data, weighted graphs, etc.
  - [ ] Additional feature flags?
- [ ] Integration Testing
- [X] Having `neighbor_iter` instead of `neighbors` would be faster for adjacency matrices, due to no heap allocation.
  - [X] Q: How to do the types for this? Probably use associated types... A: Just use ``impl Iterator<Item = usize>``.
- [ ] Conversion b/w Storage Types
- [ ] AI Generated Logo

# Quandaries
- [ ] The new neighbor functions do not support .rev and other `DoubleEndedIterator` operations. This messes with the expected order of BFS. In all fairness, neighborhoods might best be described as unordered unless an order is induced.