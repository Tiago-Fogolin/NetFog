from graph.graph import Graph
from layouts.layouts import RandomLayout, CircularLayout
import json

matriz_adjacencia = [
    [0, 4, 0, 2, 0, 0],
    [4, 0, 1, 0, 5, 0],
    [0, 1, 0, 3, 0, 1],
    [1, 0, 2, 0, 2, 0],
    [0, 1, 0, 1, 0, 1],
    [0, 0, 1, 0, 1, 0]
]

grafo = Graph.from_adjacency_matrix(matriz_adjacencia, directed=True)

grafo.output_html('teste.html', layout=CircularLayout)

