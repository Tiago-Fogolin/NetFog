from graph.graph import Graph
from layouts.layouts import RandomLayout, CircularLayout, SpringLayout
from style.graph_style import GraphStyle
import json
import time

adj_matrix = [
 [0, 1, 1, 1, 0, 0, 0, 0, 0], 
 [1, 0, 1, 0, 1, 0, 0, 0, 0], 
 [1, 1, 0, 0, 1, 0, 0, 0, 0], 
 [1, 0, 0, 0, 1, 0, 0, 1, 0], 
 [0, 1, 1, 1, 0, 1, 0, 0, 0], 
 [0, 0, 0, 0, 1, 0, 0, 0, 0],
 [0, 0, 0, 0, 1, 0, 1, 0, 0],
 [0, 0, 0, 0, 1, 0, 0, 0, 0],
 [0, 0, 0, 0, 1, 0, 0, 1, 0]
]

grafo = Graph.from_adjacency_matrix(adj_matrix)

grafo.output_html("grafo.html", layout=SpringLayout)