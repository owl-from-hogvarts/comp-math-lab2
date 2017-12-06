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

/* Test of sin() function. 500 random cases for fabs(x) < 10.
   $Id: sin-500.c 1923 2009-03-07 14:02:24Z dmix $
 */
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include "progmem.h"

union lofl_u {
    long lo;
    float fl;
};

volatile union lofl_u v = { .lo = 1 };

PROGMEM const struct {		/* Table of test cases.	*/
    long x;		/* argument	*/
    long z;		/* sin(x)	*/
} t[] = {

    /* x, sin(x)	*/
    { 0xbe463583,0xbe44f940 }, /*   1: -1.9356351e-01 -1.9235706e-01 */
    { 0x3f380afe,0x3f289816 }, /*   2:  7.1891773e-01  6.5857065e-01 */
    { 0x3f067d3a,0x3f00633f }, /*   3:  5.2534831e-01  5.0151438e-01 */
    { 0xbeceac19,0xbec91af6 }, /*   4: -4.0365675e-01 -3.9278382e-01 */
    { 0x3f1fad4c,0x3f1585d6 }, /*   5:  6.2373805e-01  5.8407342e-01 */
    { 0xbf9c654f,0xbf709230 }, /*   6: -1.2218417e+00 -9.3973064e-01 */
    { 0x40afaf14,0xbf3666cd }, /*   7:  5.4901218e+00 -7.1250612e-01 */
    { 0x3f31d799,0x3f23e0ff }, /*   8:  6.9469601e-01  6.4015192e-01 */
    { 0xbf306b02,0xbf22c841 }, /*   9: -6.8913281e-01 -6.3586813e-01 */
    { 0xbef482ca,0xbeeb5281 }, /*  10: -4.7756034e-01 -4.5961383e-01 */
    { 0xbfd93ff6,0xbf7df49a }, /*  11: -1.6972644e+00 -9.9201357e-01 */
    { 0x3f3309e6,0x3f24cbda }, /*  12:  6.9936979e-01  6.4373553e-01 */
    { 0xbf14d4e8,0xbf0c9691 }, /*  13: -5.8137369e-01 -5.4917246e-01 */
    { 0x402293be,0x3f10d40e }, /*  14:  2.5402675e+00  5.6573570e-01 */
    { 0x3f31903b,0x3f23aa25 }, /*  15:  6.9360703e-01  6.3931495e-01 */
    { 0xbf0feaf4,0xbf0874b8 }, /*  16: -5.6217885e-01 -5.3303099e-01 */
    { 0xbf377155,0xbf282455 }, /*  17: -7.1657306e-01 -6.5680438e-01 */
    { 0x3f4f4acc,0x3f395ee9 }, /*  18:  8.0973506e-01  7.2410446e-01 */
    { 0xc016d14f,0xbf34f59f }, /*  19: -2.3565252e+00 -7.0687288e-01 */
    { 0xbf4ff582,0xbf39d47d }, /*  20: -8.1233990e-01 -7.2589856e-01 */
    { 0xbcf3d280,0xbcf3c949 }, /*  21: -2.9763460e-02 -2.9759066e-02 */
    { 0x40f20695,0x3f7542bf }, /*  22:  7.5633035e+00  9.5804971e-01 */
    { 0xbf4f4202,0xbf3958d9 }, /*  23: -8.0960095e-01 -7.2401196e-01 */
    { 0x3f69ce9e,0x3f4aa1bb }, /*  24:  9.1330898e-01  7.9153031e-01 */
    { 0xc0066b37,0xbf5cf190 }, /*  25: -2.1002939e+00 -8.6306095e-01 */
    { 0xbf6bbb40,0xbf4bcd53 }, /*  26: -9.2082596e-01 -7.9610175e-01 */
    { 0x3f1c2754,0x3f12a61b }, /*  27:  6.0997510e-01  5.7284707e-01 */
    { 0xbfcac146,0xbf7ffa44 }, /*  28: -1.5840232e+00 -9.9991250e-01 */
    { 0x3efa52a6,0x3ef077fe }, /*  29:  4.8891181e-01  4.6966547e-01 */
    { 0x3f24d1f2,0x3f19aac3 }, /*  30:  6.4382851e-01  6.0026187e-01 */
    { 0xbf6588a7,0xbf47fdfa }, /*  31: -8.9661640e-01 -7.8121912e-01 */
    { 0xc06be5a1,0x3f048fb0 }, /*  32: -3.6858904e+00  5.1781750e-01 */
    { 0xbee2e461,0xbedb89da }, /*  33: -4.4314864e-01 -4.2878610e-01 */
    { 0x3f6cc6b5,0x3f4c6ebe }, /*  34:  9.2490703e-01  7.9856479e-01 */
    { 0xbee45026,0xbedcd244 }, /*  35: -4.4592398e-01 -4.3129170e-01 */
    { 0xbf7f1174,0xbf56e964 }, /*  36: -9.9636006e-01 -8.3949876e-01 */
    { 0x3f031edf,0x3efaecb6 }, /*  37:  5.1218981e-01  4.9008721e-01 */
    { 0xbf80cb9d,0xbf58459a }, /*  38: -1.0062138e+00 -8.4481204e-01 */
    { 0xbec994bc,0xbec469d3 }, /*  39: -3.9371288e-01 -3.8361987e-01 */
    { 0xbe86e164,0xbe855363 }, /*  40: -2.6343834e-01 -2.6040182e-01 */
    { 0xbf93c6d3,0xbf6a22e5 }, /*  41: -1.1545051e+00 -9.1459495e-01 */
    { 0xbf4bdbca,0xbf36fc6b }, /*  42: -7.9632246e-01 -7.1478909e-01 */
    { 0xbf1106de,0xbf09649c }, /*  43: -5.6651103e-01 -5.3669143e-01 */
    { 0xbf340453,0xbf258b2f }, /*  44: -7.0319098e-01 -6.4665502e-01 */
    { 0xbf9d3eef,0xbf7125a6 }, /*  45: -1.2284831e+00 -9.4198072e-01 */
    { 0x3f810f7c,0x3f588e1d }, /*  46:  1.0082850e+00  8.4591848e-01 */
    { 0x3f3718a7,0x3f27e16d }, /*  47:  7.1521991e-01  6.5578347e-01 */
    { 0x4012c18d,0x3f40143f }, /*  48:  2.2930634e+00  7.5030893e-01 */
    { 0xbf1851c4,0xbf0f7d60 }, /*  49: -5.9499764e-01 -5.6050682e-01 */
    { 0x3f5de7d6,0x3f43245f }, /*  50:  8.6681879e-01  7.6227373e-01 */
    { 0x3f0b5064,0x3f0489f2 }, /*  51:  5.4419541e-01  5.1772988e-01 */
    { 0xbf4782d0,0xbf33eb5a }, /*  52: -7.7933979e-01 -7.0280993e-01 */
    { 0xbea86772,0xbea5624f }, /*  53: -3.2891423e-01 -3.2301566e-01 */
    { 0xbe4efa6d,0xbe4d925d }, /*  54: -2.0212717e-01 -2.0075364e-01 */
    { 0xbfceb26b,0xbf7fc081 }, /*  55: -1.6148199e+00 -9.9903113e-01 */
    { 0xbec20dd4,0xbebd70f9 }, /*  56: -3.7901175e-01 -3.7000254e-01 */
    { 0xbf6b602f,0xbf4b962a }, /*  57: -9.1943640e-01 -7.9526007e-01 */
    { 0x3ec9f8d4,0x3ec4c63e }, /*  58:  3.9447653e-01  3.8432497e-01 */
    { 0x3f052dc5,0x3efe8131 }, /*  59:  5.2022964e-01  4.9707940e-01 */
    { 0xbf044531,0xbefced2a }, /*  60: -5.1668078e-01 -4.9399692e-01 */
    { 0x3f069c71,0x3f007e3f }, /*  61:  5.2582461e-01  5.0192636e-01 */
    { 0xbeec6798,0xbee41803 }, /*  62: -4.6172786e-01 -4.4549569e-01 */
    { 0xbf06464d,0xbf0033b7 }, /*  63: -5.2451020e-01 -5.0078911e-01 */
    { 0x3ec42373,0x3ebf6052 }, /*  64:  3.8308296e-01  3.7378174e-01 */
    { 0xbfe53af6,0xbf79d37a }, /*  65: -1.7908618e+00 -9.7588313e-01 */
    { 0xbf0c741f,0xbf058333 }, /*  66: -5.4864687e-01 -5.2153319e-01 */
    { 0x3e9e891d,0x3e9c03b0 }, /*  67:  3.0963984e-01  3.0471563e-01 */
    { 0xbf0b2be9,0xbf046abb }, /*  68: -5.4363877e-01 -5.1725358e-01 */
    { 0xbd931696,0xbd92f639 }, /*  69: -7.1820423e-02 -7.1758695e-02 */
    { 0x3f0c9953,0x3f05a2ef }, /*  70:  5.4921454e-01  5.2201742e-01 */
    { 0x40ce290d,0x3e2275d1 }, /*  71:  6.4425111e+00  1.5865256e-01 */
    { 0x3fa2579c,0x3f74605d }, /*  72:  1.2682986e+00  9.5459539e-01 */
    { 0x3f8ae0b2,0x3f62612a }, /*  73:  1.0849822e+00  8.8429511e-01 */
    { 0xbeeab4fa,0xbee29296 }, /*  74: -4.5841199e-01 -4.4252461e-01 */
    { 0x3f8b6399,0x3f62daf2 }, /*  75:  1.0889770e+00  8.8615334e-01 */
    { 0x4086869a,0xbf5f9d72 }, /*  76:  4.2039309e+00 -8.7349617e-01 */
    { 0x3f4d8202,0x3f3822b8 }, /*  77:  8.0276501e-01  7.1927977e-01 */
    { 0xbf94b83a,0xbf6ae480 }, /*  78: -1.1618721e+00 -9.1754913e-01 */
    { 0xbf07886e,0xbf014a25 }, /*  79: -5.2942550e-01 -5.0503761e-01 */
    { 0x3f4d9d9e,0x3f3835e5 }, /*  80:  8.0318630e-01  7.1957237e-01 */
    { 0x3f1e8c0d,0x3f149aaf }, /*  81:  6.1932451e-01  5.8048528e-01 */
    { 0xbf6a2b11,0xbf4ada2d }, /*  82: -9.1471964e-01 -7.9239160e-01 */
    { 0xbf8ba39b,0xbf631628 }, /*  83: -1.0909303e+00 -8.8705683e-01 */
    { 0x40b845ec,0xbf003ba4 }, /*  84:  5.7585354e+00 -5.0091004e-01 */
    { 0xbf62ee29,0xbf465b47 }, /*  85: -8.8644654e-01 -7.7483028e-01 */
    { 0x3fc6cb16,0x3f7ff5b5 }, /*  86:  1.5530727e+00  9.9984294e-01 */
    { 0xbef7bb4e,0xbeee2d94 }, /*  87: -4.8385090e-01 -4.6519148e-01 */
    { 0xbf4c9716,0xbf377f36 }, /*  88: -7.9918039e-01 -7.1678483e-01 */
    { 0xbf323c1a,0xbf242e29 }, /*  89: -6.9622958e-01 -6.4132935e-01 */
    { 0xbedca08a,0xbed5dcc3 }, /*  90: -4.3091232e-01 -4.1769990e-01 */
    { 0x4030cf97,0x3ebd6617 }, /*  91:  2.7626703e+00  3.6991951e-01 */
    { 0x3ed0212d,0x3eca71d9 }, /*  92:  4.0650311e-01  3.9539984e-01 */
    { 0xc00c8e09,0xbf4f8cfb }, /*  93: -2.1961691e+00 -8.1074494e-01 */
    { 0xbf3c824a,0xbf2bedd3 }, /*  94: -7.3636305e-01 -6.7159766e-01 */
    { 0xbee6c513,0xbedf0909 }, /*  95: -4.5072231e-01 -4.3561581e-01 */
    { 0x3f84c0e8,0x3f5c6750 }, /*  96:  1.0371370e+00  8.6095142e-01 */
    { 0x3f1a6e1a,0x3f113b99 }, /*  97:  6.0324252e-01  5.6731564e-01 */
    { 0xbe078626,0xbe0720f4 }, /*  98: -1.3234767e-01 -1.3196164e-01 */
    { 0x3fd62e07,0x3f7ea825 }, /*  99:  1.6732796e+00  9.9475318e-01 */
    { 0x3f17c530,0x3f0f08df }, /* 100:  5.9285259e-01  5.5872911e-01 */
    { 0xbf4e3686,0xbf389ff2 }, /* 101: -8.0551946e-01 -7.2119057e-01 */
    { 0xbf7df4d4,0xbf564e38 }, /* 102: -9.9201703e-01 -8.3713102e-01 */
    { 0xbf31b8da,0xbf23c95f }, /* 103: -6.9422686e-01 -6.3979143e-01 */
    { 0xbef8ca8b,0xbeef1d8c }, /* 104: -4.8592028e-01 -4.6702230e-01 */
    { 0x3e8c4e3a,0x3e8a8e5e }, /* 105:  2.7403432e-01  2.7061743e-01 */
    { 0xbee54840,0xbeddb201 }, /* 106: -4.4781685e-01 -4.3299869e-01 */
    { 0x3f3600cb,0x3f270dc1 }, /* 107:  7.1094960e-01  6.5255362e-01 */
    { 0x3f507c12,0x3f3a30f0 }, /* 108:  8.1439316e-01  7.2730923e-01 */
    { 0xbf47331f,0xbf33b2a0 }, /* 109: -7.7812380e-01 -7.0194435e-01 */
    { 0x3f31c555,0x3f23d2f6 }, /* 110:  6.9441730e-01  6.3993776e-01 */
    { 0x3f11e3ab,0x3f0a1eb7 }, /* 111:  5.6988019e-01  5.3953117e-01 */
    { 0xbf497de2,0xbf3552b0 }, /* 112: -7.8707707e-01 -7.0829296e-01 */
    { 0xbf6a8cd4,0xbf4b15c1 }, /* 113: -9.1621137e-01 -7.9330069e-01 */
    { 0xbf4beead,0xbf3709a0 }, /* 114: -7.9661065e-01 -7.1499062e-01 */
    { 0x3ec3eac4,0x3ebf2bbd }, /* 115:  3.8265049e-01  3.7338057e-01 */
    { 0x3f4d7a8b,0x3f381d88 }, /* 116:  8.0265111e-01  7.1920061e-01 */
    { 0xbf2ff313,0xbf226b9f }, /* 117: -6.8730277e-01 -6.3445467e-01 */
    { 0xbe8c5e9a,0xbe8a9e21 }, /* 118: -2.7415925e-01 -2.7073768e-01 */
    { 0xc03ec959,0xbe23b384 }, /* 119: -2.9810393e+00 -1.5986449e-01 */
    { 0x3a5fecd8,0x3a5fecd6 }, /* 120:  8.5420674e-04  8.5420662e-04 */
    { 0x3f667883,0x3f48935e }, /* 121:  9.0027636e-01  7.8349864e-01 */
    { 0x3f7432c2,0x3f50d03f }, /* 122:  9.5389950e-01  8.1567758e-01 */
    { 0xbf82dada,0xbf5a729f }, /* 123: -1.0223038e+00 -8.5331148e-01 */
    { 0xbf140f38,0xbf0bf132 }, /* 124: -5.7835722e-01 -5.4664910e-01 */
    { 0x3f03a383,0x3efbd3d2 }, /* 125:  5.1421374e-01  4.9185044e-01 */
    { 0xbf36bff2,0xbf279e6a }, /* 126: -7.1386635e-01 -6.5476096e-01 */
    { 0xbd5dbecb,0xbd5da311 }, /* 127: -5.4137032e-02 -5.4110590e-02 */
    { 0x3e9c9173,0x3e9a23ae }, /* 128:  3.0579719e-01  3.0105346e-01 */
    { 0x3f4bcab5,0x3f36f078 }, /* 129:  7.9606181e-01  7.1460676e-01 */
    { 0xbef48b75,0xbeeb5a34 }, /* 130: -4.7762647e-01 -4.5967257e-01 */
    { 0x3f4939f9,0x3f3522b9 }, /* 131:  7.8604084e-01  7.0756108e-01 */
    { 0xbf6b17c2,0xbf4b6a39 }, /* 132: -9.1833127e-01 -7.9458958e-01 */
    { 0xbf431e8e,0xbf30c4d7 }, /* 133: -7.6218498e-01 -6.9050354e-01 */
    { 0xbf377dd0,0xbf282dbf }, /* 134: -7.1676350e-01 -6.5694803e-01 */
    { 0xbd40a9b4,0xbd409785 }, /* 135: -4.7036842e-02 -4.7019500e-02 */
    { 0xbefca85c,0xbef28709 }, /* 136: -4.9347198e-01 -4.7368649e-01 */
    { 0x3f31b48b,0x3f23c60f }, /* 137:  6.9416112e-01  6.3974088e-01 */
    { 0xbf1306e7,0xbf0b1393 }, /* 138: -5.7432407e-01 -5.4326743e-01 */
    { 0x3f32188f,0x3f2412e2 }, /* 139:  6.9568723e-01  6.4091313e-01 */
    { 0x3e04af2a,0x3e045032 }, /* 140:  1.2957445e-01  1.2921217e-01 */
    { 0x3f3590e7,0x3f26b8e8 }, /* 141:  7.0924228e-01  6.5125895e-01 */
    { 0x3f32bc2b,0x3f249057 }, /* 142:  6.9818372e-01  6.4282745e-01 */
    { 0x3f0e1bab,0x3f06ebdd }, /* 143:  5.5510968e-01  5.2703649e-01 */
    { 0x3f32c27a,0x3f24952c }, /* 144:  6.9827998e-01  6.4290118e-01 */
    { 0xbf48b241,0xbf34c2b8 }, /* 145: -7.8396994e-01 -7.0609617e-01 */
    { 0x3f324464,0x3f243485 }, /* 146:  6.9635606e-01  6.4142638e-01 */
    { 0x3f00aee3,0x3ef6a9ff }, /* 147:  5.0266856e-01  4.8176572e-01 */
    { 0x3f331953,0x3f24d7a8 }, /* 148:  6.9960517e-01  6.4391565e-01 */
    { 0xbb5a94c2,0xbb5a94a7 }, /* 149: -3.3352827e-03 -3.3352764e-03 */
    { 0x405b87ef,0xbe91b5d3 }, /* 150:  3.4301717e+00 -2.8459033e-01 */
    { 0xbf4c7e0b,0xbf376dbf }, /* 151: -7.9879826e-01 -7.1651834e-01 */
    { 0xbede736f,0xbed784a0 }, /* 152: -4.3447444e-01 -4.2093372e-01 */
    { 0x3f37cec1,0x3f286abc }, /* 153:  7.1799856e-01  6.5787864e-01 */
    { 0xbf2159d8,0xbf16e0de }, /* 154: -6.3027716e-01 -5.8936870e-01 */
    { 0x3f671996,0x3f48f74e }, /* 155:  9.0273416e-01  7.8502357e-01 */
    { 0x3f4979f8,0x3f354fec }, /* 156:  7.8701735e-01  7.0825076e-01 */
    { 0xbeb96a8d,0xbeb563d3 }, /* 157: -3.6214104e-01 -3.5427722e-01 */
    { 0x3ef8856d,0x3eeee06c }, /* 158:  4.8539296e-01  4.6655595e-01 */
    { 0x3f6fc784,0x3f4e39e2 }, /* 159:  9.3663812e-01  8.0557072e-01 */
    { 0xc10e923b,0xbefbc43b }, /* 160: -8.9107008e+00 -4.9173149e-01 */
    { 0x3fa1c817,0x3f740a41 }, /* 161:  1.2639188e+00  9.5328146e-01 */
    { 0x3f517ad8,0x3f3adf6f }, /* 162:  8.1828070e-01  7.2997183e-01 */
    { 0xbf3577c1,0xbf26a5d2 }, /* 163: -7.0885855e-01 -6.5096772e-01 */
    { 0x3f0f2aea,0x3f07d216 }, /* 164:  5.5924857e-01  5.3054941e-01 */
    { 0x3f53a51c,0x3f3c588b }, /* 165:  8.2673812e-01  7.3572606e-01 */
    { 0xbf6a450f,0xbf4aea07 }, /* 166: -9.1511625e-01 -7.9263347e-01 */
    { 0xbf7fc69f,0xbf574b9f }, /* 167: -9.9912447e-01 -8.4099764e-01 */
    { 0x3d40a1e5,0x3d408fb8 }, /* 168:  4.7029395e-02  4.7012061e-02 */
    { 0xbf86ca18,0xbf5e7265 }, /* 169: -1.0530424e+00 -8.6893302e-01 */
    { 0x3f1696d9,0x3f0e0dbc }, /* 170:  5.8823925e-01  5.5489707e-01 */
    { 0xbf3349a1,0xbf24fc9a }, /* 171: -7.0034224e-01 -6.4447939e-01 */
    { 0xbf7ed5ec,0xbf56c905 }, /* 172: -9.9545169e-01 -8.3900481e-01 */
    { 0x3f6d4c9a,0x3f4cbf3a }, /* 173:  9.2695010e-01  7.9979289e-01 */
    { 0xbf12ea4f,0xbf0afb91 }, /* 174: -5.7388777e-01 -5.4290110e-01 */
    { 0x3e6e479b,0x3e6c229a }, /* 175:  2.3269503e-01  2.3060074e-01 */
    { 0x3f4c1513,0x3f372476 }, /* 176:  7.9719657e-01  7.1540010e-01 */
    { 0xbef46873,0xbeeb3b1c }, /* 177: -4.7735938e-01 -4.5943534e-01 */
    { 0x4018b64a,0x3f2f8558 }, /* 178:  2.3861260e+00  6.8562841e-01 */
    { 0xbf5eb195,0xbf43a6b7 }, /* 179: -8.6989719e-01 -7.6426262e-01 */
    { 0x3f359159,0x3f26b93f }, /* 180:  7.0924908e-01  6.5126413e-01 */
    { 0xbfb0c265,0xbf7b6650 }, /* 181: -1.3809325e+00 -9.8202991e-01 */
    { 0x3f2f20a4,0x3f21c8c0 }, /* 182:  6.8409181e-01  6.3196945e-01 */
    { 0xbf61e083,0xbf45b062 }, /* 183: -8.8233203e-01 -7.7222264e-01 */
    { 0xbf36acc1,0xbf278fe9 }, /* 184: -7.1357352e-01 -6.5453964e-01 */
    { 0xbf981f90,0xbf6d8416 }, /* 185: -1.1884632e+00 -9.2779672e-01 */
    { 0x3ed866c1,0x3ed20403 }, /* 186:  4.2265895e-01  4.1018686e-01 */
    { 0xbf1b12f2,0xbf11c33b }, /* 187: -6.0575783e-01 -5.6938523e-01 */
    { 0xbf46e5a6,0xbf337b6a }, /* 188: -7.7694166e-01 -7.0110190e-01 */
    { 0x3f36c900,0x3f27a542 }, /* 189:  7.1400452e-01  6.5486538e-01 */
    { 0xbf779069,0xbf52be15 }, /* 190: -9.6704727e-01 -8.2321292e-01 */
    { 0x3f338dc1,0x3f2530ab }, /* 191:  7.0138174e-01  6.4527386e-01 */
    { 0x3f150c7e,0x3f0cc502 }, /* 192:  5.8222187e-01  5.4988110e-01 */
    { 0xbf7267ee,0xbf4fc581 }, /* 193: -9.4689834e-01 -8.1160742e-01 */
    { 0x3ee8e87e,0x3ee0f549 }, /* 194:  4.5489877e-01  4.3937138e-01 */
    { 0x37586f2e,0x37586f2e }, /* 195:  1.2900489e-05  1.2900489e-05 */
    { 0x3f625d15,0x3f45ff70 }, /* 196:  8.8423282e-01  7.7342892e-01 */
    { 0xbf7df422,0xbf564dd6 }, /* 197: -9.9200642e-01 -8.3712518e-01 */
    { 0x3f1afe94,0x3f11b27c }, /* 198:  6.0544705e-01  5.6912971e-01 */
    { 0x3eff062f,0x3ef49bea }, /* 199:  4.9809405e-01  4.7775203e-01 */
    { 0x3fc3eb3e,0x3f7fcb1b }, /* 200:  1.5306165e+00  9.9919289e-01 */
    { 0x3f315364,0x3f237b58 }, /* 201:  6.9267869e-01  6.3860083e-01 */
    { 0xbf6596f9,0xbf4806eb }, /* 202: -8.9683491e-01 -7.8135556e-01 */
    { 0xbf349a96,0xbf25fdb0 }, /* 203: -7.0548379e-01 -6.4840221e-01 */
    { 0xbfe36fdd,0xbf7a95a1 }, /* 204: -1.7768513e+00 -9.7884566e-01 */
    { 0xbf98c5d0,0xbf6dff5a }, /* 205: -1.1935368e+00 -9.2967761e-01 */
    { 0x3f36f3c8,0x3f27c595 }, /* 206:  7.1465731e-01  6.5535861e-01 */
    { 0x3f89d5bd,0x3f6165f1 }, /* 207:  1.0768353e+00  8.8046175e-01 */
    { 0x3f5c4e22,0x3f421a37 }, /* 208:  8.6056721e-01  7.5821251e-01 */
    { 0xbf0c32eb,0xbf054b8c }, /* 209: -5.4765195e-01 -5.2068400e-01 */
    { 0x3e6c61d6,0x3e6a49d3 }, /* 210:  2.3084196e-01  2.2879724e-01 */
    { 0xbedb1e8c,0xbed47dd1 }, /* 211: -4.2796743e-01 -4.1502240e-01 */
    { 0x3efc8d05,0x3ef26ef5 }, /* 212:  4.9326339e-01  4.7350278e-01 */
    { 0x3eb6d176,0x3eb2f547 }, /* 213:  3.5706681e-01  3.4952757e-01 */
    { 0xbe579cfe,0xbe560608 }, /* 214: -2.1055982e-01 -2.0900738e-01 */
    { 0x3ebffac5,0x3ebb8335 }, /* 215:  3.7496009e-01  3.6623541e-01 */
    { 0x400a286c,0x3f550438 }, /* 216:  2.1587172e+00  8.3209562e-01 */
    { 0x3f323c93,0x3f242e86 }, /* 217:  6.9623679e-01  6.4133489e-01 */
    { 0x3ee440cf,0x3edcc46d }, /* 218:  4.4580695e-01  4.3118611e-01 */
    { 0x3fc507d3,0x3f7fdf80 }, /* 219:  1.5393013e+00  9.9950409e-01 */
    { 0xbf8573b9,0xbf5d1c65 }, /* 220: -1.0425941e+00 -8.6371452e-01 */
    { 0x3f31fc98,0x3f23fd6a }, /* 221:  6.9526052e-01  6.4058554e-01 */
    { 0xbf32cf93,0xbf249f34 }, /* 222: -6.9847983e-01 -6.4305425e-01 */
    { 0x3f1baeab,0x3f124324 }, /* 223:  6.0813397e-01  5.7133698e-01 */
    { 0x3f6565bd,0x3f47e82d }, /* 224:  8.9608365e-01  7.8088647e-01 */
    { 0x3f837822,0x3f5b15fe }, /* 225:  1.0271037e+00  8.5580432e-01 */
    { 0x3e6b4907,0x3e69386e }, /* 226:  2.2977076e-01  2.2775432e-01 */
    { 0xbf1c3965,0xbf12b4e9 }, /* 227: -6.1025077e-01 -5.7307297e-01 */
    { 0x3f4cb033,0x3f3790b8 }, /* 228:  7.9956359e-01  7.1705198e-01 */
    { 0xbec18d34,0xbebcf974 }, /* 229: -3.7803042e-01 -3.6909068e-01 */
    { 0x3c557eff,0x3c557d73 }, /* 230:  1.3030767e-02  1.3030398e-02 */
    { 0x20cdfc7e,0x20cdfc7e }, /* 231:  3.4895436e-19  3.4895436e-19 */
    { 0x40db9e6e,0x3f0c460c }, /* 232:  6.8630896e+00  5.4794383e-01 */
    { 0x3f4bb892,0x3f36e3c8 }, /* 233:  7.9578507e-01  7.1441317e-01 */
    { 0xbf19ddd1,0xbf10c4b1 }, /* 234: -6.0104090e-01 -5.6550127e-01 */
    { 0x3f4ded46,0x3f386d2d }, /* 235:  8.0440176e-01  7.2041589e-01 */
    { 0xbf0a8f52,0xbf03e49f }, /* 236: -5.4124939e-01 -5.1520723e-01 */
    { 0x3f799c06,0x3f53e5a2 }, /* 237:  9.7503698e-01  8.2772267e-01 */
    { 0xbd453aed,0xbd45276b }, /* 238: -4.8151899e-02 -4.8133295e-02 */
    { 0xbf8b6a0c,0xbf62e0eb }, /* 239: -1.0891738e+00 -8.8624448e-01 */
    { 0x3ff68adf,0x3f700262 }, /* 240:  1.9261130e+00  9.3753636e-01 */
    { 0xac5804b8,0xac5804b8 }, /* 241: -3.0698066e-12 -3.0698066e-12 */
    { 0x3f5e7476,0x3f437f47 }, /* 242:  8.6896455e-01  7.6366085e-01 */
    { 0xbf683995,0xbf49a936 }, /* 243: -9.0712863e-01 -7.8773820e-01 */
    { 0xbfe42c50,0xbf7a4775 }, /* 244: -1.7826023e+00 -9.7765285e-01 */
    { 0x3eea8fa0,0x3ee27117 }, /* 245:  4.5812702e-01  4.4226906e-01 */
    { 0xbf0e831c,0xbf0743bb }, /* 246: -5.5668807e-01 -5.2837723e-01 */
    { 0xbf45c245,0xbf32ab31 }, /* 247: -7.7249557e-01 -6.9792467e-01 */
    { 0xbf659e16,0xbf480b5b }, /* 248: -8.9694345e-01 -7.8142327e-01 */
    { 0x3f079fd2,0x3f015e54 }, /* 249:  5.2978241e-01  5.0534558e-01 */
    { 0x3fa7d6fa,0x3f776cf0 }, /* 250:  1.3112481e+00  9.6650600e-01 */
    { 0x3f20d05b,0x3f1671b6 }, /* 251:  6.2817925e-01  5.8767259e-01 */
    { 0x3f10d487,0x3f093a1f }, /* 252:  5.6574291e-01  5.3604311e-01 */
    { 0x3e38e8dd,0x3e37e806 }, /* 253:  1.8057580e-01  1.7959604e-01 */
    { 0x3f3282ec,0x3f246479 }, /* 254:  6.9731021e-01  6.4215809e-01 */
    { 0x3fae9231,0x3f7a8978 }, /* 255:  1.3638364e+00  9.7866011e-01 */
    { 0x3f263b1c,0x3f1acb06 }, /* 256:  6.4933944e-01  6.0466039e-01 */
    { 0xbf66e5ed,0xbf48d749 }, /* 257: -9.0194589e-01 -7.8453499e-01 */
    { 0x3f7f4f87,0x3f570b18 }, /* 258:  9.9730724e-01  8.4001303e-01 */
    { 0x3f9701b3,0x3f6cac79 }, /* 259:  1.1797394e+00  9.2450672e-01 */
    { 0xbf477e68,0xbf33e837 }, /* 260: -7.7927256e-01 -7.0276207e-01 */
    { 0x3fbc02e2,0x3f7eaba8 }, /* 261:  1.4688380e+00  9.9480677e-01 */
    { 0x3f203521,0x3f15f402 }, /* 262:  6.2581068e-01  5.8575451e-01 */
    { 0xbf32ba4c,0xbf248ee8 }, /* 263: -6.9815516e-01 -6.4280558e-01 */
    { 0xbf86127e,0xbf5dbbc3 }, /* 264: -1.0474393e+00 -8.6614627e-01 */
    { 0x3f64152a,0x3f471541 }, /* 265:  8.9094794e-01  7.7766806e-01 */
    { 0x3f1d2207,0x3f137351 }, /* 266:  6.1380047e-01  5.7597834e-01 */
    { 0x406ed7e6,0xbf0e7fe7 }, /* 267:  3.7319274e+00 -5.5663913e-01 */
    { 0x3f18d148,0x3f0fe6e8 }, /* 268:  5.9694338e-01  5.6211710e-01 */
    { 0x3f9293a8,0x3f6927d9 }, /* 269:  1.1451311e+00  9.1076428e-01 */
    { 0xbf8461ab,0xbf5c062e }, /* 270: -1.0342306e+00 -8.5946929e-01 */
    { 0x3ff159b3,0x3f736c65 }, /* 271:  1.8855499e+00  9.5087272e-01 */
    { 0x3f64a9c5,0x3f47728c }, /* 272:  8.9321548e-01  7.7909160e-01 */
    { 0xbf189b32,0xbf0fba29 }, /* 273: -5.9611809e-01 -5.6143433e-01 */
    { 0x3f3730be,0x3f27f39c }, /* 274:  7.1558750e-01  6.5606093e-01 */
    { 0x3f928e35,0x3f692358 }, /* 275:  1.1449648e+00  9.1069555e-01 */
    { 0x3f4e85f8,0x3f38d6f2 }, /* 276:  8.0673170e-01  7.2202981e-01 */
    { 0x3f5cc67e,0x3f42689b }, /* 277:  8.6240375e-01  7.5940865e-01 */
    { 0xbf059a94,0xbeff3dee }, /* 278: -5.2188993e-01 -4.9851936e-01 */
    { 0xbf6c7613,0xbf4c3e2c }, /* 279: -9.2367667e-01 -7.9782367e-01 */
    { 0xbf459d90,0xbf3290e5 }, /* 280: -7.7193546e-01 -6.9752342e-01 */
    { 0xbf3a9633,0xbf2a7ffe }, /* 281: -7.2885436e-01 -6.6601551e-01 */
    { 0xc09d7093,0x3f7a80cc }, /* 282: -4.9199920e+00  9.7852778e-01 */
    { 0x3fb43dd5,0x3f7c9eef }, /* 283:  1.4081370e+00  9.8680013e-01 */
    { 0xbee68fe7,0xbeded92c }, /* 284: -4.5031664e-01 -4.3525064e-01 */
    { 0xbf4a0358,0xbf35b0ce }, /* 285: -7.8911352e-01 -7.0972908e-01 */
    { 0xbd534026,0xbd53282d }, /* 286: -5.1574849e-02 -5.1551986e-02 */
    { 0x3f4a3ebd,0x3f35daa0 }, /* 287:  7.9001981e-01  7.1036720e-01 */
    { 0xbfd3ab0c,0xbf7f1f25 }, /* 288: -1.6536574e+00 -9.9656898e-01 */
    { 0xbe9f3846,0xbe9caa7b }, /* 289: -3.1097621e-01 -3.0598816e-01 */
    { 0xc0652b52,0x3ed9b30d }, /* 290: -3.5807691e+00  4.2519417e-01 */
    { 0x3f7e0827,0x3f5658c9 }, /* 291:  9.9231189e-01  8.3729225e-01 */
    { 0xbfb69313,0xbf7d559e }, /* 292: -1.4263633e+00 -9.8958766e-01 */
    { 0x3f14cd0e,0x3f0c9001 }, /* 293:  5.8125389e-01  5.4907233e-01 */
    { 0x3fa8f534,0x3f77fd70 }, /* 294:  1.3199830e+00  9.6871090e-01 */
    { 0x3f6bff99,0x3f4bf6a8 }, /* 295:  9.2186886e-01  7.9673243e-01 */
    { 0xbf1de5cf,0xbf141332 }, /* 296: -6.1678785e-01 -5.7841790e-01 */
    { 0xbf6bcbf4,0xbf4bd76e }, /* 297: -9.2108083e-01 -7.9625595e-01 */
    { 0x3f4d33a3,0x3f37ec3d }, /* 298:  8.0156916e-01  7.1844846e-01 */
    { 0xbf533b7f,0xbf3c10f4 }, /* 299: -8.2512659e-01 -7.3463368e-01 */
    { 0xbf113c8a,0xbf0991e2 }, /* 300: -5.6733000e-01 -5.3738225e-01 */
    { 0xbf10c7aa,0xbf092f43 }, /* 301: -5.6554663e-01 -5.3587741e-01 */
    { 0xbefe2dab,0xbef3dd9f }, /* 302: -4.9644217e-01 -4.7630021e-01 */
    { 0x3f4cd973,0x3f37ad76 }, /* 303:  8.0019301e-01  7.1749055e-01 */
    { 0x3f357014,0x3f269ffe }, /* 304:  7.0874143e-01  6.5087879e-01 */
    { 0x3f7cbdb4,0x3f55a36a }, /* 305:  9.8726964e-01  8.3452475e-01 */
    { 0x3efcfc75,0x3ef2d116 }, /* 306:  4.9411359e-01  4.7425145e-01 */
    { 0x3f22d32f,0x3f181110 }, /* 307:  6.3603491e-01  5.9401035e-01 */
    { 0x37f4be73,0x37f4be73 }, /* 308:  2.9175751e-05  2.9175751e-05 */
    { 0x3f362347,0x3f2727e1 }, /* 309:  7.1147579e-01  6.5295225e-01 */
    { 0xbf6e4c10,0xbf4d582d }, /* 310: -9.3084812e-01 -8.0212671e-01 */
    { 0x3f476849,0x3f33d87a }, /* 311:  7.7893502e-01  7.0252192e-01 */
    { 0x3efceb3b,0x3ef2c1ec }, /* 312:  4.9398217e-01  4.7413576e-01 */
    { 0x3f50fea4,0x3f3a8a75 }, /* 313:  8.1638551e-01  7.2867519e-01 */
    { 0xbf0cd74e,0xbf05d7c9 }, /* 314: -5.5016029e-01 -5.2282387e-01 */
    { 0xbf0fa58c,0xbf0839f9 }, /* 315: -5.6111979e-01 -5.3213459e-01 */
    { 0xbfe2604b,0xbf7b028e }, /* 316: -1.7685636e+00 -9.8050773e-01 */
    { 0xbf244939,0xbf193d54 }, /* 317: -6.4174229e-01 -5.9859204e-01 */
    { 0x3f108475,0x3f08f680 }, /* 318:  5.6452113e-01  5.3501129e-01 */
    { 0xbefb90e2,0xbef190c3 }, /* 319: -4.9133974e-01 -4.7180757e-01 */
    { 0x3ef0cf69,0x3ee8078e }, /* 320:  4.7033241e-01  4.5318264e-01 */
    { 0xbf4b94b7,0xbf36caaf }, /* 321: -7.9523796e-01 -7.1403021e-01 */
    { 0x3f645e3c,0x3f474329 }, /* 322:  8.9206290e-01  7.7836853e-01 */
    { 0x3f173f8d,0x3f0e99f6 }, /* 323:  5.9081346e-01  5.5703676e-01 */
    { 0xbf28b08e,0xbf1cbe88 }, /* 324: -6.5894401e-01 -6.1228228e-01 */
    { 0xbf10e1fe,0xbf09457d }, /* 325: -5.6594837e-01 -5.3621656e-01 */
    { 0xbf450551,0xbf3223ae }, /* 326: -7.6961237e-01 -6.9585693e-01 */
    { 0xbec5b0c4,0xbec0d09d }, /* 327: -3.8611424e-01 -3.7659159e-01 */
    { 0x3f07baf2,0x3f0175bc }, /* 328:  5.3019631e-01  5.0570273e-01 */
    { 0x3ff50746,0x3f710bc2 }, /* 329:  1.9142845e+00  9.4158566e-01 */
    { 0xbee52f87,0xbedd9bb8 }, /* 330: -4.4762823e-01 -4.3282866e-01 */
    { 0x3f01837d,0x3ef81e45 }, /* 331:  5.0591260e-01  4.8460594e-01 */
    { 0xbf34aba9,0xbf260aaf }, /* 332: -7.0574433e-01 -6.4860052e-01 */
    { 0xbf11626c,0xbf09b1d3 }, /* 333: -5.6790805e-01 -5.3786963e-01 */
    { 0x3f4d3cd9,0x3f37f2a5 }, /* 334:  8.0170971e-01  7.1854621e-01 */
    { 0x3f9125cb,0x3f67f5fb }, /* 335:  1.1339658e+00  9.0609711e-01 */
    { 0xbf4c0e93,0xbf371feb }, /* 336: -7.9709738e-01 -7.1533078e-01 */
    { 0xbeebd2b3,0xbee392ac }, /* 337: -4.6059188e-01 -4.4447839e-01 */
    { 0x3f8008e7,0x3f577443 }, /* 338:  1.0002717e+00  8.4161776e-01 */
    { 0xbea6c8dc,0xbea3d9bc }, /* 339: -3.2575119e-01 -3.2002056e-01 */
    { 0x3f4d4531,0x3f37f872 }, /* 340:  8.0183703e-01  7.1863472e-01 */
    { 0xbf15e0b0,0xbf0d760d }, /* 341: -5.8545971e-01 -5.5258256e-01 */
    { 0xbf7a822c,0xbf546671 }, /* 342: -9.7854877e-01 -8.2968813e-01 */
    { 0x3f849203,0x3f5c378b }, /* 343:  1.0357059e+00  8.6022252e-01 */
    { 0x3f4a6942,0x3f35f88b }, /* 344:  7.9066861e-01  7.1082371e-01 */
    { 0x3f34d3ac,0x3f262922 }, /* 345:  7.0635486e-01  6.4906514e-01 */
    { 0x3f332a89,0x3f24e4d3 }, /* 346:  6.9986778e-01  6.4411658e-01 */
    { 0x3f4b71f0,0x3f36b254 }, /* 347:  7.9470730e-01  7.1365857e-01 */
    { 0xbf6dd621,0xbf4d11aa }, /* 348: -9.2904860e-01 -8.0105078e-01 */
    { 0x3efc6f23,0x3ef254a2 }, /* 349:  4.9303541e-01  4.7330195e-01 */
    { 0x3f325656,0x3f244249 }, /* 350:  6.9662988e-01  6.4163643e-01 */
    { 0xbf4be887,0xbf370553 }, /* 351: -7.9651684e-01 -7.1492499e-01 */
    { 0xbee0322b,0xbed91988 }, /* 352: -4.3788275e-01 -4.2402291e-01 */
    { 0xbf73ca19,0xbf5093a2 }, /* 353: -9.5230252e-01 -8.1475270e-01 */
    { 0x3f08fd53,0x3f028b73 }, /* 354:  5.3511542e-01  5.0994033e-01 */
    { 0xbf4d7661,0xbf381aa3 }, /* 355: -8.0258757e-01 -7.1915644e-01 */
    { 0x3f75b49a,0x3f51ae88 }, /* 356:  9.5978701e-01  8.1906939e-01 */
    { 0x3edf0b40,0x3ed80e4d }, /* 357:  4.3563271e-01  4.2198411e-01 */
    { 0xbf78ec44,0xbf5382d0 }, /* 358: -9.7235513e-01 -8.2621479e-01 */
    { 0xbed04163,0xbeca8f6e }, /* 359: -4.0674886e-01 -3.9562553e-01 */
    { 0x3f327ad0,0x3f245e42 }, /* 360:  6.9718647e-01  6.4206326e-01 */
    { 0x3ef8a234,0x3eeef9e0 }, /* 361:  4.8561251e-01  4.6675014e-01 */
    { 0x3f66ed09,0x3f48dbb2 }, /* 362:  9.0205437e-01  7.8460228e-01 */
    { 0xbedb11c5,0xbed47231 }, /* 363: -4.2786995e-01 -4.1493371e-01 */
    { 0xbf4d9768,0xbf383195 }, /* 364: -8.0309153e-01 -7.1950656e-01 */
    { 0xbf37256d,0xbf27eb11 }, /* 365: -7.1541482e-01 -6.5593058e-01 */
    { 0xbf1e00a8,0xbf142917 }, /* 366: -6.1719751e-01 -5.7875198e-01 */
    { 0x3ef4e249,0x3eeba74d }, /* 367:  4.7828892e-01  4.6026078e-01 */
    { 0xbf738a1b,0xbf506e81 }, /* 368: -9.5132607e-01 -8.1418616e-01 */
    { 0xbf0be6e3,0xbf050a9d }, /* 369: -5.4649180e-01 -5.1969320e-01 */
    { 0x3f4f38c1,0x3f395277 }, /* 370:  8.0945975e-01  7.2391456e-01 */
    { 0x3f3253a7,0x3f24403a }, /* 371:  6.9658893e-01  6.4160502e-01 */
    { 0x3eefb796,0x3ee70dfa }, /* 372:  4.6819752e-01  4.5127851e-01 */
    { 0x3f207f4c,0x3f163018 }, /* 373:  6.2694240e-01  5.8667135e-01 */
    { 0x3efcd8d0,0x3ef2b1b4 }, /* 374:  4.9384165e-01  4.7401202e-01 */
    { 0xbf6d2015,0xbf4ca47d }, /* 375: -9.2627078e-01 -7.9938489e-01 */
    { 0xbdc0fed6,0xbdc0b5be }, /* 376: -9.4236061e-02 -9.4096646e-02 */
    { 0x3efbff34,0x3ef1f202 }, /* 377:  4.9218142e-01  4.7254950e-01 */
    { 0x3f3ba3d3,0x3f2b48bf }, /* 378:  7.3296851e-01  6.6907877e-01 */
    { 0x3f1dfcdb,0x3f1425fe }, /* 379:  6.1713952e-01  5.7870471e-01 */
    { 0x401940a0,0x3f2df0f0 }, /* 380:  2.3945694e+00  6.7945766e-01 */
    { 0xbf753533,0xbf516557 }, /* 381: -9.5784301e-01 -8.1795257e-01 */
    { 0xbfeab28d,0xbf77364b }, /* 382: -1.8335739e+00 -9.6567219e-01 */
    { 0xbea70efd,0xbea41c2b }, /* 383: -3.2628623e-01 -3.2052740e-01 */
    { 0x3ef87c55,0x3eeed861 }, /* 384:  4.8532358e-01  4.6649459e-01 */
    { 0xbf6d9a37,0xbf4cedc7 }, /* 385: -9.2813438e-01 -8.0050319e-01 */
    { 0xbedde6dc,0xbed70515 }, /* 386: -4.3340194e-01 -4.1996065e-01 */
    { 0x3f35cd30,0x3f26e6a3 }, /* 387:  7.1016216e-01  6.5195674e-01 */
    { 0xbf079c8a,0xbf015b7f }, /* 388: -5.2973235e-01 -5.0530237e-01 */
    { 0xbfa58e98,0xbf7636ed }, /* 389: -1.2934141e+00 -9.6177560e-01 */
    { 0xbefc8c81,0xbef26e80 }, /* 390: -4.9325946e-01 -4.7349930e-01 */
    { 0x3f329dce,0x3f247914 }, /* 391:  6.9772041e-01  6.4247251e-01 */
    { 0xbf0c21ea,0xbf053d08 }, /* 392: -5.4739249e-01 -5.2046251e-01 */
    { 0xbf37b2ad,0xbf285596 }, /* 393: -7.1757013e-01 -6.5755594e-01 */
    { 0x3f4ceccc,0x3f37baf0 }, /* 394:  8.0048823e-01  7.1769619e-01 */
    { 0x3f717e88,0x3f4f3cd0 }, /* 395:  9.4333696e-01  8.0952168e-01 */
    { 0xbf7c7dd7,0xbf558033 }, /* 396: -9.8629516e-01 -8.3398741e-01 */
    { 0xbef2f051,0xbee9ecc7 }, /* 397: -4.7448972e-01 -4.5688459e-01 */
    { 0xbf07e2a3,0xbf0197f8 }, /* 398: -5.3080195e-01 -5.0622511e-01 */
    { 0x3f3631c1,0x3f2732d7 }, /* 399:  7.1169668e-01  6.5311950e-01 */
    { 0xbf5e0b9d,0xbf433b86 }, /* 400: -8.6736470e-01 -7.6262701e-01 */
    { 0x3f823922,0x3f59c94d }, /* 401:  1.0173686e+00  8.5072786e-01 */
    { 0x3f682bfe,0x3f49a0d7 }, /* 402:  9.0692127e-01  7.8761047e-01 */
    { 0x3f357677,0x3f26a4d7 }, /* 403:  7.0883888e-01  6.5095276e-01 */
    { 0xbe75a363,0xbe734a09 }, /* 404: -2.3988108e-01 -2.3758711e-01 */
    { 0xbec8ad22,0xbec393dd }, /* 405: -3.9194590e-01 -3.8198748e-01 */
    { 0xbf000fe2,0xbef59324 }, /* 406: -5.0024235e-01 -4.7963822e-01 */
    { 0xbf2d4fe5,0xbf205f89 }, /* 407: -6.7700034e-01 -6.2645775e-01 */
    { 0x3f16138c,0x3f0da06e }, /* 408:  5.8623576e-01  5.5322921e-01 */
    { 0xbeffe68e,0xbef560ef }, /* 409: -4.9980587e-01 -4.7925517e-01 */
    { 0xbebc7ad9,0xbeb8406a }, /* 410: -3.6812475e-01 -3.5986644e-01 */
    { 0x3ef5017f,0x3eebc302 }, /* 411:  4.7852704e-01  4.6047217e-01 */
    { 0xbf5c401e,0xbf421113 }, /* 412: -8.6035335e-01 -7.5807303e-01 */
    { 0x3f4adb4c,0x3f3648af }, /* 413:  7.9240870e-01  7.1204656e-01 */
    { 0x3f687a9f,0x3f49d140 }, /* 414:  9.0812105e-01  7.8834915e-01 */
    { 0x3f686cfc,0x3f49c8dc }, /* 415:  9.0791297e-01  7.8822112e-01 */
    { 0x3f0c094a,0x3f052800 }, /* 416:  5.4701674e-01  5.2014160e-01 */
    { 0x3f6061bc,0x3f44bc53 }, /* 417:  8.7649131e-01  7.6849860e-01 */
    { 0x3efe173d,0x3ef3c9e6 }, /* 418:  4.9627104e-01  4.7614974e-01 */
    { 0x3f2eb081,0x3f2171c8 }, /* 419:  6.8238074e-01  6.3064241e-01 */
    { 0x40cbde3d,0x3db35db0 }, /* 420:  6.3708787e+00  8.7581038e-02 */
    { 0xbf6ee5a3,0xbf4db3bd }, /* 421: -9.3319148e-01 -8.0352384e-01 */
    { 0xbf3352d5,0xbf2503a4 }, /* 422: -7.0048267e-01 -6.4458680e-01 */
    { 0x3f01dc8d,0x3ef8ba05 }, /* 423:  5.0727159e-01  4.8579422e-01 */
    { 0xbf490373,0xbf34fc2d }, /* 424: -7.8520888e-01 -7.0697290e-01 */
    { 0xbf46507c,0xbf3310ef }, /* 425: -7.7466559e-01 -6.9947714e-01 */
    { 0xbfd92732,0xbf7dfad4 }, /* 426: -1.6965086e+00 -9.9210858e-01 */
    { 0x3f9e50ac,0x3f71db43 }, /* 427:  1.2368369e+00  9.4475192e-01 */
    { 0xbf4df849,0xbf3874cf }, /* 428: -8.0456978e-01 -7.2053236e-01 */
    { 0xbf4929ff,0xbf35176e }, /* 429: -7.8579706e-01 -7.0738876e-01 */
    { 0x3f1c1645,0x3f12981f }, /* 430:  6.0971481e-01  5.7263368e-01 */
    { 0xbf4b5aca,0xbf36a21d }, /* 431: -7.9435408e-01 -7.1341115e-01 */
    { 0x3f116125,0x3f09b0c0 }, /* 432:  5.6788856e-01  5.3785324e-01 */
    { 0x3f831544,0x3f5aaf73 }, /* 433:  1.0240865e+00  8.5423964e-01 */
    { 0xbf17a4c7,0xbf0eedfc }, /* 434: -5.9235805e-01 -5.5831885e-01 */
    { 0xbf0b0190,0xbf04467b }, /* 435: -5.4299259e-01 -5.1670045e-01 */
    { 0x3f16125f,0x3f0d9f74 }, /* 436:  5.8621782e-01  5.5321431e-01 */
    { 0xbfb019b5,0xbf7b25ca }, /* 437: -1.3757845e+00 -9.8104537e-01 */
    { 0xbf9ac1ee,0xbf6f6e5e }, /* 438: -1.2090433e+00 -9.3527782e-01 */
    { 0x3f6a8052,0x3f4b0e23 }, /* 439:  9.1602051e-01  7.9318446e-01 */
    { 0x3ef72ea9,0x3eedb10b }, /* 440:  4.8277786e-01  4.6424136e-01 */
    { 0xbf7ef61d,0xbf56da88 }, /* 441: -9.9594289e-01 -8.3927202e-01 */
    { 0x3f311c0d,0x3f2350be }, /* 442:  6.9183427e-01  6.3795078e-01 */
    { 0xbf45cae1,0xbf32b15b }, /* 443: -7.7262694e-01 -6.9801873e-01 */
    { 0x3f321019,0x3f240c63 }, /* 444:  6.9555813e-01  6.4081401e-01 */
    { 0xbee17f06,0xbeda46ce }, /* 445: -4.4042224e-01 -4.2632145e-01 */
    { 0xbfb2a17a,0xbf7c1442 }, /* 446: -1.3955529e+00 -9.8468411e-01 */
    { 0xbfc03a75,0xbf7f63ff }, /* 447: -1.5017840e+00 -9.9761957e-01 */
    { 0xbf35e4e9,0xbf26f89f }, /* 448: -7.1052414e-01 -6.5223116e-01 */
    { 0x3fb5c886,0x3f7d1a13 }, /* 449:  1.4201820e+00  9.8867911e-01 */
    { 0x3ef16474,0x3ee88c60 }, /* 450:  4.7146952e-01  4.5419598e-01 */
    { 0xbdd8eadc,0xbdd88317 }, /* 451: -1.0591671e-01 -1.0571878e-01 */
    { 0xbf7288a3,0xbf4fd89b }, /* 452: -9.4739741e-01 -8.1189889e-01 */
    { 0xbf4b0b3c,0xbf366a54 }, /* 453: -7.9314017e-01 -7.1255994e-01 */
    { 0x3f04c1de,0x3efdc5d9 }, /* 454:  5.1858318e-01  4.9565008e-01 */
    { 0x4013ced0,0x3f3d459e }, /* 455:  2.3094978e+00  7.3934352e-01 */
    { 0xbf344cf3,0xbf25c28e }, /* 456: -7.0429915e-01 -6.4749992e-01 */
    { 0xbf6bb651,0xbf4bca56 }, /* 457: -9.2075068e-01 -7.9605615e-01 */
    { 0x3ee7608c,0x3edf94f1 }, /* 458:  4.5190847e-01  4.3668321e-01 */
    { 0xbeff305f,0xbef4c0f9 }, /* 459: -4.9841592e-01 -4.7803476e-01 */
    { 0xbf4559fa,0xbf326071 }, /* 460: -7.7090418e-01 -6.9678408e-01 */
    { 0xbf7ec67d,0xbf56c09f }, /* 461: -9.9521619e-01 -8.3887666e-01 */
    { 0xbfca11ae,0xbf7ffdf9 }, /* 462: -1.5786645e+00 -9.9996907e-01 */
    { 0xbf323138,0xbf2425cf }, /* 463: -6.9606352e-01 -6.4120191e-01 */
    { 0xbee0ea64,0xbed9c052 }, /* 464: -4.3928826e-01 -4.2529541e-01 */
    { 0x3f71476f,0x3f4f1c73 }, /* 465:  9.4249624e-01  8.0902785e-01 */
    { 0xbea3aad9,0xbea0e4eb }, /* 466: -3.1966284e-01 -3.1424651e-01 */
    { 0x3f649894,0x3f4767c4 }, /* 467:  8.9295316e-01  7.7892709e-01 */
    { 0xbf5d7054,0xbf42d6f0 }, /* 468: -8.6499524e-01 -7.6109219e-01 */
    { 0xbf4ad214,0xbf364236 }, /* 469: -7.9226804e-01 -7.1194780e-01 */
    { 0x3f0dcc5a,0x3f06a86f }, /* 470:  5.5389941e-01  5.2600759e-01 */
    { 0xbee416e6,0xbedc9e9c }, /* 471: -4.4548720e-01 -4.3089759e-01 */
    { 0x3f6b397b,0x3f4b7eb0 }, /* 472:  9.1884583e-01  7.9490185e-01 */
    { 0x3f0446a7,0x3efcefb4 }, /* 473:  5.1670307e-01  4.9401629e-01 */
    { 0x3fc87e0a,0x3f7fff5a }, /* 474:  1.5663464e+00  9.9999011e-01 */
    { 0x3f64cfd5,0x3f478a66 }, /* 475:  8.9379627e-01  7.7945554e-01 */
    { 0xbf4ede55,0xbf39140a }, /* 476: -8.0808002e-01 -7.2296202e-01 */
    { 0xbf3617aa,0xbf271f15 }, /* 477: -7.1129858e-01 -6.5281802e-01 */
    { 0xbf17b391,0xbf0efa41 }, /* 478: -5.9258372e-01 -5.5850607e-01 */
    { 0x3ed31035,0x3ecd22d2 }, /* 479:  4.1223302e-01  4.0065628e-01 */
    { 0xbf485de7,0xbf3486f3 }, /* 480: -7.8268284e-01 -7.0518416e-01 */
    { 0xbf025a07,0xbef99541 }, /* 481: -5.0918621e-01 -4.8746684e-01 */
    { 0x3f9bb002,0x3f701544 }, /* 482:  1.2163088e+00  9.3782449e-01 */
    { 0xbf79edf8,0xbf541392 }, /* 483: -9.7628736e-01 -8.2842362e-01 */
    { 0xbb449600,0xbb4495ed }, /* 484: -2.9996634e-03 -2.9996589e-03 */
    { 0xbf93475f,0xbf69bb5e }, /* 485: -1.1506156e+00 -9.1301525e-01 */
    { 0xbd929b7f,0xbd927b73 }, /* 486: -7.1585648e-02 -7.1524523e-02 */
    { 0x405b2257,0xbe8ea9f9 }, /* 487:  3.4239709e+00 -2.7864054e-01 */
    { 0xbf3394a1,0xbf2535ec }, /* 488: -7.0148665e-01 -6.4535403e-01 */
    { 0x3f5f5853,0x3f441216 }, /* 489:  8.7244147e-01  7.6590097e-01 */
    { 0x3f46ec49,0x3f338026 }, /* 490:  7.7704293e-01  7.0117414e-01 */
    { 0x3f5020f0,0x3f39f259 }, /* 491:  8.1300259e-01  7.2635418e-01 */
    { 0xbf6a7db2,0xbf4b0c8a }, /* 492: -9.1598046e-01 -7.9316008e-01 */
    { 0x3f4b06ed,0x3f36674f }, /* 493:  7.9307443e-01  7.1251386e-01 */
    { 0x3f7e0d88,0x3f565bba }, /* 494:  9.9239397e-01  8.3733714e-01 */
    { 0x3f2f769c,0x3f220b57 }, /* 495:  6.8540359e-01  6.3298553e-01 */
    { 0x3cd8060b,0x3cd7ffa2 }, /* 496:  2.6370069e-02  2.6367012e-02 */
    { 0x3f34c515,0x3f261e08 }, /* 497:  7.0613223e-01  6.4889574e-01 */
    { 0x3f4e84b9,0x3f38d616 }, /* 498:  8.0671269e-01  7.2201669e-01 */
    { 0xbf17ac5b,0xbf0ef446 }, /* 499: -5.9247369e-01 -5.5841482e-01 */
    { 0x3f67f4e3,0x3f497edd }, /* 500:  9.0608042e-01  7.8709203e-01 */
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
    union lofl_u x, z;
    unsigned long v1, z1, r;
    int i;
    
    for (i = 0; i < (int) (sizeof(t) / sizeof(t[0])); i++) {
	x.lo = pgm_read_dword (& t[i].x);
	z.lo = pgm_read_dword (& t[i].z);
	v.fl = sin (x.fl);
	
	v1 = (v.lo < 0) ? (unsigned long)~(v.lo) : v.lo + 0x80000000;
	z1 = (z.lo < 0) ? (unsigned long)~(z.lo) : z.lo + 0x80000000;
	r = (v1 >= z1) ? v1 - z1 : z1 - v1;
	
	if (r > 2) x_exit (i+1);
    }
    return 0;
}
