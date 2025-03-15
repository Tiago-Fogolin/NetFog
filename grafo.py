from graph.graph import Graph
from layouts.layouts import RandomLayout, CircularLayout
import json
import time

matriz_adjacencia = [
    [0, 1, 0, 1, 0, 0],
    [1, 0, 1, 0, 1, 0],
    [0, 1, 0, 1, 0, 1],
    [1, 0, 1, 0, 1, 0],
    [0, 1, 0, 1, 0, 1],
    [0, 0, 1, 0, 1, 0]
]
grafo = Graph.from_adjacency_matrix(matriz_adjacencia, directed=True)

grafo.output_html('teste.html')
