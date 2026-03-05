# NetFog

> [!CAUTION]
> **Work in Progress:** This library is in its early stages of development (Alpha).
> 
> * **Unstable API:** Functions and class structures may change significantly between versions.
> * **Bugs Expected:** You might encounter unexpected behavior. Please feel free to open an issue!

---

## Overview

**NetFog** is a library for creating and manipulating graphs.
Currently supports / planned features:
- Directed and undirected graphs
- Weighted edges
- Metrics
- Algorithms like BFS, DFS, Dijkstra
- Graph visualization
- Simulations

This library is designed to be **lightweight, fast, and easy to use**.

---

## Installation
You can install NetFog directly from PyPI using pip:
```bash
pip install netfog
```

## Quick Start

```python
from netfog import Graph, Node

# Creates the graph object
g = Graph()

# Add nodes
g.add_node("A")
g.add_node("B")

# Add a connection (edge or arc)
# Here we use the label of the nodes for easier manipulation
g.create_connection("A", "B", weight=10, directed=False)

# Check all connections
print(g.get_connections())
```

## Bibliometric Networks via OpenAlex

```python
from netfog import Graph, OpenAlexGraphType

# 1. Grafo de coautoria baseado em um termo de busca
grafo_coautoria = Graph.from_openalex(
    api_key="SUA_CHAVE_API_AQUI",
    graph_type=OpenAlexGraphType.Coauthorship,
    search="Bibliometrics",
    limit=100
)

# 2. Grafo de coocorrência de palavras-chave (intensidade mínima: 3)
grafo_coocorrencia = Graph.from_openalex(
    api_key="SUA_CHAVE_API_AQUI",
    graph_type=OpenAlexGraphType.KeywordCooccurrence,
    keyword="Bibliometrics",
    limit=100,
    min_weight=3
)

# 3. Grafo de cocitação de documentos baseado em um autor específico
grafo_cocitacao_documentos = Graph.from_openalex(
    api_key="SUA_CHAVE_API_AQUI",
    graph_type=OpenAlexGraphType.WorkCocitation,
    author="Jesus Mena-Chalco",
    limit=100
)

# 4. Grafo de cocitação de autores baseado em um autor específico
grafo_cocitacao_autores = Graph.from_openalex(
    api_key="SUA_CHAVE_API_AQUI",
    graph_type=OpenAlexGraphType.AuthorCocitation,
    author="Jesus Mena-Chalco",
    limit=100
)
```

For full documentation, see [docs](https://github.com/Tiago-Fogolin/NetFog/blob/master/docs/index.md)
