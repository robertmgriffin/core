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
use crate::xxx::big;
use crate::xxx::big::BIG;
use crate::xxx::fp::FP;
use crate::xxx::ecp;
use crate::xxx::ecp::ECP;
use crate::xxx::dbig::DBIG;
use crate::xxx::ecp8::ECP8;
use crate::xxx::fp16::FP16;
use crate::xxx::pair256;
use crate::xxx::rom;
use crate::hmac;

/* BLS API Functions */

pub const BFS: usize = big::MODBYTES as usize;
pub const BGS: usize = big::MODBYTES as usize;
pub const BLS_OK: isize = 0;
pub const BLS_FAIL: isize = -1;

// NOTE this must be accessed in unsafe mode.
// But it is just written to once at start-up, so actually safe.
static mut G2_TAB: [FP16; ecp::G2_TABLE] = [FP16::new(); ecp::G2_TABLE];

fn ceil(a: usize,b: usize) -> usize {
    return (a-1)/b+1;
}

/* output u \in F_p */
fn hash_to_field(hash: usize,hlen: usize ,u: &mut [FP], dst: &[u8],m: &[u8],ctr: usize) {
    let q = BIG::new_ints(&rom::MODULUS);
    let el = ceil(q.nbits()+ecp::AESKEY*8,8);

    let mut okm: [u8;256]=[0;256];
    let mut fd: [u8;128]=[0;128];

    hmac::xmd_expand(hash,hlen,&mut okm,el*ctr,&dst,&m);
    for i in 0..ctr {
        for j in 0..el {
            fd[j]=okm[el*i+j];
        }
        u[i]=FP::new_big(&DBIG::frombytes(&fd[0 .. el]).dmod(&q));
    }
}

/* output u \in F_p 
fn hash_to_base(hash: usize,hlen: usize ,dst: &[u8],m: &[u8],ctr: isize) -> BIG {
    let q = BIG::new_ints(&rom::MODULUS);
    let el = ceil(q.nbits()+ecp::AESKEY*8,8);
    let tag = String::from("H2C");
    let tl = tag.len();
    let t=tag.as_bytes();
    let mut info:[u8; 16] = [0; 16];
    for i in 0..tl {
        info[i]=t[i];
    }
    info[tl]=ctr as u8;

    let mut prk: [u8;64]=[0;64];
    let mut okm: [u8;128]=[0;128];

    hmac::hkdf_extract(hash,hlen,&mut prk,Some(&dst),m);
    hmac::hkdf_expand(hash,hlen,&mut okm,el,&prk,&info[0 .. tl+1]);

    let mut dx = DBIG::frombytes(&okm[0 .. el]);
    let u=dx.dmod(&q);

    return u;
} */

/* hash a message to an ECP point, using SHA2, random oracle method */
#[allow(non_snake_case)]
pub fn bls_hash_to_point(m: &[u8]) -> ECP {
    let dst= String::from("BLS_SIG_ZZZG1_XMD:SHA512-SSWU-RO-_NUL_".to_ascii_uppercase());
    let mut u: [FP; 2] = [
        FP::new(),
        FP::new(),
    ];
    hash_to_field(hmac::MC_SHA2,ecp::HASH_TYPE,&mut u,dst.as_bytes(),m,2);

    let mut P=ECP::map2point(&u[0]);
    let P1=ECP::map2point(&u[1]);
    P.add(&P1);
    P.cfp();
    P.affine();
    return P;
}

pub fn init() -> isize {
    let g = ECP8::generator();
    if g.is_infinity() {
        return BLS_FAIL;
    }
    unsafe {
        pair256::precomp(&mut G2_TAB, &g);
    }
    return BLS_OK;
}

/* generate key pair, private key s, public key w */
pub fn key_pair_generate(ikm: &[u8], s: &mut [u8], w: &mut [u8]) -> isize {
    let r = BIG::new_ints(&rom::CURVE_ORDER);   
    let el = ceil(3*ceil(r.nbits(),8),2);
    let g = ECP8::generator();
    
    let salt=String::from("BLS-SIG-KEYGEN-SALT-");
    let info=String::from("");

    let mut prk: [u8;64]=[0;64];
    let mut okm: [u8;128]=[0;128];

    let hlen=ecp::HASH_TYPE;

    hmac::hkdf_extract(hmac::MC_SHA2,hlen,&mut prk,Some(&salt.as_bytes()),ikm);
    hmac::hkdf_expand(hmac::MC_SHA2,hlen,&mut okm,el,&prk[0 .. hlen],&info.as_bytes());

    let mut dx = DBIG::frombytes(&okm[0 .. el]);
    let mut sc = dx.dmod(&r);
    sc.tobytes(s);

    pair256::g2mul(&g, &sc).tobytes(w,true);  // true for public key compression
    return BLS_OK;
}

/* Sign message m using private key s to produce signature sig */

pub fn core_sign(sig: &mut [u8], m: &[u8], s: &[u8]) -> isize {
    let d = bls_hash_to_point(m);
    let sc = BIG::frombytes(&s);
    pair256::g1mul(&d, &sc).tobytes(sig, true);
    return BLS_OK;
}

/* Verify signature given message m, the signature sig, and the public key w */

pub fn core_verify(sig: &[u8], m: &[u8], w: &[u8]) -> isize {
    let hm = bls_hash_to_point(m);

    let mut d = ECP::frombytes(&sig);
    if !pair256::g1member(&d) {
        return BLS_FAIL;
    }
    d.neg();

    let pk = ECP8::frombytes(&w);


    // Use new multi-pairing mechanism
    let mut r = pair256::initmp();
    //    pair256::another(&mut r,&g,&d);
    unsafe {
        pair256::another_pc(&mut r, &G2_TAB, &d);
    }
    pair256::another(&mut r, &pk, &hm);
    let mut v = pair256::miller(&mut r);

    //.. or alternatively
    //    let mut v = pair256::ate2(&g, &d, &pk, &hm);

    v = pair256::fexp(&v);
    if v.isunity() {
        return BLS_OK;
    }
    return BLS_FAIL;
}
