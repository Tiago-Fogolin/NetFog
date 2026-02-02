# Visualization & Export

This section covers how to visualize your graph and export it to different file formats. NetFog provides built-in support for generating vector graphics and standard network data files.

---

## Python Reference

### Enums & Classes

#### `Layout`
Specifies the positioning algorithm for the nodes:
* **Random**: Positions nodes at random coordinates.
* **Circular**: Places nodes in a perfect circle.
* **Spring**: Uses a force-directed algorithm to space nodes out (ideal for identifying clusters).

#### `GraphStyle`
Customizes the visual appearance of the graph. If not provided, a default theme is used.
* **Nodes**: `node_color`, `node_border`, `node_radius`.
* **Markers (Arrows)**: `marker_svg` (path), `marker_fill`, `marker_width`, `marker_height`.
* **Edges**: `line_color`, `line_min_width`, `line_max_width`, `dynamic_line_size` (scales width by weight).

---

### Methods

- `output_svg(layout: Layout, override_positions: bool, style: GraphStyle) -> str`  
Generates a raw SVG string of the graph. Useful for embedding in notebooks, documentation, or web applications.

- `output_html(file_name: str, layout: Layout, override_positions: bool, style: GraphStyle)`  
Wraps the generated SVG in a standalone HTML file and saves it to disk.


- `output_net_file(file_name: str)`  
Exports the graph structure to a `.net` (Pajek) format, preserving node labels and weights.

- `output_json_file(file_name: str)`  
Saves the graph as a JSON file, including node coordinates and distinguishing between directed arcs and undirected edges.
---

## Python Examples

```python
from netfog import Graph, Layout, GraphStyle

# Load your graph
g = Graph.from_json_file("network.json")

# 1. Basic Export to HTML
g.output_html("graph_view.html", layout=Layout.Spring)

# 2. Custom Styling
custom_style = GraphStyle(
    node_colo="red",
    node_radius=25,
    line_color="blue",
    dynamic_line_size=True
)

# Generate an SVG string with the Circular layout
svg_code = g.output_svg(layout=Layout.Circular, style=custom_style)

# 3. Data Export
g.output_net_file("exported_data.net")
```

[!TIP] **Override Positions**: If your graph already has specific coordinates (e.g., loaded from a file), set override_positions=False to preserve them. Set it to True if you want the Layout algorithm to recalculate new positions.