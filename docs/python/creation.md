# Graph Creation & Manipulation

This section explains how to **create graphs, add nodes, and create connections** in NetFog.

---

## Python Reference

### Enums

#### `OpenAlexGraphType`
An enumeration defining the type of network to be generated from the OpenAlex data.
- `Coauthorship`: Nodes represent authors, and connections represent co-authored works.
- `KeywordCooccurrence`: Nodes represent concepts/keywords, and connections indicate they appear together in the same works.
- `Cocitation`: Nodes represent works, and connections indicate they are cited together by other works.

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

- `from_net_file(file_path: str) -> Graph`  
  Creates a graph from a .net (Pajek) file. This method supports node labels, spatial coordinates ($x$, $y$), and weighted connections defined in the file.

- `from_json_file(file_path: str) -> Graph`  
  Creates a graph from a JSON file. The schema supports node coordinates and distinguishes between undirected connections (edges) and directed ones (arcs).
  JSON structure example:
  ```json
  {
    "nodes": [
      {"label": "1", "x": 10.5, "y": 20.0},
      {"label": "2", "x": 15.0, "y": 25.0}
    ],
    "edges": [
      {"source": "1", "target": "2", "weight": 1.0}
    ],
    "arcs": [
      {"source": "2", "target": "3", "weight": 0.5}
    ]
  }
  ```
---

- `from_openalex(api_key: str, graph_type: OpenAlexGraphType, search=None, author=None, author_id=None, author_orcid=None, keyword=None, limit=None, min_weight=None) -> Graph`  
  Creates a graph by dynamically querying the OpenAlex API based on specified filters.

  **Parameters:**
  * `api_key`: Your OpenAlex API key (or email address for the polite pool).
  * `graph_type`: The structure of the generated graph (e.g., `OpenAlexGraphType.Coauthorship`).
  * `search`: A broad, general search query across OpenAlex works.
  * `author`: Searches for an author by name. *Note: This will automatically use the first author that appears in the search results.*
  * `author_id`: A precise search using a specific OpenAlex Author ID.
  * `author_orcid`: A precise search using a specific author's ORCID.
  * `keyword`: Filters works associated with a specific keyword.
  * `limit`: The maximum number of items to retrieve from the API.
  * `min_weight`: The minimum weight an edge must have to be included in the final graph.
  

## Python Examples

```python
from netfog import Graph, Node, OpenAlexGraphType

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

# Example: Co-authorship graph using a precise ORCID
g_openalex = Graph.from_openalex(
    api_key="your_email@example.com", 
    graph_type=OpenAlexGraphType.Coauthorship, 
    author_orcid="0000-0002-1825-0097",
    limit=100,
    min_weight=2
)
print("OpenAlex Graph Nodes:", len(g_openalex.get_nodes()))
```
