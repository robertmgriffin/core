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

/* RSA Functions - see main program below */

#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <time.h>

#include "rsa_WWW.h"
//#include "rsa_support.h"

#define ROUNDUP(a,b) ((a)-1)/(b)+1

/* Mask Generation Function */

static void MGF1(int sha, octet *z, int olen, octet *mask)
{
    char h[64];
    octet H = {0, sizeof(h), h};
    int hlen = sha;
    int counter, cthreshold;

    OCT_empty(mask);

    cthreshold = ROUNDUP(olen, hlen);
    for (counter = 0; counter < cthreshold; counter++)
    {
        GPhash(MC_SHA2,sha,&H,0, 0,z,counter,NULL);
        //hashit(sha, z, counter, &H);
        if (mask->len + hlen > olen) OCT_jbytes(mask, H.val, olen % hlen);
        else                     OCT_joctet(mask, &H);
    }
    OCT_clear(&H);
}

/* SHAXXX identifier strings */
const unsigned char SHA256ID[] = {0x30, 0x31, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x01, 0x05, 0x00, 0x04, 0x20};
const unsigned char SHA384ID[] = {0x30, 0x41, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x02, 0x05, 0x00, 0x04, 0x30};
const unsigned char SHA512ID[] = {0x30, 0x51, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x03, 0x05, 0x00, 0x04, 0x40};

/* PKCS 1.5 padding of a message to be signed */

int PKCS15_WWW(int sha, octet *m, octet *w)
{
    int olen = w->max;
    int hlen = sha;
    int idlen = 19;
    char h[64];
    octet H = {0, sizeof(h), h};

    if (olen < idlen + hlen + 10) return 0;
    GPhash(MC_SHA2,sha,&H,0,0,m,-1,NULL);
    //hashit(sha, m, -1, &H);

    OCT_empty(w);
    OCT_jbyte(w, 0x00, 1);
    OCT_jbyte(w, 0x01, 1);
    OCT_jbyte(w, 0xff, olen - idlen - hlen - 3);
    OCT_jbyte(w, 0x00, 1);

    if (hlen == 32) OCT_jbytes(w, (char *)SHA256ID, idlen);
    if (hlen == 48) OCT_jbytes(w, (char *)SHA384ID, idlen);
    if (hlen == 64) OCT_jbytes(w, (char *)SHA512ID, idlen);

    OCT_joctet(w, &H);

    return 1;
}

/* OAEP Message Encoding for Encryption */

int OAEP_ENCODE_WWW(int sha, octet *m, csprng *RNG, octet *p, octet *f)
{
    int slen, olen = f->max - 1;
    int mlen = m->len;
    int hlen, seedlen;
    char dbmask[MAX_RSA_BYTES], seed[64];
    octet DBMASK = {0, sizeof(dbmask), dbmask};
    octet SEED = {0, sizeof(seed), seed};

    hlen = seedlen = sha;
    if (mlen > olen - hlen - seedlen - 1) return 0;
    if (m == f) return 0; /* must be distinct octets */

    GPhash(MC_SHA2,sha,f,0,0,p,-1,NULL);
    //hashit(sha, p, -1, f);

    slen = olen - mlen - hlen - seedlen - 1;

    OCT_jbyte(f, 0, slen);
    OCT_jbyte(f, 0x1, 1);
    OCT_joctet(f, m);

    OCT_rand(&SEED, RNG, seedlen);

    MGF1(sha, &SEED, olen - seedlen, &DBMASK);

    OCT_xor(&DBMASK, f);
    MGF1(sha, &DBMASK, seedlen, f);

    OCT_xor(f, &SEED);

    OCT_joctet(f, &DBMASK);

    OCT_pad(f, f->max);
    OCT_clear(&SEED);
    OCT_clear(&DBMASK);

    return 1;
}

/* OAEP Message Decoding for Decryption */

int OAEP_DECODE_WWW(int sha, octet *p, octet *f)
{
    int comp, x, t;
    int i, k, olen = f->max - 1;
    int hlen, seedlen;
    char dbmask[MAX_RSA_BYTES], seed[64], chash[64];
    octet DBMASK = {0, sizeof(dbmask), dbmask};
    octet SEED = {0, sizeof(seed), seed};
    octet CHASH = {0, sizeof(chash), chash};

    seedlen = hlen = sha;
    if (olen < seedlen + hlen + 1) return 0;
    if (!OCT_pad(f, olen + 1)) return 0;

    GPhash(MC_SHA2,sha,&CHASH,0,0,p,-1,NULL);
    //hashit(sha, p, -1, &CHASH);

    x = f->val[0];
    for (i = seedlen; i < olen; i++)
        DBMASK.val[i - seedlen] = f->val[i + 1];
    DBMASK.len = olen - seedlen;

    MGF1(sha, &DBMASK, seedlen, &SEED);
    for (i = 0; i < seedlen; i++) SEED.val[i] ^= f->val[i + 1];
    MGF1(sha, &SEED, olen - seedlen, f);
    OCT_xor(&DBMASK, f);

    comp = OCT_ncomp(&CHASH, &DBMASK, hlen);

    OCT_shl(&DBMASK, hlen);

    OCT_clear(&SEED);
    OCT_clear(&CHASH);

    for (k = 0;; k++)
    {
        if (k >= DBMASK.len)
        {
            OCT_clear(&DBMASK);
            return 0;
        }
        if (DBMASK.val[k] != 0) break;
    }

    t = DBMASK.val[k];
    if (!comp || x != 0 || t != 0x01)
    {
        OCT_clear(&DBMASK);
        return 0;
    }

    OCT_shl(&DBMASK, k + 1);
    OCT_copy(f, &DBMASK);
    OCT_clear(&DBMASK);

    return 1;
}


/* generate an RSA key pair */
void RSA_WWW_KEY_PAIR(csprng *RNG, sign32 e, rsa_private_key_WWW *PRIV, rsa_public_key_WWW *PUB, octet *P, octet* Q)
{
    /* IEEE1363 A16.11/A16.12 more or less */
    BIG_XXX t[HFLEN_WWW], p1[HFLEN_WWW], q1[HFLEN_WWW];

    if (RNG != NULL)
    {

        for (;;)
        {

            FF_WWW_random(PRIV->p, RNG, HFLEN_WWW);
            while (FF_WWW_lastbits(PRIV->p, 2) != 3) FF_WWW_inc(PRIV->p, 1, HFLEN_WWW);
            while (!FF_WWW_prime(PRIV->p, RNG, HFLEN_WWW))
                FF_WWW_inc(PRIV->p, 4, HFLEN_WWW);

            FF_WWW_copy(p1, PRIV->p, HFLEN_WWW);
            FF_WWW_dec(p1, 1, HFLEN_WWW);

            if (FF_WWW_cfactor(p1, e, HFLEN_WWW)) continue;
            break;
        }

        for (;;)
        {
            FF_WWW_random(PRIV->q, RNG, HFLEN_WWW);
            while (FF_WWW_lastbits(PRIV->q, 2) != 3) FF_WWW_inc(PRIV->q, 1, HFLEN_WWW);
            while (!FF_WWW_prime(PRIV->q, RNG, HFLEN_WWW))
                FF_WWW_inc(PRIV->q, 4, HFLEN_WWW);

            FF_WWW_copy(q1, PRIV->q, HFLEN_WWW);
            FF_WWW_dec(q1, 1, HFLEN_WWW);
            if (FF_WWW_cfactor(q1, e, HFLEN_WWW)) continue;

            break;
        }

    }
    else
    {
        FF_WWW_fromOctet(PRIV->p, P, HFLEN_WWW);
        FF_WWW_fromOctet(PRIV->q, Q, HFLEN_WWW);

        FF_WWW_copy(p1, PRIV->p, HFLEN_WWW);
        FF_WWW_dec(p1, 1, HFLEN_WWW);

        FF_WWW_copy(q1, PRIV->q, HFLEN_WWW);
        FF_WWW_dec(q1, 1, HFLEN_WWW);
    }

    FF_WWW_mul(PUB->n, PRIV->p, PRIV->q, HFLEN_WWW);
    PUB->e = e;

    FF_WWW_copy(t, p1, HFLEN_WWW);
    FF_WWW_shr(t, HFLEN_WWW);
    FF_WWW_init(PRIV->dp, e, HFLEN_WWW);
    FF_WWW_invmodp(PRIV->dp, PRIV->dp, t, HFLEN_WWW);
    if (FF_WWW_parity(PRIV->dp) == 0) FF_WWW_add(PRIV->dp, PRIV->dp, t, HFLEN_WWW);
    FF_WWW_norm(PRIV->dp, HFLEN_WWW);

    FF_WWW_copy(t, q1, HFLEN_WWW);
    FF_WWW_shr(t, HFLEN_WWW);
    FF_WWW_init(PRIV->dq, e, HFLEN_WWW);
    FF_WWW_invmodp(PRIV->dq, PRIV->dq, t, HFLEN_WWW);
    if (FF_WWW_parity(PRIV->dq) == 0) FF_WWW_add(PRIV->dq, PRIV->dq, t, HFLEN_WWW);
    FF_WWW_norm(PRIV->dq, HFLEN_WWW);

    FF_WWW_invmodp(PRIV->c, PRIV->p, PRIV->q, HFLEN_WWW);

    return;
}

/* destroy the Private Key structure */
void RSA_WWW_PRIVATE_KEY_KILL(rsa_private_key_WWW *PRIV)
{
    FF_WWW_zero(PRIV->p, HFLEN_WWW);
    FF_WWW_zero(PRIV->q, HFLEN_WWW);
    FF_WWW_zero(PRIV->dp, HFLEN_WWW);
    FF_WWW_zero(PRIV->dq, HFLEN_WWW);
    FF_WWW_zero(PRIV->c, HFLEN_WWW);
}

void RSA_WWW_fromOctet(BIG_XXX x[], octet *w)
{
    FF_WWW_fromOctet(x, w, FFLEN_WWW);
}

/* RSA encryption with the public key */
void RSA_WWW_ENCRYPT(rsa_public_key_WWW *PUB, octet *F, octet *G)
{
    BIG_XXX f[FFLEN_WWW];
    FF_WWW_fromOctet(f, F, FFLEN_WWW);

    FF_WWW_power(f, f, PUB->e, PUB->n, FFLEN_WWW);

    FF_WWW_toOctet(G, f, FFLEN_WWW);
}

/* RSA decryption with the private key */
void RSA_WWW_DECRYPT(rsa_private_key_WWW *PRIV, octet *G, octet *F)
{
    BIG_XXX g[FFLEN_WWW], t[FFLEN_WWW], jp[HFLEN_WWW], jq[HFLEN_WWW];

    FF_WWW_fromOctet(g, G, FFLEN_WWW);

    FF_WWW_dmod(jp, g, PRIV->p, HFLEN_WWW);
    FF_WWW_dmod(jq, g, PRIV->q, HFLEN_WWW);

    FF_WWW_skpow(jp, jp, PRIV->dp, PRIV->p, HFLEN_WWW);
    FF_WWW_skpow(jq, jq, PRIV->dq, PRIV->q, HFLEN_WWW);


    FF_WWW_zero(g, FFLEN_WWW);
    FF_WWW_copy(g, jp, HFLEN_WWW);
    FF_WWW_mod(jp, PRIV->q, HFLEN_WWW);
    if (FF_WWW_comp(jp, jq, HFLEN_WWW) > 0)
        FF_WWW_add(jq, jq, PRIV->q, HFLEN_WWW);
    FF_WWW_sub(jq, jq, jp, HFLEN_WWW);
    FF_WWW_norm(jq, HFLEN_WWW);

    FF_WWW_mul(t, PRIV->c, jq, HFLEN_WWW);
    FF_WWW_dmod(jq, t, PRIV->q, HFLEN_WWW);

    FF_WWW_mul(t, jq, PRIV->p, HFLEN_WWW);
    FF_WWW_add(g, t, g, FFLEN_WWW);
    FF_WWW_norm(g, FFLEN_WWW);

    FF_WWW_toOctet(F, g, FFLEN_WWW);

    return;
}

