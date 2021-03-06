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

use crate::arch::Chunk;
use crate::nums512w::big::NLEN;

// Base Bits= 29
// nums512 Modulus
pub const MODULUS: [Chunk; NLEN] = [
    0x1FFFFDC7, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF,
    0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF,
    0x1FFFFFFF, 0x7FFFF,
];
pub const ROI: [Chunk; NLEN] = [
    0x1FFFFDC6, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF,
    0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF,
    0x1FFFFFFF, 0x7FFFF,
];
pub const R2MODP: [Chunk; NLEN] = [
    0xB100000, 0x278, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0,
];
pub const MCONST: Chunk = 0x239;

// nums512w Curve
pub const CURVE_COF_I: isize = 1;
pub const CURVE_A: isize = -3;
pub const CURVE_B_I: isize = 121243;
pub const CURVE_COF: [Chunk; NLEN] = [
    0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
];
pub const CURVE_B: [Chunk; NLEN] = [
    0x1D99B, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
];
pub const CURVE_ORDER: [Chunk; NLEN] = [
    0x433555D, 0x10A9F9C8, 0x1F3490F3, 0xD166CC0, 0xBDC63B5, 0xC76CBE8, 0xC6D3F09, 0x1F729CF0,
    0x1F5B3CA4, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF,
    0x1FFFFFFF, 0x7FFFF,
];
pub const CURVE_GX: [Chunk; NLEN] = [
    0xCABAE57, 0x4143CAC, 0x1BD778B7, 0x1AC026FA, 0x15831D5, 0x14312AB, 0x167A4DE5, 0xA20ED66,
    0x195021A1, 0x129836CF, 0x1141B830, 0xA03ED0A, 0xCAD83BB, 0x1E9DA94C, 0xDC00A80, 0x1527B45,
    0x1447141D, 0x1D601,
];
pub const CURVE_GY: [Chunk; NLEN] = [
    0x183527A6, 0x1D043B01, 0x1F43FA48, 0x16B83C99, 0x5602CF2, 0x1420592D, 0x17A70486, 0x1B5161DD,
    0x14A28415, 0x3DE8A78, 0x3D2C983, 0x17797719, 0x197DBDEA, 0x15D88025, 0x1BBB718F, 0xAD679C1,
    0x14CA29AD, 0x4A1D2,
];
