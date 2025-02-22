from graph.graph import Graph
from layouts.layouts import RandomLayout, CircularLayout
import json

matriz_adjacencia = [
    [0, 1, 0, 1, 0, 0],
    [1, 0, 1, 0, 1, 0],
    [0, 1, 0, 1, 0, 1],
    [1, 0, 1, 0, 1, 0],
    [0, 1, 0, 1, 0, 1],
    [0, 0, 1, 0, 1, 0]
]

grafo = Graph.from_net_file('data (2).net')

grafo.output_html('teste2.html')

