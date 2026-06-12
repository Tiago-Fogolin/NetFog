# Graph Creation & Manipulation

This section explains how to **create graphs, add nodes, and create connections** in NetFog.

---

## Python Reference

### Enums

#### `OpenAlexGraphType`
An enumeration defining the type of network to be generated from the OpenAlex data.
- `Coauthorship`: Nodes represent authors, and connections represent co-authored works.
- `KeywordCooccurrence`: Nodes represent concepts/keywords, and connections indicate they appear together in the same works.
- `WorkCocitation`: Nodes represent works, and connections indicate they are cited together by other works.
- `AuthorCocitation`: Nodes represent authors, and connections indicate they are cited together by other works.

#### `GraphStructureType`
An enumeration that controls which internal data structure backs the graph. The default is `AdjacencyList`.
- `AdjacencyMatrix`: Dense matrix — fast edge lookup, high memory usage for sparse graphs.
- `AdjacencyList`: Sparse list — efficient for graphs with few edges per node.
- `CompressedSparseRow` (CSR): Read-optimised sparse format — very fast traversal, not suitable for frequent mutations.
- `PackedCompressedSparseRow` (PCSR): Variant of CSR with packed storage; best for dense-sparse mixed workloads.

### Classes

#### `Graph`
Represents a graph. You can add nodes, create connections, inspect the graph, and construct graphs from adjacency matrices.

**Methods:**
- `add_node(label: str) -> Node`  
  Adds a new node to the graph by label and returns the created `Node` object.

- `add_nodes_from(labels: list[str]) -> None`  
  Batch-adds multiple nodes to the graph from a list of labels. More efficient than calling `add_node` repeatedly.

- `create_connection(from_label: str, to_label: str, weight: float = 0., directed: bool = False) -> None`  
  Creates a connection (edge/arc) between two nodes. You can specify the weight and whether it is directed.

- `add_edges_from(connections: list[tuple[str, str, float, bool | None]]) -> None`  
  Batch-adds multiple edges at once. Each tuple is `(from_label, to_label, weight, directed)`.

- `get_connections(from_name="from", to_name="to", use_id=False) -> list[dict]`  
  Returns a list of dictionaries, one per edge. Each dictionary contains the keys specified by `from_name` and `to_name` (defaulting to `"from"` and `"to"`), plus `"weight"` and `"directed"`. Set `use_id=True` to use numeric node IDs instead of labels.

- `get_all_edges() -> Iterable[tuple[int, int, float, bool]]`  
  Returns a lazy iterator over all edges as `(from_id, to_id, weight, directed)` tuples.

- `node_by_label(node_label: str) -> Node`  
  Looks up and returns a `Node` object by its label. Raises `ValueError` if the label is not found.

---

**Static Constructors:**

- `from_adjacency_matrix(adj_matrix: list[list[float]], directed: bool = False, custom_labels: list[str] | None = None, structure: GraphStructureType | None = None) -> Graph`  
  Creates a graph from a 2-D adjacency matrix. Optional custom labels can be provided; otherwise nodes are labelled `"0"`, `"1"`, …

- `from_net_file(file_path: str, structure: GraphStructureType | None = None) -> Graph`  
  Creates a graph from a `.net` (Pajek) file. This method supports node labels, spatial coordinates ($x$, $y$), and weighted connections defined in the file.

- `from_json_file(file_path: str, structure: GraphStructureType | None = None) -> Graph`  
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

- `from_mtx_file(file_path: str, structure: GraphStructureType | None = None) -> Graph`  
  Creates a graph from a Matrix Market (`.mtx`) file.

- `from_edge_list_file(file_path: str, directed: bool = False, structure: GraphStructureType | None = None) -> Graph`  
  Creates a graph from a plain edge-list file, where each line contains a pair of node labels (and an optional weight).

- `from_overpass_address(address: str, radius: float, structure: GraphStructureType | None = None) -> Graph`  
  Creates a street-network graph from OpenStreetMap data by querying the Overpass API. `address` is a free-text location string and `radius` is the search radius in metres.

- `from_synthetic(graph_type: SyntheticGraphType, structure: GraphStructureType | None = None) -> Graph`  
  Creates a graph using a synthetic generative model. See `SyntheticGraphType` for available models.

---

- `from_openalex(api_key: str, graph_type: OpenAlexGraphType, search=None, author=None, author_id=None, author_orcid=None, keyword=None, limit=None, min_weight=None, save_json_path=None, structure: GraphStructureType | None = None) -> Graph`  
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
  * `save_json_path`: If provided, saves the raw API response JSON to this file path.
  

## Python Examples

```python
from netfog import Graph, Node, OpenAlexGraphType, SyntheticGraphType, GraphStructureType

# Create a graph (default: AdjacencyList)
g = Graph()

# Add a single node
g.add_node("A")

# Batch-add nodes
g.add_nodes_from(["B", "C", "D"])

# Create a single connection
g.create_connection("A", "B", weight=10, directed=False)

# Batch-add edges
g.add_edges_from([
    ("B", "C", 3.0, False),
    ("C", "D", 1.5, True),
])

# Inspect connections
print("Connections:", g.get_connections())

# Iterate over all edges (lazy)
for from_id, to_id, weight, directed in g.get_all_edges():
    print(from_id, "->", to_id, weight, directed)

# Look up a node by label
node_b = g.node_by_label("B")
print("Node B id:", node_b.id)

# --- Static constructors ---

# From adjacency matrix
adj_matrix = [
    [0, 1, 0],
    [0, 0, 1],
    [1, 0, 0]
]
g2 = Graph.from_adjacency_matrix(adj_matrix, directed=True, custom_labels=["X", "Y", "Z"])
print("Node count:", g2.get_node_count())
print("Connections:", g2.get_connections())

# From file formats
g3 = Graph.from_net_file("network.net")
g4 = Graph.from_json_file("network.json")
g5 = Graph.from_mtx_file("network.mtx")
g6 = Graph.from_edge_list_file("edges.txt", directed=False)

# From OpenStreetMap (Overpass)
g_osm = Graph.from_overpass_address("Eiffel Tower, Paris", radius=500)
print("OSM node count:", g_osm.get_node_count())

# Synthetic graphs
g_er  = Graph.from_synthetic(SyntheticGraphType.erdos_renyi(n=100, p=0.05))
g_ba  = Graph.from_synthetic(SyntheticGraphType.barabasi_albert(n=100, m=3))
g_ws  = Graph.from_synthetic(SyntheticGraphType.watts_strogatz(n=100, k=4, beta=0.3))
print("Barabasi-Albert edges:", g_ba.get_edge_count())

# From OpenAlex (academic network)
g_openalex = Graph.from_openalex(
    api_key="your_email@example.com",
    graph_type=OpenAlexGraphType.Coauthorship,
    author_orcid="0000-0002-1825-0097",
    limit=100,
    min_weight=2
)
print("OpenAlex node count:", g_openalex.get_node_count())
```
