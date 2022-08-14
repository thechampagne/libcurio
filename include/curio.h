#ifndef __CURIO_H__
#define __CURIO_H__

#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    char* raw;
    char* protocol;
    ptrdiff_t status;
    char* status_text;
    void* headers;
    size_t header_count;
    void* cookies;
    size_t cookie_count;
    char* body;
    char** warnings;
    size_t warnings_count;
} curio_t;
  
extern int curio_request_get(curio_t* curio, const char* url);

#ifdef __cplusplus
}
#endif

#endif // __CURIO_H__