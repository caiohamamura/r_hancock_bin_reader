#include <R.h>
#include <Rinternals.h>


SEXP c_sum2(SEXP a) {
  int n = length(a);
  SEXP out = PROTECT(allocVector(INTSXP, 1));
  int *res, *input; 
  res = INTEGER(out);
  *res = 0;
  input = INTEGER(a);
  
  for (int i=0;i<n;i++) {
    *res += input[i];
  }
  UNPROTECT(1);
  
  return out;
}

SEXP c_add_one(SEXP a) {
  int n = length(a);
  SEXP out = PROTECT(allocVector(INTSXP, n));
  int *res, *input; 
  res = INTEGER(out);
  input = INTEGER(a);
  
  for (int i=0;i<n;i++) {
    res[i] += input[i]+1;
  }
  UNPROTECT(1);
  
  return out;
}