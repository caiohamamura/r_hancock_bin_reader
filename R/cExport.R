# In R ----------------------------------------
c_sum <- function(a) {
  .Call('c_sum2', PACKAGE = 'MyFirstRust', a)
}

# In R ----------------------------------------
c_add_one <- function(a) {
  .Call('c_add_one', PACKAGE = 'MyFirstRust', a)
}