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
public struct ROM{
 
#if D32
// Base Bits= 29
//  bls24 Curve Modulus
    
static let Modulus:[Chunk] = [0xA06152B,0x2260B3A,0xB4C36BE,0x5FFC5D0,0xBDB6A64,0x5B78E2E,0x1C1A28CA,0x10E6441B,0x1F244061,0xB4704F0,0x141E5CCD,0x9837504,0x3F2E77E,0xD763740,0x1316EA0E,0xF0079,0x555C]
static let ROI:[Chunk] = [0xA06152A,0x2260B3A,0xB4C36BE,0x5FFC5D0,0xBDB6A64,0x5B78E2E,0x1C1A28CA,0x10E6441B,0x1F244061,0xB4704F0,0x141E5CCD,0x9837504,0x3F2E77E,0xD763740,0x1316EA0E,0xF0079,0x555C]
static let R2modp:[Chunk] = [0x8533EA9,0x6A02789,0x183B24DE,0x1E45ECF8,0xC8F8F37,0x10CAD209,0x4C0C4B8,0x9B1FABD,0xDEBE4C0,0xDC353F9,0x18A18E26,0x10F489BB,0x31206A5,0x19673BBF,0x6BE69F9,0xB091169,0x9CD]
static let MConst:Chunk = 0x95FE7D
static public let Fra:[Chunk] = [0x1BF96F1D,0xAE53A55,0x31BFEEB,0x183FF17A,0x6237469,0x12A4F4F1,0x12101FE3,0x16E79D94,0xFF59267,0x5EB4EB4,0x78CC49F,0x274BA33,0x149184F3,0x16C6DCBA,0x1C90B694,0x10F729CE,0x4BBC]
static public let Frb:[Chunk] = [0xE0CA60E,0x1740D0E4,0x83037D2,0xDBFD456,0x5B7F5FA,0x1312993D,0xA0A08E6,0x19FEA687,0xF2EADF9,0x55BB63C,0xC91982E,0x70EBAD1,0xF61628B,0x16AF5A85,0x16863379,0xF17D6AA,0x99F]

static let CURVE_Cof_I:Int = 0
static let CURVE_A:Int = 0
static let CURVE_B_I:Int = 19
static public let CURVE_B:[Chunk] = [0x13,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]
static public let CURVE_Order:[Chunk] = [0x10000001,0xD047FF,0x1FD54464,0x1E3CE067,0xE322DDA,0x1D356F3F,0x7433B44,0x49091F9,0x1729CC2,0x250286C,0x16E62ED,0xB403E1E,0x1001000,0x80,0x0,0x0,0x0]
static public let CURVE_Gx:[Chunk] = [0xBE3CCD4,0x33B07AF,0x1B67D159,0x3DFC5B5,0xEBA1FCC,0x1A3C1F84,0x56BE204,0xEF8DF1B,0x11AE2D84,0x5FEE546,0x161B3BF9,0x183B20EE,0x1EA5D99B,0x14F0C5BF,0xBE521B7,0x17C682F9,0x1AB2]
static public let CURVE_Gy:[Chunk] = [0x121E5245,0x65D2E56,0x11577DB1,0x16DACC11,0x14F39746,0x459F694,0x12483FCF,0xC828B04,0xFD63E5A,0x7B1D52,0xAFDE738,0xF349254,0x1A4529FF,0x10E53353,0xF91DEE1,0x16E18D8A,0x47FC]

static public let CURVE_Bnx:[Chunk] = [0x11FF80,0x80010,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]
static public let CURVE_Cof:[Chunk] = [0x11FF7F,0x80010,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]

//static public let CURVE_Cof:[Chunk] = [0x19F415AB,0x1E0FFDFF,0x15AAADFF,0xAA,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]
static public let CURVE_Cru:[Chunk] = [0xDD794A9,0x1DE138A3,0x2BCCE90,0xC746127,0x15223DDC,0x1DD8890B,0xED08DB7,0xE24B9F,0xE379CE6,0x37011AC,0x11BAC820,0x1EEFAD01,0x200860F,0x147218A6,0xF16A209,0xF0079,0x555C]
static public let CURVE_Pxaa:[Chunk] = [0x14E24678,0x1F149A9B,0x9609022,0x1C186868,0xCDEFC69,0x1C87BB2E,0x14A2235F,0x7586755,0x5896747,0x159BFE92,0x3B5572E,0x1710A521,0x71EB14A,0xC643C33,0x12581DE5,0x1BCA747D,0x959]
static public let CURVE_Pxab:[Chunk] = [0x1FB099B8,0x3FCF5D7,0x4A91C0E,0xC6EEB40,0x11FC2385,0x11B5AE8D,0x1A9CC3E7,0x194FE144,0x185DB2A5,0x930E1C7,0x14F85F9A,0x1F2ED4E,0x1D1BE5AD,0xF26169C,0xCF7F194,0x1DA1062E,0x3B0D]
static public let CURVE_Pxba:[Chunk] = [0x11AD15D3,0xD0E6F38,0x17DB85BB,0x30A62F1,0x1EA3E09A,0x17B25FA1,0x1B7959AC,0x1165B19A,0x6C74FDB,0x18F790E1,0x12278FDA,0x1E008F79,0x103F329,0x14619FF1,0x1EBCAA8,0xFF5A9CA,0x3EC2]
static public let CURVE_Pxbb:[Chunk] = [0x1EE0F480,0x3D5943A,0xF5B12E3,0x128AADC8,0x180E1CB9,0x1EFD916F,0x48BC7F,0x1D5EE1FA,0x5698EF5,0x11D6AED9,0x1386BC6E,0x196E900B,0x1CE2E465,0xC2A8ED3,0x1E67DF99,0x71B7940,0xA5B]
static public let CURVE_Pyaa:[Chunk] = [0x14781AA0,0xC324C98,0xEDC2AC,0x16C13B46,0x145FC44B,0x12529530,0x1310A8C4,0x1768C5C0,0xE19AE68,0x56E1C1D,0x13DAF93F,0x17E94366,0xF901AD0,0x76800CC,0x10250D8B,0x1E6BAE6D,0x5057]
static public let CURVE_Pyab:[Chunk] = [0xEAE08FA,0xDDF62BF,0xA97E5AB,0xF0EE97,0x99A42CA,0x1C326578,0xF33DC11,0x8B913F7,0xFEF8552,0x19F35B90,0x58DDBDE,0xFC32FF2,0x1587B5DF,0xB5EB07A,0x1A258DE0,0x1692CC3D,0x2CE2]
static public let CURVE_Pyba:[Chunk] = [0x5F0CC41,0xB9813B5,0x14C2A87D,0xFF1264A,0x19AF8A14,0x6CE6C3,0x2A7F8A2,0x121DCA7D,0x7D37153,0x19D21078,0x15466DC7,0x1362982B,0x1DD3CB5B,0x1CFC0D1C,0x18C69AF8,0x8CC7DC,0x1807]
static public let CURVE_Pybb:[Chunk] = [0x115C1CAE,0x78D9732,0x16C26237,0x5A81A6A,0x1C38A777,0x56121FE,0x4DAD9D7,0x1BEBA670,0xA1D72FC,0xD60B274,0x19734258,0x1D621775,0x4691771,0x14206B68,0x17B22DE4,0x29D5B37,0x499D]
static let CURVE_W:[[Chunk]] = [[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]]
static let CURVE_SB:[[[Chunk]]] = [[[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]],[[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]]] 
static let CURVE_WB:[[Chunk]] = [[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]]
static let CURVE_BB:[[[Chunk]]] = [[[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]],[[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]],[[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]],[[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]]]



#endif

#if D64
// Base Bits= 56

static let Modulus:[Chunk] = [0x44C1674A06152B,0xFFE2E82D30DAF8,0x6F1C5CBDB6A642,0x3220DF068A328B,0xE09E1F24406187,0xBA825079733568,0x6E803F2E77E4C1,0x3CCC5BA839AEC,0x555C0078]
static let ROI:[Chunk] = [0x44C1674A06152A,0xFFE2E82D30DAF8,0x6F1C5CBDB6A642,0x3220DF068A328B,0xE09E1F24406187,0xBA825079733568,0x6E803F2E77E4C1,0x3CCC5BA839AEC,0x555C0078]
static let R2modp:[Chunk] = [0x6A4A1FE013DF5B,0xE8E46D4D1BDE65,0x1F841391F45C67,0x9148A4516FB28,0x4398524EDF4C88,0x41C0E241B6DCE8,0xE42C208C19411,0xA7FE6FD73A7B1C,0xFCCCA76]
static let MConst:Chunk = 0xBD5D7D8095FE7D
static public let Fra:[Chunk] = [0x5CA74ABBF96F1D,0x1FF8BD0C6FFBAD,0x49E9E26237469C,0x3CECA48407F8E5,0x69D68FF59267B7,0x5D199E33127CBD,0xB97549184F313A,0x4E77242DA52D8D,0x4BBC87B9]
static public let Frb:[Chunk] = [0xE81A1C8E0CA60E,0xDFEA2B20C0DF4A,0x25327A5B7F5FA6,0xF5343A828239A6,0x76C78F2EADF9CF,0x5D68B24660B8AB,0xB50AF61628B387,0xB555A18CDE6D5E,0x99F78BE]

static let CURVE_Cof_I:Int = 0
static let CURVE_A:Int = 0
static let CURVE_B_I:Int = 19
static public let CURVE_B:[Chunk] = [0x13,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]
static public let CURVE_Order:[Chunk] = [0x1A08FFF0000001,0x1E7033FF551190,0x6ADE7EE322DDAF,0x848FC9D0CED13A,0x50D81729CC224,0x1F0F05B98BB44A,0x10010010005A0,0x0,0x0]
static public let CURVE_Gx:[Chunk] = [0x6760F5EBE3CCD4,0xEFE2DAED9F4564,0x783F08EBA1FCC1,0xC6F8D95AF88134,0xDCA8D1AE2D8477,0x9077586CEFE4BF,0x8B7FEA5D99BC1D,0x17CAF9486DE9E1,0x1AB2BE34]
static public let CURVE_Gy:[Chunk] = [0xCBA5CAD21E5245,0x6D6608C55DF6C4,0xB3ED294F39746B,0x145824920FF3C8,0x63AA4FD63E5A64,0x492A2BF79CE00F,0x66A7A4529FF79A,0x6C53E477B861CA,0x47FCB70C]

static public let CURVE_Bnx:[Chunk] = [0x100020011FF80,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]
static public let CURVE_Cof:[Chunk] = [0x100020011FF7F,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]
//static public let CURVE_Cof:[Chunk] = [0xC1FFBFF9F415AB,0x5556AAB7FF,0x0,0x0,0x0,0x0,0x0,0x0,0x0]
static public let CURVE_Cru:[Chunk] = [0xBC27146DD794A9,0x3A30938AF33A43,0xB112175223DDC6,0x125CFBB4236DFB,0x2358E379CE607,0xD680C6EB20806E,0x314C200860FF77,0x3CBC5A88268E4,0x555C0078]
static public let CURVE_Pxaa:[Chunk] = [0xE2935374E24678,0xC34342582408B,0xF765CCDEFC69E,0xC33AAD2888D7F9,0x7FD2458967473A,0x52908ED55CBAB3,0x786671EB14AB88,0xA3EC96077958C8,0x959DE53]
static public let CURVE_Pxab:[Chunk] = [0x7F9EBAFFB099B8,0x3775A012A47038,0x6B5D1B1FC23856,0x7F0A26A730F9E3,0x1C38F85DB2A5CA,0x76A753E17E6926,0x2D39D1BE5AD0F9,0x31733DFC651E4C,0x3B0DED08]
static public let CURVE_Pxba:[Chunk] = [0xA1CDE711AD15D3,0x853178DF6E16ED,0x64BF43EA3E09A1,0x2D8CD6DE566B2F,0xF21C26C74FDB8B,0x47BCC89E3F6B1E,0x3FE2103F329F00,0x4E507AF2AA28C3,0x3EC27FAD]
static public let CURVE_Pxbb:[Chunk] = [0x7AB2875EE0F480,0x4556E43D6C4B8C,0xFB22DF80E1CB99,0xF70FD0122F1FFD,0xD5DB25698EF5EA,0x4805CE1AF1BA3A,0x1DA7CE2E465CB7,0xCA0799F7E65855,0xA5B38DB]
static public let CURVE_Pyaa:[Chunk] = [0x86499314781AA0,0x609DA303B70AB1,0xA52A6145FC44BB,0x462E04C42A3124,0xC383AE19AE68BB,0xA1B34F6BE4FCAD,0x198F901AD0BF4,0x736C094362CED0,0x5057F35D]
static public let CURVE_Pyab:[Chunk] = [0xBBEC57EEAE08FA,0x78774BAA5F96AD,0x64CAF099A42CA0,0xC89FBBCCF70478,0x6B720FEF855245,0x97F916376F7B3E,0x60F5587B5DF7E1,0x61EE89637816BD,0x2CE2B496]
static public let CURVE_Pyba:[Chunk] = [0x730276A5F0CC41,0xF89325530AA1F5,0xD9CD879AF8A147,0xEE53E8A9FE2880,0x420F07D3715390,0x4C15D519B71F3A,0x1A39DD3CB5B9B1,0x3EE631A6BE39F8,0x18070466]
static public let CURVE_Pybb:[Chunk] = [0xF1B2E6515C1CAE,0xD40D355B0988DC,0xC243FDC38A7772,0x5D338136B675CA,0x164E8A1D72FCDF,0xBBAE5CD0961AC,0xD6D04691771EB1,0xD9BDEC8B792840,0x499D14EA]
static let CURVE_W:[[Chunk]] = [[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]]
static let CURVE_SB:[[[Chunk]]] = [[[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]],[[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]]]
static let CURVE_WB:[[Chunk]] = [[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]]
static let CURVE_BB:[[[Chunk]]] = [[[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]],[[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]],[[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]],[[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0]]]

#endif



}

