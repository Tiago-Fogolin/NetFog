# NetFog

⚠️ Warning: This library is still under development and has not yet been published on PyPI or CRAN.  
It is expected to be published on PyPI, at the very least, by the end of 2025.

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
n1 = Node("A")
n2 = Node("B")

# Add a connection (edge or arc)
# Here we use the label of the nodes for easier manipulation
g.create_connection("A", "B", weight=10, directed=False)

# Check all connections
print(g.get_connections())
```