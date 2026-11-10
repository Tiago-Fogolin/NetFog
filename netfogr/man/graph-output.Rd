\name{graph-output}
\alias{output_svg}
\alias{output_html}
\alias{output_net_file}
\alias{output_json_file}
\alias{output_mtx_file}
\alias{output_edge_list_file}
\title{Render and export a graph}
\description{
Compute a layout and render the graph as SVG (returned as a string) or
HTML (written to a file), or export its edges/nodes in one of the
supported file formats.
}
\usage{
output_svg(graph, layout = "random", override_positions = FALSE)
output_html(graph, file_name, layout = "random", override_positions = FALSE)
output_net_file(graph, path)
output_json_file(graph, path)
output_mtx_file(graph, path)
output_edge_list_file(graph, path)
}
\arguments{
  \item{graph}{a \code{netfog_graph}.}
  \item{layout}{character scalar, one of \code{"random"} (default),
    \code{"circular"}, \code{"spring"} or \code{"force_atlas2"}. Any
    other value falls back to \code{"random"}.}
  \item{override_positions}{logical scalar. If \code{FALSE} (default)
    and the graph already has positions computed, they are reused
    instead of recomputing the layout.}
  \item{file_name, path}{character scalar, the output file path.}
}
\details{
Styling (node color, marker shape, line width, etc.) is not yet
exposed to R -- \code{output_svg}/\code{output_html} always use the
default \code{GraphStyle} from the Rust side.
}
\value{
\code{output_svg} returns the SVG document as a character scalar. The
other functions write to \code{file_name}/\code{path} and return
\code{NULL} invisibly.
}
\examples{
\dontrun{
g <- new_graph()
add_node(g, "A")
add_node(g, "B")
create_connection(g, "A", "B")
svg <- output_svg(g, layout = "circular")
output_json_file(g, tempfile(fileext = ".json"))
}
}
