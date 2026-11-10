\name{graph-construction}
\alias{new_graph}
\alias{add_node}
\alias{add_nodes}
\alias{create_connection}
\title{Create and mutate a graph}
\description{
Construct a new \code{netfog_graph} and add nodes or edges to it.
}
\usage{
new_graph(structure = "adjacency_list")
add_node(graph, label)
add_nodes(graph, count)
create_connection(graph, from, to, weight = 1, directed = FALSE)
}
\arguments{
  \item{structure}{character scalar. Internal storage structure to use:
    one of \code{"adjacency_list"} (default), \code{"adjacency_matrix"},
    \code{"compressed_sparse_row"}, \code{"packed_compressed_sparse_row"}
    or \code{"bfs_csr"}. Any other value falls back to
    \code{"adjacency_list"}.}
  \item{graph}{a \code{netfog_graph}, as returned by \code{new_graph}.}
  \item{label}{character scalar. The node's label.}
  \item{count}{integer scalar. Number of anonymous (id-only) nodes to
    add. A graph must use either labeled nodes (\code{add_node}) or
    id-only nodes (\code{add_nodes}) exclusively -- mixing the two on
    the same graph raises an error.}
  \item{from}{node reference (character label or numeric id) for the
    edge's source.}
  \item{to}{node reference (character label or numeric id) for the
    edge's target.}
  \item{weight}{numeric scalar. Edge weight, default \code{1}.}
  \item{directed}{logical scalar. Whether the edge is directed,
    default \code{FALSE}.}
}
\value{
\code{new_graph} returns a \code{netfog_graph} object (an opaque
pointer wrapped in a list, with an S3 class for dispatch). The other
functions return their argument invisibly -- they mutate the
underlying Rust graph in place.
}
\examples{
\dontrun{
g <- new_graph("adjacency_list")
add_node(g, "A")
add_node(g, "B")
create_connection(g, "A", "B", weight = 2, directed = FALSE)
}
}
