/* Copyright (c) 2008  Dmitry Xmelkov
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

/* Test of scanf(): 'c' conversion directive.
   $Id: sscanf-c1.c 2282 2012-01-05 04:01:28Z dmix $	*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "progmem.h"

#ifdef	__AVR__
# define ASSERT(expr)			\
    do {				\
	if (!(expr)) exit(__LINE__);	\
    } while (0)
# define EXIT(v)	exit (v)
# if defined(__AVR_ATmega128__)
  /* ATmega128 has enough RAM for sprintf(), print to 0x2000 in XRAM. */
#  define PRINTF(f...)	sprintf((char *)0x2000, f)
# else
  /* small AVR */
#  define PRINTF(f...)
# endif
#else
# include <assert.h>
# define ASSERT(expr)	assert (expr)
# define EXIT(v)	exit ((v) < 256 ? (v) : 255)
# define PRINTF(f...)	printf (f)
# define sscanf_P	sscanf
# define memcmp_P	memcmp
#endif

/* Next variables are useful to debug the AVR.	*/
int vrslt = 1;
struct {
    char s[12];
} v = { {1} };

void Check (int line, int expval, int rslt)
{
    vrslt = rslt;
    if (rslt != expval) {
	PRINTF ("\nLine %d:  expect= %d, rslt= %d\n", line, expval, rslt);
	EXIT (line);
    }
}

/* The sscanf() is called 4 times: SRAM and FLASH format, 2 values
   to fill before run.	*/
#define CHECK(expval, ass_expr, str, fmt, ...)				\
    do {								\
	PROGMEM static const char fmt_p[] = fmt;			\
	char str_s[220];						\
	char fmt_s[40];							\
	char FILL;							\
	int i;								\
	int (* volatile vp)(const char *, const char *, ...);		\
									\
	ASSERT (sizeof(str_s) >= sizeof(str));				\
	ASSERT (sizeof(fmt_s) >= sizeof(fmt_p));			\
	strcpy_P (str_s, PSTR(str));					\
	strcpy_P (fmt_s, fmt_p);					\
									\
	for (FILL = 0; FILL < 4; FILL++) {				\
	    memset (&v, FILL, sizeof(v));				\
	    vp = (FILL & 1) ? sscanf_P : sscanf;			\
	    i = vp (str_s, (FILL & 1) ? fmt_p : fmt_s, ##__VA_ARGS__);	\
	    Check (__LINE__, expval, i);				\
    	    ASSERT (ass_expr);						\
	}								\
    } while (0)

int main ()
{
    /* Empty input.	*/
    CHECK (-1, 1, "", "%c", v.s);
    CHECK (-1, 1, "", " %c", v.s);
    CHECK (-1, 1, " ", " %c", v.s);
    CHECK (-1, 1, " ", "  %c", v.s);
    CHECK (-1, 1, "\t\n\v\f\r", " %c", v.s);
    
    /* Normal conversion.	*/
    CHECK (1, (v.s[0] == 'a'), "a", "%c", v.s);
    CHECK (3, !memcmp_P (v.s, PSTR(" \001\377"), 3),
	   " \001\377", "%c%c%c", v.s, v.s + 1, v.s + 2);
    CHECK (4, !memcmp_P (v.s, PSTR("DCBA"), 4),
	   "ABCD", "%c%c%c%c", v.s + 3, v.s + 2, v.s + 1, v.s);

    /* Input match.	*/
    CHECK (1, (v.s[0] == 'q'), "%The        %q", "%%The %%%c", v.s);

    /* Suppress a writing.	*/
    CHECK (0, (v.s[0] == FILL), "a", "%*c", v.s);
    CHECK (3, !memcmp_P(v.s, PSTR("ACD"), 3),
	   "ABCD", "%c%*c%c%c", v.s, v.s + 1, v.s + 2);

    /* Width field.	*/
    CHECK (1, (v.s[0] == 'A'), "A", "%1c", v.s);
    CHECK (1, !memcmp_P(v.s, PSTR("AB"), 2), "AB", "%2c", v.s);
    CHECK (1, !memcmp_P(v.s, PSTR("The_quick_br"), 12),
	   "The_quick_brown_fox", "%12c", v.s);
    CHECK (1, !memcmp_P(v.s, PSTR("A\t D"), 4), "A\t D", "%4c", v.s);
    CHECK (2, !memcmp_P(v.s, PSTR("3412"), 4),
	   "1234", "%2c%2c", v.s + 2, v.s);

    /* Suppress and width.	*/
    CHECK (0, (v.s[0] == FILL), "A", "%*1c", v.s);
    CHECK (0, (v.s[0] == FILL), "AA", "%*2c", v.s);

    return 0;
}
