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

/* Test of itoa() function with patterns.
   $Id: itoa-2.c 2298 2012-07-29 15:30:34Z dmix $	*/

#include <stdlib.h>
#include <string.h>
#include "progmem.h"

#ifndef	__AVR__
# include <stdio.h>
# define PRINTFLN(line, fmt, ...)	\
    printf ("\n%s:%d: " fmt "\n", __FILE__, line, ##__VA_ARGS__);
# define EXIT(code)	exit ((code) < 255 ? (code) : 100 + (code) % 100)
#else
# define PRINTFLN(args...)
# define EXIT(code)	exit (code)
#endif

#ifndef	__AVR__
static char *utoa_recursive (unsigned val, char *s, unsigned radix)
{
    int c;

    if (val >= radix)
	s = utoa_recursive (val / radix, s, radix);
    c = val % radix;
    c += (c < 10 ? '0' : 'a' - 10);
    *s++ = c;
    return s;
}

static char *itoa (int val, char *s, int radix)
{
    if (radix < 2 || radix > 36) {
	s[0] = 0;
    } else {
	char *p = s;
	if (radix == 10 && val < 0) {
	    val = -val;
	    *p++ = '-';
	}
	*utoa_recursive (val, p, radix) = 0;
    }
    return s;
}
#endif

static void
Check (int line, int val, const char *rslt, int radix)
{
    char s[40];

#ifndef	__AVR__
    if (radix >= 2 && radix <= 36) {
	long x = strtol (rslt, 0, radix);
	if (x != val) {
	    PRINTFLN (line, "strtol(\"%s\",,%d)= %ld,  val= %d",
		      rslt, radix, x, val);
	    EXIT (2000 + line);
	}
    }
#endif

    memset (s, 0xff, sizeof (s) - 1);
    s[sizeof (s) - 1] = 0;
    if (itoa (val, s, radix) != s) {
	PRINTFLN (line, "Incorrect result");
	EXIT (1000 + line);
    }

    if (strcmp_P (s, rslt)) {
	PRINTFLN (line, "itoa(%d,,%d) puts: %s, must: %s",
		  val, radix, s, rslt);
	EXIT (line);
    }
}

#define CHECK(val, rslt, base)	\
    Check (__LINE__, val, PSTR (rslt), base)


int main ()
{
    int i;

    /* Invalid radix.	*/
    CHECK (0, "", 0);
    CHECK (0, "", 1);
    CHECK (0, "", 37);
    CHECK (10, "", 0x0102);
    CHECK (-256, "", -10);

    /* Decimal negative.	*/
    CHECK (-1, "-1", 10);
    CHECK (-255, "-255", 10);
    CHECK (-256, "-256", 10);
    CHECK (-257, "-257", 10);
    CHECK (-32767, "-32767", 10);
    CHECK (-32768, "-32768", 10);

    /* Only radix == 10 is signed.	*/
    for (i = 2; i <= 36; i++) {
	char s[40];
	itoa (-1, s, i);
	if (i == 10) {
	    if (s[0] != '-') exit (__LINE__);
	} else {
	    if (s[0] == '-') exit (__LINE__);
	}
    }

    CHECK (-12345, "-12345", 10);

    return 0;
}
