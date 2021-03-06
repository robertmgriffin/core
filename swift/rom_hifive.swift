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

//
//  rom.swift
//
//  Created by Michael Scott on 12/06/2015.
//  Copyright (c) 2015 Michael Scott. All rights reserved.
//

public struct ROM{
 
#if D32

// Base Bits= 29
//  hifive Curve Modulus
static let Modulus:[Chunk] = [0x1FFFFFFD,0x1FFFFFFF,0x1FFFFFFF,0x1FFFFFFF,0x1FFFFFFF,0x1FFFFFFF,0x1FFFFFFF,0x1FFFFFFF,0x1FFFFFFF,0x1FFFFFFF,0x1FFFFFFF,0x1FFFF] 
static let ROI:[Chunk] = [0x1559D3B8,0x1C02413F,0xFAB18DA,0x128A7172,0xB95DA4C,0x16D4FE54,0x3107D87,0xAA7BEF3,0x1C38B2B4,0x1A93C08F,0x10F80C7B,0x1F27F]
static let R2modp:[Chunk] = [0x9000000,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]
static let MConst:Chunk = 0x3

//  hifive Curve
static let CURVE_Cof_I:Int = 8
static let CURVE_Cof:[Chunk] = [0x8,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]
static let CURVE_A:Int = 1
static let CURVE_B_I:Int = 11111
static let CURVE_B:[Chunk] = [0x2B67,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]
static public let CURVE_Order:[Chunk] = [0x1E9FA805,0x197CACB9,0x1E4EEA9E,0x17AD70F,0x1FA9850C,0x38A0A,0x0,0x0,0x0,0x0,0x0,0x4000]
static public let CURVE_Gx:[Chunk] = [0xC,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]
static public let CURVE_Gy:[Chunk] = [0x5FE8632,0x15F63428,0xD976C4,0x1AACA194,0x35B6DB5,0x8E3F7A,0x52D1B0E,0xF0A7A36,0x1C161D00,0x8170C70,0x1185AD59,0x181B]
 

#endif

#if D64

// Base Bits= 60
//  hifive Curve Modulus
static let Modulus:[Chunk] = [0xFFFFFFFFFFFFFFD,0xFFFFFFFFFFFFFFF,0xFFFFFFFFFFFFFFF,0xFFFFFFFFFFFFFFF,0xFFFFFFFFFFFFFFF,0xFFFFFFFFF]
static let ROI:[Chunk] = [0xB804827F559D3B8,0xC94538B93EAC636,0x1EDA9FCA8B95DA4,0xB4553DF798C41F6,0x1EF527811FC38B2,0xF93FC3E03]
static let R2modp:[Chunk] = [0x9000000000000,0x0,0x0,0x0,0x0,0x0]
static let MConst:Chunk = 0x3

//  hifive Curve
static let CURVE_Cof_I:Int = 8
static let CURVE_Cof:[Chunk] = [0x8,0x0,0x0,0x0,0x0,0x0]
static let CURVE_A:Int = 1
static let CURVE_B_I:Int = 11111
static let CURVE_B:[Chunk] = [0x2B67,0x0,0x0,0x0,0x0,0x0]
static public let CURVE_Order:[Chunk] = [0xB2F95973E9FA805,0xC0BD6B87F93BAA7,0x71415FA9850,0x0,0x0,0x200000000]
static public let CURVE_Gx:[Chunk] = [0xC,0x0,0x0,0x0,0x0,0x0]
static public let CURVE_Gy:[Chunk] = [0x2BEC68505FE8632,0x5D5650CA0365DB1,0x3811C7EF435B6DB,0x7853D1B14B46C,0x56502E18E1C161D,0xC0DC616B]
 

#endif

}

