# NetFog

> [!CAUTION]
> **Work in Progress:** This library is in its early stages of development (Alpha).
> 
> * **Unstable API:** Functions and class structures may change significantly between versions.
> * **Bugs Expected:** You might encounter unexpected behavior. Please feel free to open an issue!

---

## Overview

**NetFog** is a library for creating and manipulating graphs.
Currently supports / planned features:
- Directed and undirected graphs
- Weighted edges
- Metrics
- Algorithms like BFS, DFS, Dijkstra
- Graph visualization
- Simulations

This library is designed to be **lightweight, fast, and easy to use**.

---

## Quick Start

```python
from netfog import Graph, Node

# Creates the graph object
g = Graph()

# Add nodes
g.add_node("A")
g.add_node("B")

# Add a connection (edge or arc)
# Here we use the label of the nodes for easier manipulation
g.create_connection("A", "B", weight=10, directed=False)

# Check all connections
print(g.get_connections())
```


For full documentation, see [docs](docs/index.md)
