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

/* Test of 'o' specification.
   $Id: sprintf-3.c 2148 2010-06-09 14:24:29Z joerg_wunsch $	*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "progmem.h"

#ifndef __AVR__
# define sprintf_P	sprintf
#endif

void Check (int line,
            const char *expstr,
	    int retval, const char *retstr)
{
    int code;

    if (retval != (int)strlen (retstr))
	code = 1000 + line;
    else if (strcmp_P (retstr, expstr))
	code = line;
    else
	return;
#if	!defined(__AVR__)
    if (code) {
	printf ("\nLine %3d: expect: %3d, \"%s\","
		"\n          output: %3d, \"%s\"\n",
		line, strlen (expstr), expstr, retval, retstr);
	code = (line < 256 ? line : 255);	/* common OS restriction */
    }
#elif	defined(DEBUG)
    exit ((int)retstr);
#endif
    exit (code);
}

/* 'vp' is used to avoid gcc warnings about format string.	*/
#define CHECK(expstr, fmt, ...)	do {				\
    char s[260];						\
    int i;							\
    int (* volatile vp)(char *, const char *, ...) = sprintf_P;	\
    memset (s, 0, sizeof(s));					\
    i = vp (s, PSTR(fmt), ##__VA_ARGS__);			\
    Check (__LINE__, PSTR(expstr), i, s);			\
} while (0)

int main ()
{
    /* '%o' specification	*/
    CHECK ("0", "%o", 0);
    CHECK ("123456", "%o", 0123456);
    CHECK ("1 77777 100000 100001 177777",
           "%o %o %o %o %o",
	   1, 0x7fff, 0x8000, 0x8001, 0xffff);

    CHECK ("0 1 177777", "%0o %0o %0o", 0, 1, 0xffff);
    CHECK ("0 1 177777", "% o % o % o", 0, 1, 0xffff);
    CHECK ("0 1 177777", "%+o %+o %+o", 0, 1, 0xffff);
    CHECK ("0 1 177777", "%-o %-o %-o", 0, 1, 0xffff);
    CHECK ("0 01 0177777", "%#o %#o %#o", 0, 1, 0xffff);

#ifdef	__AVR__
    CHECK ("0", "%.0o", 0);
#endif
    CHECK ("1 177777", "%.0o %.0o", 1, 0xffff);
    CHECK ("0 1 177777", "%.1o %.1o %.1o", 0, 1, 0xffff);
    CHECK ("00 01 10 177777", "%.2o %.2o %.2o %.2o", 0, 1, 8, 0xffff);
    CHECK ("0000 0001 0012 0123 1234 12345",
	   "%.4o %.4o %.4o %.4o %.4o %.4o",
	   0, 1, 012, 0123, 01234, 012345);

    CHECK ("0.1.177777", "%1o.%1o.%1o", 0, 1, 0xffff);
    CHECK (" 1.12.123", "%2o.%2o.%2o", 1, 012, 0123);
    CHECK ("   1.  12. 123.1234.12345",
	   "%4o.%4o.%4o.%4o.%4o", 1, 012, 0123, 01234, 012345);
    CHECK (" 012.0123.01234", "%#4o.%#4o.%#4o", 012, 0123, 01234);
    CHECK ("  14.177777", "% 4o.% 4o", 014, 0xffff);
    CHECK ("  16.177777", "%+4o.%+4o", 016, 0xffff);
    CHECK ("17  .177777", "%-4o.%-4o", 017, 0xffff);

    CHECK ("0000.0001.0012.0123.1234.12345.177777",
	   "%04o.%04o.%04o.%04o.%04o.%04o.%04o",
	   0, 1, 012, 0123, 01234, 012345, 0177777);

    CHECK ("    0001", "%08.4o", 1);		/* 0 - ignored	*/
    CHECK ("       1", "%08.0o", 1);
    CHECK ("       1", "%08.o", 1);

    CHECK ("    0001.    0123.    1234.  177777",
	   "%8.4o.%8.4o.%8.4o.%8.4o", 1, 0123, 01234, 0xffff);
    CHECK ("    0012.    0013.    0014",
	   "%08.4o.% 8.4o.%+8.4o", 012, 013, 014);
    CHECK ("0001    .0123    .1234    .12345   ",
	   "%-8.4o.%-8.4o.%-8.4o.%-8.4o", 1, 0123, 01234, 012345);

    CHECK ("    0000.    0001.    0012.    0123.   01234",
           "%#8.4o.%#8.4o.%#8.4o.%#8.4o.%#8.4o", 0, 1, 012, 0123, 01234);

#if !defined(__AVR_AT90S8515__) /* this does not fit */
    CHECK ("                                                                "
	   "                                                                "
	   "                                                                "
	   "                                                              1",
	   "%255o", 1);
    CHECK ("1                                                               "
	   "                                                                "
	   "                                                                "
	   "                                                               ",
	   "%-255o", 1);
    CHECK ("0000000000000000000000000000000000000000000000000000000000000000"
	   "0000000000000000000000000000000000000000000000000000000000000000"
	   "0000000000000000000000000000000000000000000000000000000000000000"
	   "000000000000000000000000000000000000000000000000000000000000002",
	   "%.255o", 2);

    /* long arg	*/
    CHECK ("0 1234567 76543210 17777777777 20000000000 20000000001 37777777777",
	   "%lo %lo %lo %lo %lo %lo %lo",
	   0L, 01234567L, 076543210L,
	   0x7fffffffL, 0x80000000L, 0x80000001L, 0xffffffffL);
#endif

    /* short arg	*/
    CHECK ("0 2 177776", "%ho %ho %ho", 0, 2, 0xfffe);

    return 0;
}
