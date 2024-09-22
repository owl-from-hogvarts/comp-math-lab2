/*-
 * Copyright (c) 1992, 1993
 *	The Regents of the University of California.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * *  Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * *  Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * *  Neither the name of the University nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE REGENTS AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */

/* $Id: assert.c 1944 2009-04-01 23:12:20Z arcanum $ */

#if defined(LIBC_SCCS) && !defined(lint)
static char sccsid[] = "@(#)assert.c	8.1 (Berkeley) 6/4/93";
#endif /* LIBC_SCCS and not lint */

/* taken from:
__FBSDID("$FreeBSD: src/lib/libc/gen/assert.c,v 1.7 2002/02/01 00:57:29 obrien Exp $");
*/

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include "sectionname.h"

ATTRIBUTE_CLIB_SECTION
void
__assert (const char *func, const char *file, int line, const char *failedexpr)
{
	if (func == NULL)
		(void)fprintf(stderr,
		     "Assertion failed: (%s), file %s, line %d.\n", failedexpr,
		     file, line);
	else
		(void)fprintf(stderr,
		     "Assertion failed: (%s), function %s, file %s, line %d.\n",
		     failedexpr, func, file, line);
	abort();
	/* NOTREACHED */
}
