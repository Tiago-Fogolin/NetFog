new_graph <- function(structure = "adjacency_list") {
    ptr <- .Call("wrap__RGraph__new", structure, PACKAGE = "netfogr")
    return(structure(list(ptr = ptr), class = "netfog_graph"))
}

add_node <- function(graph, label) {
    invisible(.Call("wrap__RGraph__add_node", graph$ptr, label, PACKAGE = "netfogr"))
}

add_nodes <- function(graph, count) {
    invisible(.Call("wrap__RGraph__add_nodes", graph$ptr, as.integer(count), PACKAGE = "netfogr"))
}

create_connection <- function(graph, from, to, weight = 1, directed = FALSE) {
    invisible(.Call(
        "wrap__RGraph__create_connection",
        graph$ptr,
        from,
        to,
        as.double(weight),
        as.logical(directed),
        PACKAGE = "netfogr"
    ))
}

node_by_label <- function(graph, node) {
    return(.Call("wrap__RGraph__node_by_label", graph$ptr, node, PACKAGE = "netfogr"))
}

get_all_edges <- function(graph) {
    return(.Call("wrap__RGraph__get_all_edges", graph$ptr, PACKAGE = "netfogr"))
}

generate_adjacency_matrix <- function(graph) {
    return(.Call("wrap__RGraph__generate_adjacency_matrix", graph$ptr, PACKAGE = "netfogr"))
}

get_total_weight <- function(graph) {
    return(.Call("wrap__RGraph__get_total_weight", graph$ptr, PACKAGE = "netfogr"))
}

get_mean_weight <- function(graph) {
    return(.Call("wrap__RGraph__get_mean_weight", graph$ptr, PACKAGE = "netfogr"))
}

get_node_count <- function(graph) {
    return(.Call("wrap__RGraph__get_node_count", graph$ptr, PACKAGE = "netfogr"))
}

get_edge_count <- function(graph) {
    return(.Call("wrap__RGraph__get_edge_count", graph$ptr, PACKAGE = "netfogr"))
}

get_density <- function(graph, directed = FALSE) {
    return(.Call("wrap__RGraph__get_density", graph$ptr, as.logical(directed), PACKAGE = "netfogr"))
}

get_average_degree <- function(graph, directed = FALSE) {
    return(.Call("wrap__RGraph__get_average_degree", graph$ptr, as.logical(directed), PACKAGE = "netfogr"))
}

compute_degrees <- function(graph, node) {
    return(.Call("wrap__RGraph__compute_degrees", graph$ptr, node, PACKAGE = "netfogr"))
}

get_centrality_degrees <- function(graph, node) {
    return(.Call("wrap__RGraph__get_centrality_degrees", graph$ptr, node, PACKAGE = "netfogr"))
}

get_node_strength <- function(graph, node) {
    return(.Call("wrap__RGraph__get_node_strength", graph$ptr, node, PACKAGE = "netfogr"))
}

compute_entropy <- function(graph) {
    return(.Call("wrap__RGraph__compute_entropy", graph$ptr, PACKAGE = "netfogr"))
}

get_max_possible_entropy <- function(graph) {
    return(.Call("wrap__RGraph__get_max_possible_entropy", graph$ptr, PACKAGE = "netfogr"))
}

get_skewness <- function(graph) {
    return(.Call("wrap__RGraph__get_skewness", graph$ptr, PACKAGE = "netfogr"))
}

dfs <- function(graph, start) {
    return(.Call("wrap__RGraph__dfs", graph$ptr, start, PACKAGE = "netfogr"))
}

bfs <- function(graph, start) {
    return(.Call("wrap__RGraph__bfs", graph$ptr, start, PACKAGE = "netfogr"))
}

dijkstra <- function(graph, start) {
    return(.Call("wrap__RGraph__dijkstra", graph$ptr, start, PACKAGE = "netfogr"))
}

connected_components <- function(graph) {
    return(.Call("wrap__RGraph__connected_components", graph$ptr, PACKAGE = "netfogr"))
}

pagerank <- function(graph, damping = 0.85, max_iter = 100, tolerance = 1e-6) {
    return(.Call(
        "wrap__RGraph__pagerank",
        graph$ptr,
        as.double(damping),
        as.integer(max_iter),
        as.double(tolerance),
        PACKAGE = "netfogr"
    ))
}

triangle_count <- function(graph) {
    return(.Call("wrap__RGraph__triangle_count", graph$ptr, PACKAGE = "netfogr"))
}

triangle_count_by_node <- function(graph) {
    return(.Call("wrap__RGraph__triangle_count_by_node", graph$ptr, PACKAGE = "netfogr"))
}

betweenness_centrality <- function(graph) {
    return(.Call("wrap__RGraph__betweenness_centrality", graph$ptr, PACKAGE = "netfogr"))
}

closeness_centrality <- function(graph, node) {
    return(.Call("wrap__RGraph__closeness_centrality", graph$ptr, node, PACKAGE = "netfogr"))
}

clustering_coefficient <- function(graph, node) {
    return(.Call("wrap__RGraph__clustering_coefficient", graph$ptr, node, PACKAGE = "netfogr"))
}

transitivity <- function(graph) {
    return(.Call("wrap__RGraph__transitivity", graph$ptr, PACKAGE = "netfogr"))
}

k_core_decomposition <- function(graph) {
    return(.Call("wrap__RGraph__k_core_decomposition", graph$ptr, PACKAGE = "netfogr"))
}

output_svg <- function(graph, layout = "random", override_positions = FALSE) {
    return(.Call(
        "wrap__RGraph__output_svg",
        graph$ptr,
        layout,
        as.logical(override_positions),
        PACKAGE = "netfogr"
    ))
}

output_html <- function(graph, file_name, layout = "random", override_positions = FALSE) {
    invisible(.Call(
        "wrap__RGraph__output_html",
        graph$ptr,
        file_name,
        layout,
        as.logical(override_positions),
        PACKAGE = "netfogr"
    ))
}

output_net_file <- function(graph, path) {
    invisible(.Call("wrap__RGraph__output_net_file", graph$ptr, path, PACKAGE = "netfogr"))
}

output_json_file <- function(graph, path) {
    invisible(.Call("wrap__RGraph__output_json_file", graph$ptr, path, PACKAGE = "netfogr"))
}

output_mtx_file <- function(graph, path) {
    invisible(.Call("wrap__RGraph__output_mtx_file", graph$ptr, path, PACKAGE = "netfogr"))
}

output_edge_list_file <- function(graph, path) {
    invisible(.Call("wrap__RGraph__output_edge_list_file", graph$ptr, path, PACKAGE = "netfogr"))
}

print.netfog_graph <- function(x, ...) {
    cat("<netfog_graph>", get_node_count(x), "nodes,", get_edge_count(x), "edges\n")
    return(invisible(x))
}
