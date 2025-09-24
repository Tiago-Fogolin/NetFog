# Graph Creation & Manipulation

This section explains how to **create graphs, add nodes, and create connections** in NetFog.

---

## Python Reference

### Classes

#### `Graph`
Represents a graph. You can add nodes, create connections, inspect the graph, and construct graphs from adjacency matrices.

**Methods:**
- `add_node(label: str)`  
  Adds a new node to the graph by label.

- `create_connection(from_label: str, to_label: str, weight: float, directed: bool) -> None`  
  Creates a connection (edge/arc) between two nodes. You can specify the weight and whether it is directed.

- `get_connections() -> list`  
  Returns a list of all connections in the graph.

- `from_adjacency_matrix(adj_matrix: list, directed: bool, custom_labels: list | None = None) -> Graph`  
  Creates a graph from an adjacency matrix. Optional custom labels can be provided.
---

## Python Examples

```python
from netfog import Graph, Node

# Create a graph
g = Graph()

# Add nodes
g.add_node("A")
g.add_node("B")
g.add_node("C")

# Create connections
g.create_connection("A", "B", weight=10, directed=False)
g.create_connection("C", "B", weight=5, directed=True)

# Inspect graph
print("Connections:", g.get_connections())

# Create graph from adjacency matrix
adj_matrix = [
    [0, 1, 0],
    [0, 0, 1],
    [1, 0, 0]
]
g2 = Graph.from_adjacency_matrix(adj_matrix, directed=True, custom_labels=["X", "Y", "Z"])
print("Nodes in g2:", g2.get_nodes())
print("Connections in g2:", g2.get_connections())
```