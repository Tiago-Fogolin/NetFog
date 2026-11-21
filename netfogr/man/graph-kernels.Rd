\name{graph-kernels}
\alias{connected_components}
\alias{pagerank}
\alias{triangle_count}
\alias{triangle_count_by_node}
\alias{betweenness_centrality}
\title{Graph kernels (GAP Benchmark Suite style)}
\description{
Connected components (union-find), PageRank (Gauss-Seidel power
iteration), triangle counting (forward algorithm) and betweenness
centrality (Brandes, unweighted).
}
\usage{
connected_components(graph)
pagerank(graph, damping = 0.85, max_iter = 100, tolerance = 1e-6)
triangle_count(graph)
triangle_count_by_node(graph)
betweenness_centrality(graph)
}
\arguments{
  \item{graph}{a \code{netfog_graph}.}
  \item{damping}{numeric scalar in (0, 1), the PageRank damping
    factor, default \code{0.85}.}
  \item{max_iter}{integer scalar, maximum power-iteration steps,
    default \code{100}.}
  \item{tolerance}{numeric scalar, convergence threshold on total
    rank delta, default \code{1e-6}.}
}
\value{
\code{connected_components} returns an integer vector of component ids
indexed by node id. \code{pagerank} and \code{betweenness_centrality}
return a numeric vector indexed by node id. \code{triangle_count}
returns a single integer (total triangles in the graph).
\code{triangle_count_by_node} returns an integer vector of per-node
triangle counts.
}
\examples{
\dontrun{
g <- new_graph()
add_node(g, "A")
add_node(g, "B")
add_node(g, "C")
create_connection(g, "A", "B")
create_connection(g, "B", "C")
create_connection(g, "C", "A")
triangle_count(g)
pagerank(g)
}
}
