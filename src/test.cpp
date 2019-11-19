#include <Rcpp.h>


// [[Rcpp::export]]
int sum_cpp(Rcpp::IntegerVector vec) {
  int result = 0;
  for (int i = 0; i<vec.length(); i++) {
    result += vec[i];
  }
  return result;
}


// [[Rcpp::export]]
Rcpp::IntegerVector cpp_add_one(Rcpp::IntegerVector vec) {
  Rcpp::IntegerVector res(vec.length());
  for (int i = 0; i<vec.length(); i++) {
    res[i] = vec[i]+1;
  }
  return res;
}