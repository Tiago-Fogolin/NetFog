dll_path <- "target/x86_64-pc-windows-gnu/debug/netfogr.dll"
dyn.load(dll_path)
cat("dyn.load ok\n")

meta <- .Call("wrap__get_netfogr_metadata", PACKAGE = "netfogr")
cat("metadata ok, length:", length(meta), "\n")

g <- .Call("wrap__RGraph__new", "adjacency_list", PACKAGE = "netfogr")
cat("new ok\n")

.Call("wrap__RGraph__add_node", g, "A", PACKAGE = "netfogr")
.Call("wrap__RGraph__add_node", g, "B", PACKAGE = "netfogr")
.Call("wrap__RGraph__add_node", g, "C", PACKAGE = "netfogr")
cat("add_node ok\n")

count <- .Call("wrap__RGraph__get_node_count", g, PACKAGE = "netfogr")
cat("node count:", count, "\n")

.Call("wrap__RGraph__create_connection", g, "A", "B", 1.0, FALSE, PACKAGE = "netfogr")
.Call("wrap__RGraph__create_connection", g, "B", "C", 1.0, FALSE, PACKAGE = "netfogr")
.Call("wrap__RGraph__create_connection", g, "C", "A", 1.0, FALSE, PACKAGE = "netfogr")
cat("create_connection ok\n")

triangles <- .Call("wrap__RGraph__triangle_count", g, PACKAGE = "netfogr")
cat("triangle_count:", triangles, "\n")

closeness <- .Call("wrap__RGraph__closeness_centrality", g, "A", PACKAGE = "netfogr")
cat("closeness_centrality(A):", closeness, "\n")

closeness_by_id <- .Call("wrap__RGraph__closeness_centrality", g, 0, PACKAGE = "netfogr")
cat("closeness_centrality(id=0):", closeness_by_id, "\n")
