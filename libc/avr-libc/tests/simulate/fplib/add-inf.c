/* Copyright (c) 2007  Dmitry Xmelkov
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

/* Test of addition.  Infs (without NaNs).
   'Inf - Inf' is not included into this file.
   $Id: add-inf.c 1923 2009-03-07 14:02:24Z dmix $
 */
#include <stdio.h>
#include <stdlib.h>
#include "progmem.h"

union lofl_u {
    long lo;
    float fl;
};

volatile union lofl_u v = { .lo = 1 };

PROGMEM const struct {		/* Table of test cases:  x + y = z	*/
    union lofl_u x, y, z;
} t[] = {

    /* +Inf + finite --> +Inf	*/
    { { 0x7f800000 }, { 0x00000001 }, { 0x7f800000 } },
    { { 0x7f800000 }, { 0x007fffff }, { 0x7f800000 } },
    { { 0x7f800000 }, { 0x00800000 }, { 0x7f800000 } },
    { { 0x7f800000 }, { 0x3f800000 }, { 0x7f800000 } },
    { { 0x7f800000 }, { 0x7f7fffff }, { 0x7f800000 } },
    { { 0x7f800000 }, { 0x80000001 }, { 0x7f800000 } },
    { { 0x7f800000 }, { 0x807fffff }, { 0x7f800000 } },
    { { 0x7f800000 }, { 0x80800000 }, { 0x7f800000 } },
    { { 0x7f800000 }, { 0xbf800000 }, { 0x7f800000 } },
    { { 0x7f800000 }, { 0xff7fffff }, { 0x7f800000 } },

    /* -Inf + finite --> -Inf	*/
    { { 0xff800000 }, { 0x00000001 }, { 0xff800000 } },
    { { 0xff800000 }, { 0x007fffff }, { 0xff800000 } },
    { { 0xff800000 }, { 0x00800000 }, { 0xff800000 } },
    { { 0xff800000 }, { 0x3f800000 }, { 0xff800000 } },
    { { 0xff800000 }, { 0x7f7fffff }, { 0xff800000 } },
    { { 0xff800000 }, { 0x80000001 }, { 0xff800000 } },
    { { 0xff800000 }, { 0x807fffff }, { 0xff800000 } },
    { { 0xff800000 }, { 0x80800000 }, { 0xff800000 } },
    { { 0xff800000 }, { 0xbf800000 }, { 0xff800000 } },
    { { 0xff800000 }, { 0xff7fffff }, { 0xff800000 } },

    /* finite + +Inf --> +Inf	*/
    { { 0x00000001 }, { 0x7f800000 }, { 0x7f800000 } },
    { { 0x007fffff }, { 0x7f800000 }, { 0x7f800000 } },
    { { 0x00800000 }, { 0x7f800000 }, { 0x7f800000 } },
    { { 0x3f800000 }, { 0x7f800000 }, { 0x7f800000 } },
    { { 0x7f7fffff }, { 0x7f800000 }, { 0x7f800000 } },
    { { 0x80000001 }, { 0x7f800000 }, { 0x7f800000 } },
    { { 0x807fffff }, { 0x7f800000 }, { 0x7f800000 } },
    { { 0x80800000 }, { 0x7f800000 }, { 0x7f800000 } },
    { { 0xbf800000 }, { 0x7f800000 }, { 0x7f800000 } },
    { { 0xff7fffff }, { 0x7f800000 }, { 0x7f800000 } },

    /* finite + -Inf --> -Inf	*/
    { { 0x00000001 }, { 0xff800000 }, { 0xff800000 } },
    { { 0x007fffff }, { 0xff800000 }, { 0xff800000 } },
    { { 0x00800000 }, { 0xff800000 }, { 0xff800000 } },
    { { 0x3f800000 }, { 0xff800000 }, { 0xff800000 } },
    { { 0x7f7fffff }, { 0xff800000 }, { 0xff800000 } },
    { { 0x80000001 }, { 0xff800000 }, { 0xff800000 } },
    { { 0x807fffff }, { 0xff800000 }, { 0xff800000 } },
    { { 0x80800000 }, { 0xff800000 }, { 0xff800000 } },
    { { 0xbf800000 }, { 0xff800000 }, { 0xff800000 } },
    { { 0xff7fffff }, { 0xff800000 }, { 0xff800000 } },
    
    /* Inf + Inf --> Inf	*/
    { { 0x7f800000 }, { 0x7f800000 }, { 0x7f800000 } },
    { { 0xff800000 }, { 0xff800000 }, { 0xff800000 } },
    
    /* finite + finite --> Inf	*/
    { { 0x7f7fffff }, { 0x7f7fffff }, { 0x7f800000 } },
    { { 0x7f7fffff }, { 0x7f000001 }, { 0x7f800000 } },
    { { 0x7f7fffff }, { 0x7f000002 }, { 0x7f800000 } },
    { { 0x7f000001 }, { 0x7f7fffff }, { 0x7f800000 } },
    { { 0x7f000002 }, { 0x7f7fffff }, { 0x7f800000 } },
    { { 0xff7fffff }, { 0xff7fffff }, { 0xff800000 } },
    { { 0xff7fffff }, { 0xff000001 }, { 0xff800000 } },
    { { 0xff7fffff }, { 0xff000002 }, { 0xff800000 } },
    { { 0xff000001 }, { 0xff7fffff }, { 0xff800000 } },
    { { 0xff000002 }, { 0xff7fffff }, { 0xff800000 } },

};

void x_exit (int index)
{
#ifndef	__AVR__
    fprintf (stderr, "t[%d]:  %#lx\n", index - 1, v.lo);
#endif
    exit (index ? index : -1);
}

int main ()
{
    union lofl_u x,y,z;
    int i;
    
    for (i = 0; i < (int) (sizeof(t) / sizeof(t[0])); i++) {
	x.lo = pgm_read_dword (& t[i].x);
	y.lo = pgm_read_dword (& t[i].y);
	z.lo = pgm_read_dword (& t[i].z);
	v.fl = x.fl + y.fl;
	/* Comparison is integer to verify the zero sign.	*/
	if (v.lo != z.lo)
	    x_exit (i+1);
    }
    return 0;
}
