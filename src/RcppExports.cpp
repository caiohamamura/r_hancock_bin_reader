// Generated by using Rcpp::compileAttributes() -> do not edit by hand
// Generator token: 10BE3573-1514-4C36-9D1C-5A225CD40393

#include <Rcpp.h>

using namespace Rcpp;

// sum_cpp
int sum_cpp(Rcpp::IntegerVector vec);
RcppExport SEXP _HancockBinReader_sum_cpp(SEXP vecSEXP) {
BEGIN_RCPP
    Rcpp::RObject rcpp_result_gen;
    Rcpp::RNGScope rcpp_rngScope_gen;
    Rcpp::traits::input_parameter< Rcpp::IntegerVector >::type vec(vecSEXP);
    rcpp_result_gen = Rcpp::wrap(sum_cpp(vec));
    return rcpp_result_gen;
END_RCPP
}
// cpp_add_one
Rcpp::IntegerVector cpp_add_one(Rcpp::IntegerVector vec);
RcppExport SEXP _HancockBinReader_cpp_add_one(SEXP vecSEXP) {
BEGIN_RCPP
    Rcpp::RObject rcpp_result_gen;
    Rcpp::RNGScope rcpp_rngScope_gen;
    Rcpp::traits::input_parameter< Rcpp::IntegerVector >::type vec(vecSEXP);
    rcpp_result_gen = Rcpp::wrap(cpp_add_one(vec));
    return rcpp_result_gen;
END_RCPP
}

RcppExport SEXP HancockBinReader_read_hancock_bin(SEXP);

static const R_CallMethodDef CallEntries[] = {
    {"_HancockBinReader_sum_cpp", (DL_FUNC) &_HancockBinReader_sum_cpp, 1},
    {"_HancockBinReader_cpp_add_one", (DL_FUNC) &_HancockBinReader_cpp_add_one, 1},
    {"HancockBinReader_read_hancock_bin", (DL_FUNC) &HancockBinReader_read_hancock_bin, 1},
    {NULL, NULL, 0}
};

RcppExport void R_init_HancockBinReader(DllInfo *dll) {
    R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
    R_useDynamicSymbols(dll, FALSE);
}
