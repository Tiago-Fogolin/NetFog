# Graph Metrics

This section explains how to **compute metrics and statistics** of a graph in NetFog.

---

## Python Reference

**Methods:**

- `get_total_weight() -> float`  
Returns the sum of the weights of all edges in the graph.

- `get_mean_weight() -> float`  
Returns the mean weight of all edges. If the graph has no edges, returns `0`.

- `get_node_count() -> int`  
Returns the total number of nodes in the graph.

- `get_edge_count() -> int`  
Returns the total number of edges in the graph. Directed edges are counted individually.

- `get_density(directed: bool) -> float`  
Returns the density of the graph.  
  - For **directed graphs**, density = 2 × |E| / (|V| × (|V| - 1))  
  - For **undirected graphs**, density = |E| / (|V| × (|V| - 1) / 2)

- `compute_degrees(node_label: str) -> dict`  
  Returns a dictionary with the degree metrics for the given node:  
  - `in_degree`: number of incoming edges (for directed edges)  
  - `out_degree`: number of outgoing edges (for directed edges)  
  - `undirected_degree`: number of edges that are not directed  
  - `total_degree`: sum of all edges connected to the node
  
- `get_all_nodes_degrees() -> dict`  
  Returns a dictionary containing the degree metrics for each node in the graph
  - The dictionary key is the node label, and the value is another dictionary with the degree metrics (same as the return of compute_degrees).

- `get_average_degree() -> float`  
  Returns the average degree of the graph.
---

## Python Examples

```python
from netfog import Graph

g = Graph()
g.add_node("A")
g.add_node("B")
g.add_node("C")

g.create_connection("A", "B", weight=10, directed=False)
g.create_connection("C", "B", weight=5, directed=True)

# Compute metrics
print("Total weight:", g.get_total_weight())
print("Mean weight:", g.get_mean_weight())
print("Node count:", g.get_node_count())
print("Edge count:", g.get_edge_count())
print("Graph density (directed):", g.get_density(directed=True))

# Compute degrees for a node
degrees_B = g.compute_degrees("B")
print("Degrees for B:", degrees_B)

# Compute degrees for all nodes
all_degrees = g.get_all_nodes_degrees()
print("Degrees for all nodes:", all_degrees)

# Get the average degree of the graph
avg_deg = g.get_average_degree(directed=False)
print(avg_deg)
```