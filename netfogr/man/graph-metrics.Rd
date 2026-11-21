\name{graph-metrics}
\alias{closeness_centrality}
\alias{clustering_coefficient}
\alias{transitivity}
\alias{k_core_decomposition}
\title{Closeness, clustering and k-core metrics}
\description{
Closeness centrality (inverse of the sum of shortest-path distances),
local clustering coefficient and global transitivity, and the k-core
decomposition (Batagelj & Zaversnik degeneracy ordering).
}
\usage{
closeness_centrality(graph, node)
clustering_coefficient(graph, node)
transitivity(graph)
k_core_decomposition(graph)
}
\arguments{
  \item{graph}{a \code{netfog_graph}.}
  \item{node}{node reference (character label or numeric id).}
}
\value{
\code{closeness_centrality} and \code{clustering_coefficient} return a
numeric scalar for the given node (\code{0} if undefined, e.g. a node
with fewer than 2 neighbors for clustering coefficient, or an
unreachable node for closeness centrality). \code{transitivity}
returns a single numeric scalar for the whole graph.
\code{k_core_decomposition} returns a named list mapping each node id
(as a character key) to its coreness (integer).
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
closeness_centrality(g, "A")
transitivity(g)
k_core_decomposition(g)
}
}
