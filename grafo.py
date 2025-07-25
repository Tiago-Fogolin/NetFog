from graph.graph import Graph
from layouts.layouts import RandomLayout, CircularLayout
from style.graph_style import GraphStyle
import json
import time

matriz = [
    [0, 0, 2],
    [0, 0, 0],
    [1, 0, 0]
]


grafo = Graph.from_adjacency_matrix(matriz, True)

grafo.output_html("grafo.html")