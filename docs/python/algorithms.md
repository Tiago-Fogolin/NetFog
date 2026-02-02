# Graph Algorithms

This section details the search and shortest-path algorithms available in NetFog. In their current stage, these functions return the traversal order or distance metrics, serving as a foundation for future implementations of on-the-fly node processing.

---

## Python Reference

**Methods:**

- `dfs(start_node_label: str) -> list[str]`  
Performs a Depth-First Search starting from the specified node. Returns a list of node labels in the order they were visited.

    **Note**: Currently, this method returns the traversal order. Future updates will allow applying custom functions to each node during the search.

- `bfs(start_node_label: str) -> list[str]`  
Performs a Breadth-First Search starting from the specified node. Returns a list of node labels in the order they were discovered (level by level).

- `dijkstra(start_node_label: str) -> dict`  
Computes the shortest path from the starting node to all other nodes in the graph using Dijkstra's Algorithm. Returns a dictionary where keys are node labels and values are the minimum distances (weights).

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
```