/* Copyright (c) 2012  Dmitry Xmelkov
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

/* Test of progmem types (this types are deprecated): array without init.
   $Id: progtype-3.c 2401 2013-05-13 15:24:14Z joerg_wunsch $	*/

#if	__GNUC__ > 4 || (__GNUC__ == 4 && __GNUC_MINOR__ >= 2)
# pragma GCC diagnostic ignored "-Wdeprecated-declarations"
#endif

#ifndef __AVR__

/* Omit the test.	*/
int main ()	{ return 0; }

#else

#define __PROG_TYPES_COMPAT__
#include <avr/pgmspace.h>

static const prog_char s[10] = { 0 };

int main ()
{
    size_t i;

    if (sizeof (s) != 10)
	return __LINE__;
    for (i = 0; i < sizeof (s); i++) {
	if (pgm_read_byte (s + i))
	    return __LINE__;
    }

    return 0;
}

#endif	/* __AVR__ */
