#' @useDynLib HancockBinReader 
#' @evalNamespace .ns_export(ls(environment(HancockBinReader::.ns_export)))
.ns_export <- function(nms) {
  fns = grep("^(?!rs|\\.).*$", nms, value=TRUE, perl=TRUE)
  sprintf("export(%s)", paste(fns, collapse = ","))
}