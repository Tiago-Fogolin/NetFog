\name{graph-query}
\alias{node_by_label}
\alias{get_all_edges}
\alias{generate_adjacency_matrix}
\alias{get_total_weight}
\alias{get_mean_weight}
\alias{get_node_count}
\alias{get_edge_count}
\alias{get_density}
\alias{get_average_degree}
\title{Query basic graph structure}
\description{
Read-only queries over a \code{netfog_graph}: node lookup, edge
listing, matrix export, weight/count/density summaries.
}
\usage{
node_by_label(graph, node)
get_all_edges(graph)
generate_adjacency_matrix(graph)
get_total_weight(graph)
get_mean_weight(graph)
get_node_count(graph)
get_edge_count(graph)
get_density(graph, directed = FALSE)
get_average_degree(graph, directed = FALSE)
}
\arguments{
  \item{graph}{a \code{netfog_graph}.}
  \item{node}{node reference (character label or numeric id).}
  \item{directed}{logical scalar. Whether to treat the graph as
    directed when computing the summary, default \code{FALSE}.}
}
\value{
\code{node_by_label} returns a named list with \code{label}, \code{id},
\code{x} and \code{y} (the last two are \code{NA} unless a layout has
been computed). \code{get_all_edges} returns a list of named lists,
each with \code{from}, \code{to}, \code{weight} and \code{directed}.
\code{generate_adjacency_matrix} returns a numeric matrix. The
remaining functions return a numeric (or integer) scalar.
}
\examples{
\dontrun{
g <- new_graph()
add_node(g, "A")
add_node(g, "B")
create_connection(g, "A", "B")
get_node_count(g)
get_all_edges(g)
}
}
