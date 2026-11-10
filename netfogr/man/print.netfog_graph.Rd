\name{print.netfog_graph}
\alias{print.netfog_graph}
\title{Print a netfog_graph}
\description{
S3 \code{print} method for \code{netfog_graph} objects, showing the
node and edge count instead of the raw pointer.
}
\usage{
\method{print}{netfog_graph}(x, ...)
}
\arguments{
  \item{x}{a \code{netfog_graph}.}
  \item{...}{ignored, present for S3 method compatibility.}
}
\value{
Returns \code{x} invisibly, called for its side effect of printing.
}
\examples{
\dontrun{
g <- new_graph()
print(g)
}
}
