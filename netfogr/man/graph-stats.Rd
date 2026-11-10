\name{graph-stats}
\alias{compute_degrees}
\alias{get_centrality_degrees}
\alias{get_node_strength}
\alias{compute_entropy}
\alias{get_max_possible_entropy}
\alias{get_skewness}
\title{Degree-based network statistics}
\description{
Per-node degree/strength/centrality figures, and whole-graph degree
entropy and skewness.
}
\usage{
compute_degrees(graph, node)
get_centrality_degrees(graph, node)
get_node_strength(graph, node)
compute_entropy(graph)
get_max_possible_entropy(graph)
get_skewness(graph)
}
\arguments{
  \item{graph}{a \code{netfog_graph}.}
  \item{node}{node reference (character label or numeric id).}
}
\value{
\code{compute_degrees} returns a named list with \code{in_degree},
\code{out_degree}, \code{total_degree} and \code{undirected_degree}.
\code{get_centrality_degrees} returns the same four quantities
normalized by \code{node_count - 1}. \code{get_node_strength} returns
\code{in_strength}, \code{out_strength} and \code{total_strength}
(weighted degree). \code{compute_entropy} returns a named list with
\code{in_entropy}, \code{out_entropy} and \code{undirected_entropy}.
\code{get_max_possible_entropy} returns a numeric scalar
(\code{log(node_count - 1)}). \code{get_skewness} returns a named list
with \code{in_skewness}, \code{out_skewness} and
\code{undirected_skewness}.
}
\examples{
\dontrun{
g <- new_graph()
add_node(g, "A")
add_node(g, "B")
create_connection(g, "A", "B", weight = 2)
compute_degrees(g, "A")
get_node_strength(g, "A")
}
}
