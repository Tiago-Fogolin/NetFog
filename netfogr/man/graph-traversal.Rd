\name{graph-traversal}
\alias{dfs}
\alias{bfs}
\alias{dijkstra}
\title{Traversal and shortest paths}
\description{
Depth-first and breadth-first traversal order, and single-source
shortest-path distances (Dijkstra, non-negative weights).
}
\usage{
dfs(graph, start)
bfs(graph, start)
dijkstra(graph, start)
}
\arguments{
  \item{graph}{a \code{netfog_graph}.}
  \item{start}{node reference (character label or numeric id) to start
    the traversal/search from.}
}
\value{
\code{dfs} and \code{bfs} return an integer vector of node ids in
visitation order. \code{dijkstra} returns a numeric vector of
distances indexed by node id (0-based), with \code{Inf} for
unreachable nodes.
}
\examples{
\dontrun{
g <- new_graph()
add_node(g, "A")
add_node(g, "B")
add_node(g, "C")
create_connection(g, "A", "B")
create_connection(g, "B", "C")
dijkstra(g, "A")
}
}
