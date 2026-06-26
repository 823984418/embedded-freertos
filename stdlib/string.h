#ifndef STRING_H
#define STRING_H

extern __attribute__((nothrow)) void *memcpy(void *__restrict /*s1*/, const void *__restrict /*s2*/, size_t /*n*/)
    __attribute__((__nonnull__(1, 2)));

extern __attribute__((nothrow)) void *memset(void * /*s*/, int /*c*/, size_t /*n*/)
    __attribute__((__nonnull__(1)));

#endif
