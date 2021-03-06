/*
 * Copyright (c) 2012-2020 MIRACL UK Ltd.
 *
 * This file is part of MIRACL Core
 * (see https://github.com/miracl/core).
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
#include "arch.h"
#include "ecp_NUMS256E.h"

namespace NUMS256E {

/*  NUMS 256-bit Curve - Edwards */

#if CHUNK==16

using namespace B256_13;

const int CURVE_Cof_I=4;
const BIG CURVE_Cof= {0x4,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0};
const int CURVE_A= 1;
const int CURVE_B_I= -15342;
const BIG CURVE_B= {0x355,0x1FFE,0x1FFF,0x1FFF,0x1FFF,0x1FFF,0x1FFF,0x1FFF,0x1FFF,0x1FFF,0x1FFF,0x1FFF,0x1FFF,0x1FFF,0x1FFF,0x1FFF,0x1FFF,0x1FFF,0x1FFF,0x1FF};
const BIG CURVE_Order= {0xAF5,0x16EA,0x43B,0xF63,0x11A4,0x1CD,0x1D65,0x14A5,0x155A,0x20C,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x80};
const BIG CURVE_Gx= {0x13DA,0x1768,0x40B,0x1D81,0xA0D,0x1AC3,0xC20,0x1DC,0x198A,0x1061,0x6F5,0x1241,0x15F6,0xF1E,0x1734,0x46F,0xAEA,0x7DB,0x1D45,0x114};
const BIG CURVE_Gy= {0x9E6,0xC54,0x19DE,0xC2D,0x12FA,0x1769,0x215,0x1B02,0x1F61,0x38A,0x4,0xC97,0x1D9A,0xB32,0x1F3A,0x4B8,0x19D9,0x14FE,0x154F,0x89};

#endif

#if CHUNK==32

using namespace B256_29;

const int CURVE_Cof_I=4;
const BIG CURVE_Cof= {0x4,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0};
const int CURVE_A= 1;
const int CURVE_B_I= -15342;
const BIG CURVE_B= {0x1FFFC355,0x1FFFFFFF,0x1FFFFFFF,0x1FFFFFFF,0x1FFFFFFF,0x1FFFFFFF,0x1FFFFFFF,0x1FFFFFFF,0xFFFFFF};
const BIG CURVE_Order= {0xEDD4AF5,0x123D8C87,0x1650E6C6,0xAB54A5E,0x419,0x0,0x0,0x0,0x400000};
const BIG CURVE_Gx= {0xEED13DA,0x6F60481,0x20D61A8,0x13141DC6,0x9BD60C3,0x1EAFB490,0xDF73478,0x1F6D5D44,0x8A7514};
const BIG CURVE_Gy= {0x198A89E6,0x1D30B73B,0x15BB4CB,0x1EC3B021,0x18010715,0x12ECD325,0x171F3A59,0x13FB3B24,0x44D53E};

#endif

#if CHUNK==64

using namespace B256_56;

const int CURVE_Cof_I=4;
const BIG CURVE_Cof= {0x4L,0x0L,0x0L,0x0L,0x0L};
const int CURVE_A= 1;
const int CURVE_B_I= -15342;
const BIG CURVE_B= {0xFFFFFFFFFFC355L,0xFFFFFFFFFFFFFFL,0xFFFFFFFFFFFFFFL,0xFFFFFFFFFFFFFFL,0xFFFFFFFFL};
const BIG CURVE_Order= {0x47B190EEDD4AF5L,0x5AA52F59439B1AL,0x4195L,0x0L,0x40000000L};
const BIG CURVE_Gx= {0xDEC0902EED13DAL,0x8A0EE3083586A0L,0x5F69209BD60C39L,0x6AEA237DCD1E3DL,0x8A7514FBL};
const BIG CURVE_Gy= {0xA616E7798A89E6L,0x61D810856ED32FL,0xD9A64B8010715FL,0xD9D925C7CE9665L,0x44D53E9FL};

#endif

}
