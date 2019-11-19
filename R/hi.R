#' @useDynLib MyFirstRust 
#' @evalNamespace .ns_export(ls(environment(MyFirstRust::.ns_export)))
.ns_export <- function(nms) {
  fns = grep("^(?!rs|\\.).*$", nms, value=TRUE, perl=TRUE)
  sprintf("export(%s)", paste(fns, collapse = ","))
}