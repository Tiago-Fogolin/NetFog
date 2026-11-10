extern "C" {
#include <R_ext/Rdynload.h>

void R_init_netfogr_extendr(DllInfo *dll);

__declspec(dllexport) void R_init_netfogr(DllInfo *dll) {
    R_init_netfogr_extendr(dll);
}
}
