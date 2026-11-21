extern "C" {
#include <R_ext/Rdynload.h>

void R_init_netfogr_extendr(DllInfo *dll);

#if defined(_WIN32) || defined(__CYGWIN__)
__declspec(dllexport)
#else
__attribute__((visibility("default")))
#endif
void R_init_netfogr(DllInfo *dll) {
    R_init_netfogr_extendr(dll);
}
}
