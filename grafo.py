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

grafo = Graph.from_json_file('jsonexemplo.json')

grafo.output_json_file('jsonoutput.json')

