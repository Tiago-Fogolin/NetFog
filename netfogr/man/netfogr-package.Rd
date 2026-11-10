\name{netfogr-package}
\alias{netfogr-package}
\alias{netfogr}
\docType{package}
\title{R bindings for the netfog graph library}
\description{
\code{netfogr} exposes the same \code{graph_core} engine used by the
Python bindings (\code{netfog}), through a thin \code{extendr} layer.
A graph is represented by a \code{netfog_graph} object created with
\code{\link{new_graph}}; every other exported function takes that
object as its first argument.

Nodes can be referred to either by their character label (as given to
\code{\link{add_node}}) or by their integer id (0-based, assigned in
insertion order). Any argument documented as a \emph{node reference}
accepts either form.
}
\details{
Functions are grouped into:
\itemize{
  \item Construction and mutation: \code{\link{new_graph}}, \code{\link{add_node}}, \code{\link{add_nodes}}, \code{\link{create_connection}}
  \item Querying structure: \code{\link{node_by_label}}, \code{\link{get_all_edges}}, \code{\link{generate_adjacency_matrix}}, \code{\link{get_total_weight}}, \code{\link{get_mean_weight}}, \code{\link{get_node_count}}, \code{\link{get_edge_count}}, \code{\link{get_density}}, \code{\link{get_average_degree}}
  \item Degree-based statistics: \code{\link{compute_degrees}}, \code{\link{get_centrality_degrees}}, \code{\link{get_node_strength}}, \code{\link{compute_entropy}}, \code{\link{get_max_possible_entropy}}, \code{\link{get_skewness}}
  \item Traversal and shortest paths: \code{\link{dfs}}, \code{\link{bfs}}, \code{\link{dijkstra}}
  \item Graph kernels: \code{\link{connected_components}}, \code{\link{pagerank}}, \code{\link{triangle_count}}, \code{\link{triangle_count_by_node}}, \code{\link{betweenness_centrality}}
  \item Closeness / clustering metrics: \code{\link{closeness_centrality}}, \code{\link{clustering_coefficient}}, \code{\link{transitivity}}, \code{\link{k_core_decomposition}}
  \item Output: \code{\link{output_svg}}, \code{\link{output_html}}, \code{\link{output_net_file}}, \code{\link{output_json_file}}, \code{\link{output_mtx_file}}, \code{\link{output_edge_list_file}}
}

Not every method of the Python bindings has been ported yet (batch
constructors such as \code{add_edges_from_ids}, and the external-data
constructors \code{from_openalex}/\code{from_overpass_address} are not
exposed here). \code{graph_core} itself is untouched by the R bindings.
}
\author{Tiago Fogolin}
