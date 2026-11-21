# Graph Algorithms

This section details the search, shortest-path, and topological algorithms available in NetFog. In their current stage, these functions return the traversal order, distances, or per-node/per-graph scores, serving as a foundation for future implementations of on-the-fly node processing.

---

## Python Reference

**Node addressing:** wherever a method below takes a `start_node_label` (or another node-reference parameter), you can pass either a `str` (the node's label) or an `int` (its internal numeric ID, as returned by `Node.id` or by `node_by_label(...).id`). Addressing by ID is faster, since it skips the label-to-ID lookup step; addressing by label is more convenient when working with human-readable identifiers. Both forms are accepted interchangeably.

**Methods:**

- `dfs(start_node_label: str | int) -> list[str] | list[int]`  
Performs a Depth-First Search starting from the specified node. Returns a list of node labels (or IDs, if you started from an ID) in the order they were visited.

    **Note**: Currently, this method returns the traversal order. Future updates will allow applying custom functions to each node during the search.

- `bfs(start_node_label: str | int) -> list[str] | list[int]`  
Performs a Breadth-First Search starting from the specified node. Returns a list of node labels (or IDs) in the order they were discovered (level by level).

- `dijkstra(start_node_label: str | int) -> dict | list[float]`  
Computes the shortest path from the starting node to all other nodes in the graph using Dijkstra's Algorithm. When starting from a label, returns a dictionary mapping node labels to their minimum distance (weight). When starting from an ID, returns a plain list of distances indexed by node ID, for faster access.

- `connected_components() -> list[int]`  
Partitions the graph into weakly connected components (edge direction is ignored). Returns a list where position `i` holds the component ID of node `i`.

- `connected_components_count() -> int`  
Returns the total number of connected components in the graph.

- `connected_components_grouped() -> dict`  
Returns a dictionary mapping each component ID to the list of node IDs that belong to it.

- `pagerank(damping: float = 0.85, max_iter: int = 100, tolerance: float = 1e-6) -> list[float]`  
Computes the PageRank score of every node, returned as a list indexed by node ID. `damping` is the probability of following an edge rather than teleporting to a random node; `max_iter` and `tolerance` control the stopping criteria for the iterative computation.

- `triangle_count() -> int`  
Returns the total number of triangles (sets of three mutually connected nodes) in the graph.

- `triangle_count_by_node() -> list[int]`  
Returns the number of triangles each node participates in, as a list indexed by node ID.

- `betweenness_centrality() -> list[float]`  
Computes the betweenness centrality of every node — how often it lies on the shortest path between other pairs of nodes — returned as a list indexed by node ID.

---

## Python Examples

```python
from netfog import Graph

g = Graph()
for label in ["A", "B", "C", "D", "E"]:
    g.add_node(label)

g.create_connection("A", "B", weight=1.0, directed=True)
g.create_connection("A", "C", weight=4.0, directed=True)
g.create_connection("B", "C", weight=2.0, directed=True)
g.create_connection("B", "D", weight=5.0, directed=True)
g.create_connection("C", "D", weight=1.0, directed=True)

# 1. Depth-First Search (DFS)
print("DFS Order:", g.dfs("A"))

# 2. Breadth-First Search (BFS)
print("BFS Order:", g.bfs("A"))

# 3. Dijkstra (Shortest Paths)
distances = g.dijkstra("A")
print("Shortest distances from A:")
for node, dist in distances.items():
    print(f"To {node}: {dist}")

# Same calls, addressing the node by its numeric ID instead of its label
node_a = g.node_by_label("A")
print("DFS Order (by id):", g.dfs(node_a.id))
print("Distances (by id):", g.dijkstra(node_a.id))

# 4. Connected Components
print("Component per node:", g.connected_components())
print("Number of components:", g.connected_components_count())
print("Nodes grouped by component:", g.connected_components_grouped())

# 5. PageRank
print("PageRank scores:", g.pagerank())

# 6. Triangle Counting
print("Total triangles:", g.triangle_count())
print("Triangles per node:", g.triangle_count_by_node())

# 7. Betweenness Centrality
print("Betweenness centrality:", g.betweenness_centrality())
```
