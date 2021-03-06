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

/* Boneh-Lynn-Shacham signature 256-bit API */

/* Loosely (for now) following https://datatracker.ietf.org/doc/html/draft-irtf-cfrg-bls-signature-00 */

// Minimal-signature-size variant

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "bls256_ZZZ.h"

static FP16_YYY G2_TAB[G2_TABLE_ZZZ];  // space for precomputation on fixed G2 parameter

#define CEIL(a,b) (((a)-1)/(b)+1)

/* output u[i] \in F_p */
/* https://datatracker.ietf.org/doc/draft-irtf-cfrg-hash-to-curve/ */
static void hash_to_field(int hash,int hlen,FP_YYY *u,octet *DST,octet *M, int ctr)
{
    int i,j,L;
    BIG_XXX q,w;
    DBIG_XXX dx;
    char okm[256],fd[128];
    octet OKM = {0,sizeof(okm),okm};

    BIG_XXX_rcopy(q, Modulus_YYY);
    L=CEIL(BIG_XXX_nbits(q)+CURVE_SECURITY_ZZZ,8);

    XMD_Expand(hash,hlen,&OKM,L*ctr,DST,M);
    for (i=0;i<ctr;i++)
    {
        for (j=0;j<L;j++)
            fd[j]=OKM.val[i*L+j];
        
        BIG_XXX_dfromBytesLen(dx,fd,L);
        BIG_XXX_dmod(w,dx,q);
        FP_YYY_nres(&u[i],w);
    }
}

/* output u \in F_p 
static void hash_to_base(int hash,int hlen,BIG_XXX u,octet *DST,octet *M, int ctr)
{
    int L;
    BIG_XXX q;
    DBIG_XXX dx;
    char prk[64],okm[128],info[16];
    octet PRK = {0,sizeof(prk),prk};
    octet OKM = {0,sizeof(okm),okm};
    octet INFO = {0,sizeof(info),info};

    BIG_XXX_rcopy(q, Modulus_YYY);
    L=CEIL(BIG_XXX_nbits(q)+CURVE_SECURITY_ZZZ,8);

    OCT_jstring(&INFO,(char *)"H2C");
    OCT_jint(&INFO,ctr,1);
    HKDF_Extract(hash,hlen,&PRK,DST,M);
    HKDF_Expand(hash,hlen,&OKM,L,&PRK,&INFO);

    BIG_XXX_dfromBytesLen(dx,OKM.val,L);
    BIG_XXX_dmod(u,dx,q);
} */

/* hash a message to an ECP point, using SHA2, random oracle method */
static void BLS_HASH_TO_POINT(ECP_ZZZ *P, octet *M)
{
    FP_YYY u[2];
    ECP_ZZZ P1;
    char dst[50];
    octet DST = {0,sizeof(dst),dst};

    OCT_jstring(&DST,(char *)"BLS_SIG_ZZZG1_XMD:SHA512-SSWU-RO-_NUL_");
    hash_to_field(MC_SHA2,HASH_TYPE_ZZZ,u,&DST,M,2);

    ECP_ZZZ_map2point(P,&u[0]);
    ECP_ZZZ_map2point(&P1,&u[1]);
    ECP_ZZZ_add(P,&P1);
    ECP_ZZZ_cfp(P);
    ECP_ZZZ_affine(P);
}

int BLS_ZZZ_INIT()
{
    ECP8_ZZZ G;
    if (!ECP8_ZZZ_generator(&G)) return BLS_FAIL;
    PAIR_ZZZ_precomp(G2_TAB, &G);  // should be done just once on start-up
    return BLS_OK;
}

/* generate key pair, private key S, public key W */
int BLS_ZZZ_KEY_PAIR_GENERATE(octet *IKM, octet* S, octet *W)
{
    int L;
    BIG_XXX r,s;
    DBIG_XXX dx;
    ECP8_ZZZ G;
    char salt[20],prk[HASH_TYPE_ZZZ],okm[128];
    octet SALT = {0,sizeof(salt),salt};
    octet PRK = {0,sizeof(prk),prk};
    octet OKM = {0,sizeof(okm),okm};

    BIG_XXX_rcopy(r, CURVE_Order_ZZZ);
    L=CEIL(3*CEIL(BIG_XXX_nbits(r),8),2);

    if (!ECP8_ZZZ_generator(&G)) return BLS_FAIL;

    OCT_jstring(&SALT,(char *)"BLS-SIG-KEYGEN-SALT-");
    HKDF_Extract(MC_SHA2,HASH_TYPE_ZZZ,&PRK,&SALT,IKM);
    HKDF_Expand(MC_SHA2,HASH_TYPE_ZZZ,&OKM,L,&PRK,NULL);
    BIG_XXX_dfromBytesLen(dx,OKM.val,L);
    BIG_XXX_dmod(s,dx,r);
    BIG_XXX_toBytes(S->val, s);
    S->len = MODBYTES_XXX;
    PAIR_ZZZ_G2mul(&G, s);
    ECP8_ZZZ_toOctet(W, &G, true);
    return BLS_OK;
}

/* Sign message M using private key S to produce signature SIG */
int BLS_ZZZ_CORE_SIGN(octet *SIG, octet *M, octet *S)
{
    BIG_XXX s;
    ECP_ZZZ D;
    BLS_HASH_TO_POINT(&D, M);
    BIG_XXX_fromBytes(s, S->val);
    PAIR_ZZZ_G1mul(&D, s);
    ECP_ZZZ_toOctet(SIG, &D, true); /* compress output */
    return BLS_OK;
}

/* Verify signature given message M, the signature SIG, and the public key W */
int BLS_ZZZ_CORE_VERIFY(octet *SIG, octet *M, octet *W)
{
    FP48_YYY v;
    ECP8_ZZZ G, PK;
    ECP_ZZZ D, HM;
    BLS_HASH_TO_POINT(&HM, M);

    ECP_ZZZ_fromOctet(&D, SIG);
	if (!PAIR_ZZZ_G1member(&D)) return BLS_FAIL;
    ECP_ZZZ_neg(&D);

    ECP8_ZZZ_fromOctet(&PK, W);

// Use new multi-pairing mechanism

    FP48_YYY r[ATE_BITS_ZZZ];
    PAIR_ZZZ_initmp(r);
    PAIR_ZZZ_another_pc(r, G2_TAB, &D);
    PAIR_ZZZ_another(r, &PK, &HM);
    PAIR_ZZZ_miller(&v, r);

//.. or alternatively
//    PAIR_ZZZ_double_ate(&v,&G,&D,&PK,&HM);

    PAIR_ZZZ_fexp(&v);
    if (FP48_YYY_isunity(&v)) return BLS_OK;
    return BLS_FAIL;
}

