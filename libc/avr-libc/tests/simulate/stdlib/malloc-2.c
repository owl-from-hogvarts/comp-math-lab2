/* Copyright (c) 2010  Joerg Wunsch
   All rights reserved.

   Redistribution and use in source and binary forms, with or without
   modification, are permitted provided that the following conditions are met:

   * Redistributions of source code must retain the above copyright
     notice, this list of conditions and the following disclaimer.
   * Redistributions in binary form must reproduce the above copyright
     notice, this list of conditions and the following disclaimer in
     the documentation and/or other materials provided with the
     distribution.
   * Neither the name of the copyright holders nor the names of
     contributors may be used to endorse or promote products derived
     from this software without specific prior written permission.

   THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
   AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
   IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
   ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
   LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
   CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
   SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
   INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
   CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
   ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
   POSSIBILITY OF SUCH DAMAGE.
 */

/* $Id: malloc-2.c 2123 2010-06-04 14:38:23Z joerg_wunsch $ */

/* Setup the heap to a certain area, and make sure allocation won't go
   beyond the restriction. */

#ifndef	__AVR__

/* There is no sense to check on host computer. */
int main ()
{
    return 0;
}

#else

#include <stdint.h>
#include <stdlib.h>

struct __freelist {
        size_t sz;
        struct __freelist *nx;
};

extern char *__brkval;          /* first location not yet allocated */
extern struct __freelist *__flp; /* freelist pointer (head of freelist) */
extern char *__malloc_heap_start;
extern char *__malloc_heap_end;

#if defined(__AVR_ATmega128__)
#define HEAP_START 0x200
#define HEAP_END   0x1000
#define ALLOC_FAILS 0xe00
#define ALLOC_WORKS 0xdfe
#elif defined(__AVR_AT90S8515__)
#define HEAP_START 0x100
#define HEAP_END   0x200
#define ALLOC_FAILS 0x100
#define ALLOC_WORKS 0xfe
#else
#  error "Unknown MCU type"
#endif


int main(void)
{
    void *p;

    __malloc_heap_start = (void *)HEAP_START;
    __malloc_heap_end = (void *)HEAP_END;

    p = malloc(ALLOC_FAILS);
    if (p != NULL) return __LINE__;

    p = malloc(ALLOC_WORKS);
    if (p == NULL) return __LINE__;
    if (p != (void *)(HEAP_START + 2)) return __LINE__;

    return 0;
}

#endif  /* !AVR */
