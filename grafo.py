from graph.graph import Graph
from layouts.layouts import RandomLayout, CircularLayout


matriz_adjacencia = [
    [0, 1, 0, 1, 0, 0],
    [1, 0, 1, 0, 1, 0],
    [0, 1, 0, 1, 0, 1],
    [1, 0, 1, 0, 1, 0],
    [0, 1, 0, 1, 0, 1],
    [0, 0, 1, 0, 1, 0]
]

grafo = Graph.from_net_file('netnet.net')

grafo.output_html('testefinal')


