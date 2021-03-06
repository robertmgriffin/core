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

extern crate core;
use core::gcm;

fn decode_hex(s: &str) -> Vec<u8> {
    let mut x: Vec<u8> = Vec::new();
    let mut len=s.chars().count();
    if (len%2) == 1 {
        return x;
    }
    len/=2;
    let mut j=0;
    let cv: Vec<char> = s.chars().collect();
    for _ in 0..len {
        let n1=cv[j] as usize;
        let n2=cv[j+1] as usize;
	    x.push(((n1 % 32 + 9) % 25 * 16 + (n2 % 32 + 9) % 25) as u8); j+=2;
    }
    return x;
}

fn printbinary(array: &[u8]) {
    for i in 0..array.len() {
        print!("{:02X}", array[i])
    }
    println!("")
}

fn hpke_c25519() {
    //use core::ed25519;
    use core::c25519::hpke;
    use core::c25519::ecdh;
    use core::c25519::ecp;

    const EFS: usize = ecdh::EFS;
    const EGS: usize = ecdh::EGS;
    const EAS: usize = ecp::AESKEY;

	let config_id=0x2A;
    let mut pkE: [u8; EFS] = [0; EFS];
    let mut z: [u8;2*EFS] = [0; 2*EFS];

    let mut key:[u8;32]=[0;32];
    let mut nonce:[u8;12]=[0;12];

	let info=decode_hex("4f6465206f6e2061204772656369616e2055726e");
	let psk=decode_hex("6d656c6c6f6e");
	let pskID=decode_hex("456e6e796e20447572696e206172616e204d6f726961");
	let plain=decode_hex("4265617574792069732074727574682c20747275746820626561757479");
	let aad=decode_hex("436f756e742d30");

    let mut cipher:[u8;32]=[0;32];
    let mut tag:[u8;16]=[0;16];
    println!("\nTesting HPKE code for curve c25519");
// Mode 0
    let mut mode=0;
    println!("\nMode 0");
	let mut skR=decode_hex("139c8d38df6d8dfa2c8de98af621667c76c3f63f65c7c3966c4258c316f05033");
	let mut skE=decode_hex("7227b3fee6b8e01b293b826ec8671b69894a1142981bb1513afa42819f2a22ef");
	let mut pkR=decode_hex("186c394e175b7b161760b1bd5b822a0804bd066b170c695c0df123176fa7df6f");
   
	hpke::encap(config_id,None,&mut skE,&mut z[0..EFS],&mut pkE,&mut pkR);
	print!("pkE= "); printbinary(&pkE);
	print!("Encapsulated z= "); printbinary(&z[0..EFS]);

	hpke::decap(config_id,&mut z[0..EFS],&mut pkE,&mut skR);
	print!("Decapsulated z= "); printbinary(&z[0..EFS]);

	hpke::keyschedule(config_id,&mut key[0..EAS],&mut nonce,mode,&mut pkR,&mut z[0..EFS],&mut pkE,&info,None,None,None);
	print!("key= "); printbinary(&key[0..EAS]);
	print!("nonce= "); printbinary(&nonce);

	gcm::encrypt(&mut cipher[0..plain.len()],&mut tag,&key[0..EAS],&nonce,&aad,&plain);
	print!("Cipher= ");  printbinary(&cipher[0..plain.len()]);
	print!("Tag= ");  printbinary(&tag);

// Mode 1
    mode=1;
    println!("\nMode 1");
	skR=decode_hex("fdf4507c4507ececd2b2232a761ce2cdb50d993b6216e6cb9538a1fc53baed02");
	skE=decode_hex("0dc7bf88c1849fd7800f8bd52eed75f26160a3cb9d634190c48ced1daa4c341a");
	pkR=decode_hex("696977f50e929c99287852ded3fc9b2709f8cd51c0d270c45c338dece959f437");
   
	hpke::encap(config_id,None,&mut skE,&mut z[0..EFS],&mut pkE,&mut pkR);
	print!("pkE= "); printbinary(&pkE);
	print!("Encapsulated z= "); printbinary(&z[0..EFS]);

	hpke::decap(config_id,&mut z[0..EFS],&mut pkE,&mut skR);
	print!("Decapsulated z= "); printbinary(&z[0..EFS]);

	hpke::keyschedule(config_id,&mut key[0..EAS],&mut nonce,mode,&mut pkR,&mut z[0..EFS],&mut pkE,&info,Some(&psk),Some(&pskID),None);
	print!("key= "); printbinary(&key[0..EAS]);
	print!("nonce= "); printbinary(&nonce);

	gcm::encrypt(&mut cipher[0..plain.len()],&mut tag,&key[0..EAS],&nonce,&aad,&plain);
	print!("Cipher= ");  printbinary(&cipher[0..plain.len()]);
	print!("Tag= ");  printbinary(&tag);

// Mode 2
    mode=2;
    println!("\nMode 2");
	skR=decode_hex("a3f26ded71d69e6b7c4924e5756388efce4c8857b89cd00e492cd5a5778c40d7");
    let mut skI=decode_hex("42962c361fe2e92343113407b95abe4eebef311e315726c579fc6fb7f22e16c5");
	skE=decode_hex("372c79a960e1c33c6b6f69c7f4c63567f743a018d5d04fc4a243e0af2aeb3708");
	pkR=decode_hex("39c07ff4a258dfe61a1e9e23ef1d9d39efce89c5326da949a9dfa3b11b9f2f2e");
    let mut pkI=decode_hex("9888840b794623840275e594ec76a526e690b9111d21cdec3fe2faf18227f676");
   
	hpke::authencap(config_id,None,&mut skE,&mut z[0..2*EFS],&mut pkE,&mut pkR,&mut skI);
	print!("pkE= "); printbinary(&pkE);
	print!("Encapsulated z= "); printbinary(&z[0..2*EFS]);

	hpke::authdecap(config_id,&mut z[0..2*EFS],&mut pkE,&mut skR,&mut pkI);
	print!("Decapsulated z= "); printbinary(&z[0..EFS]);

	hpke::keyschedule(config_id,&mut key[0..EAS],&mut nonce,mode,&mut pkR,&mut z[0..2*EFS],&mut pkE,&info,None,None,Some(&mut pkI));
	print!("key= "); printbinary(&key[0..EAS]);
	print!("nonce= "); printbinary(&nonce);

	gcm::encrypt(&mut cipher[0..plain.len()],&mut tag,&key[0..EAS],&nonce,&aad,&plain);
	print!("Cipher= ");  printbinary(&cipher[0..plain.len()]);
	print!("Tag= ");  printbinary(&tag);

// Mode 3
    mode=3;
    println!("\nMode 3");
	skR=decode_hex("2d7c739195ba102216de162f9435991aa3ad42aeefdb7e22391ae34bae7e5a13");
    skI=decode_hex("59c77f5734aef369f30d83c7e30c6bf372e120391cdaf13f34c915030284b75d");
	skE=decode_hex("6827bbf4f7ebd0182a8ebc0ea364c7ddae1d1c8a4f58a903fa9f9f9d4228b126");
	pkR=decode_hex("cc980df06e532bdb6b957f9f5a5caf55c55f46822cdfbd97e76f6ad4c62b322b");
    pkI=decode_hex("db6ee4a53276b7bc90657cdde514f948af83c140540797ec717881490afed921");
   
	hpke::authencap(config_id,None,&mut skE,&mut z[0..2*EFS],&mut pkE,&mut pkR,&mut skI);
	print!("pkE= "); printbinary(&pkE);
	print!("Encapsulated z= "); printbinary(&z[0..2*EFS]);

	hpke::authdecap(config_id,&mut z[0..2*EFS],&mut pkE,&mut skR,&mut pkI);
	print!("Decapsulated z= "); printbinary(&z[0..EFS]);

	hpke::keyschedule(config_id,&mut key[0..EAS],&mut nonce,mode,&mut pkR,&mut z[0..2*EFS],&mut pkE,&info,Some(&psk),Some(&pskID),Some(&mut pkI));
	print!("key= "); printbinary(&key[0..EAS]);
	print!("nonce= "); printbinary(&nonce);

	gcm::encrypt(&mut cipher[0..plain.len()],&mut tag,&key[0..EAS],&nonce,&aad,&plain);
	print!("Cipher= ");  printbinary(&cipher[0..plain.len()]);
	print!("Tag= ");  printbinary(&tag);

}

fn hpke_nist521() {
    //use core::ed25519;
    use core::nist521::hpke;
    use core::nist521::ecdh;
    use core::nist521::ecp;

    const EFS: usize = ecdh::EFS;
    const EGS: usize = ecdh::EGS;
    const EAS: usize = ecp::AESKEY;

	let config_id=0x53;
    let mut pkE: [u8; 2*EFS+1] = [0; 2*EFS+1];
    let mut z: [u8;2*EFS] = [0; 2*EFS];

    let mut key:[u8;32]=[0;32];
    let mut nonce:[u8;12]=[0;12];

	let info=decode_hex("4f6465206f6e2061204772656369616e2055726e");
	let psk=decode_hex("6d656c6c6f6e");
	let pskID=decode_hex("456e6e796e20447572696e206172616e204d6f726961");
	let plain=decode_hex("4265617574792069732074727574682c20747275746820626561757479");
	let aad=decode_hex("436f756e742d30");

    let mut cipher:[u8;32]=[0;32];
    let mut tag:[u8;16]=[0;16];
    println!("\nTesting HPKE code for curve nist521");
// Mode 0
    let mut mode=0;
    println!("\nMode 0");
	let mut skR=decode_hex("011d0d02be6fe4eb49654d5641f0ebb40b594aca84a7c1e76faa1fa0f5f3582502d29fd140d07e71f538c75b93c8ca416af95ad79c5e7e2c0889adce490367492aee");
	let mut skE=decode_hex("0099f6326effd78a5c54eeba93e911f35bdd418f2e1de3281ba2172aebf251ff73614598c81686775c20618721c8c84466f2f6ca7b33636a3edbc69697087f8b414a");
	let mut pkR=decode_hex("04015c0e75e50b266e5fb8e3a3f317608bbf64843c47cca29faa3b894a4f9b95ba533bd40d343e5747f9e85d9ffee5b4ed803a20a23ff0a246ca4d9d59eae9c38291d000c396eade14ed2d42fafe7c4be26da6f07af8b6902af5fbb39f8b0b15c7120d7f57f46ed24e711dda2469f418377d8cc8de7ec8b0594e155b8d6d6ca699f99a4189");
   
	hpke::encap(config_id,None,&mut skE,&mut z[0..EFS],&mut pkE,&mut pkR);
	print!("pkE= "); printbinary(&pkE);
	print!("Encapsulated z= "); printbinary(&z[0..EFS]);

	hpke::decap(config_id,&mut z[0..EFS],&mut pkE,&mut skR);
	print!("Decapsulated z= "); printbinary(&z[0..EFS]);

	hpke::keyschedule(config_id,&mut key[0..EAS],&mut nonce,mode,&mut pkR,&mut z[0..EFS],&mut pkE,&info,None,None,None);
	print!("key= "); printbinary(&key[0..EAS]);
	print!("nonce= "); printbinary(&nonce);

	gcm::encrypt(&mut cipher[0..plain.len()],&mut tag,&key[0..EAS],&nonce,&aad,&plain);
	print!("Cipher= ");  printbinary(&cipher[0..plain.len()]);
	print!("Tag= ");  printbinary(&tag);

// Mode 1
    mode=1;
    println!("\nMode 1");
	skR=decode_hex("00953f712b5c288be50460d0372612f88c3da171196a4cd958b0f658e988aa56ce5edc018e5899acda38649fc51049610bc8d1423189a9b5ff5f7e4fe1081df3c7b3");
	skE=decode_hex("00416c4a6236475f01a0cd0a9a7a0d7229be88a63281fca350ea98d4438de57a23bd5053572bf0f7bed8bc84aba6f7322cabc6b2540e789eb5823660eaad8e206fd9");
	pkR=decode_hex("040108f48ce8fcd71e7e07270fa566cfd66d9b7ab124df141d700b82141264a01306b94265afc3c361c0c0dc514864d4b12e687697d24ddce2f0c77f9c257b8a2f5a380109cb328c87f9833f729967a74675acc31bccdccf5016d442e7b33d2ecaf6f79d8a0e79d8259b07cd173e320ca25c68acf5aa4f3793e7fc6d076baa953060847a02");
   
	hpke::encap(config_id,None,&mut skE,&mut z[0..EFS],&mut pkE,&mut pkR);
	print!("pkE= "); printbinary(&pkE);
	print!("Encapsulated z= "); printbinary(&z[0..EFS]);

	hpke::decap(config_id,&mut z[0..EFS],&mut pkE,&mut skR);
	print!("Decapsulated z= "); printbinary(&z[0..EFS]);

	hpke::keyschedule(config_id,&mut key[0..EAS],&mut nonce,mode,&mut pkR,&mut z[0..EFS],&mut pkE,&info,Some(&psk),Some(&pskID),None);
	print!("key= "); printbinary(&key[0..EAS]);
	print!("nonce= "); printbinary(&nonce);

	gcm::encrypt(&mut cipher[0..plain.len()],&mut tag,&key[0..EAS],&nonce,&aad,&plain);
	print!("Cipher= ");  printbinary(&cipher[0..plain.len()]);
	print!("Tag= ");  printbinary(&tag);

// Mode 2
    mode=2;
    println!("\nMode 2");
	skR=decode_hex("006ae74c6d37982c4a6087500b66948a715ca971e7aa43260bae4683d78818cfc728b0d72d6834f4d401f35db13f932e414b44d03071805fcbd513a57130e18e8323");
    let mut skI=decode_hex("01d9581f65c8cf1a90f1711fe377c15e68f534be11ea5e0158a8adebaa04f0be9c80d0f2517abf0cd117d9ca2073b604743076cee2405f4db2825ace05e0eae83354");
	skE=decode_hex("0195e8805187cf89fc17007d90a75dff2dc9ae824aea70adf001ef539832932f0d8cd7d3bbc94e712fb64d0e5f0acbb7cb79e5bde9d24304c8b4ed0091c8905da986");
	pkR=decode_hex("040054116ccfb36d9cd99e59b100ef9dbc70a6992b38632ee7650659275cdcab37bce7e74f2381cc7292ba418051432c3a8aeb706b3c05fcb886b3a95a306ae9863f5900e9b6db3150e7241fec23c607db539e1a2b2c1898b5d2b78ec1a3254bf022dee6e8c2f6265f71ac8e6003614accb8532dd58d5a07a959bde06b763f2f41a9c3ac32");
    let mut pkI=decode_hex("0400fdb5f8a16b80e8a61c967f4f4f6b83740fc3521cd1654ee7e1cc42f4f645a57e68942303a48d37e09b7cf2be067f03ed0b7076fe0eda129bc65209c83bafbc0b5d012ba9db99b61236f645076e7f8b10c8763517dfcefd07241e90aa6a595209fc6aafc227fd9d870c8c6b8d037dd5386513608f7779887e47db733fe97f74169d21c7");
   
	hpke::authencap(config_id,None,&mut skE,&mut z[0..2*EFS],&mut pkE,&mut pkR,&mut skI);
	print!("pkE= "); printbinary(&pkE);
	print!("Encapsulated z= "); printbinary(&z[0..2*EFS]);

	hpke::authdecap(config_id,&mut z[0..2*EFS],&mut pkE,&mut skR,&mut pkI);
	print!("Decapsulated z= "); printbinary(&z[0..EFS]);

	hpke::keyschedule(config_id,&mut key[0..EAS],&mut nonce,mode,&mut pkR,&mut z[0..2*EFS],&mut pkE,&info,None,None,Some(&mut pkI));
	print!("key= "); printbinary(&key[0..EAS]);
	print!("nonce= "); printbinary(&nonce);

	gcm::encrypt(&mut cipher[0..plain.len()],&mut tag,&key[0..EAS],&nonce,&aad,&plain);
	print!("Cipher= ");  printbinary(&cipher[0..plain.len()]);
	print!("Tag= ");  printbinary(&tag);

// Mode 3
    mode=3;
    println!("\nMode 3");
	skR=decode_hex("010bf0c9af1d5dda4c97151d6a9425c8f590aa7c0adff53c06d23380ae82bbd32f64ddf0c344d221d2f7657711d73de7fa25a75bd8fa75662029e118087276a5ef0b");
    skI=decode_hex("01800af579dbd0c91a09fa0ff3ddf21d9a1447649528d777e00962c748159e6de03880a70331fdab67761e25f78977128f81c7fee9897eea74b5ee80e94414b6aed6");
	skE=decode_hex("013e65c19df676b1d7da923475c72c359fbdd91f224d68785bdf5891bbadfd136a33cc8c31408b9652be389f52e8a19d9401aedaf549a0654f246c277af48f59b2ba");
	pkR=decode_hex("0400a1735a659c6b281e603f170b5e886ccfff35d274b042d43a815447fc80208f599704d63a67355b4d04bfdc367f60961f1b36b1e11bf3ed7599f58948f364a1023501ec6b5acd7ce1cc2c3ec6dba8d72b08e472809623ac2df65dcb658dbd7d5f599c9ac624517d780b49b3d7619f5647be76a56a73fe2c3fc2ae620159cb1b7a437a94");
    pkI=decode_hex("0400f0fe8e86b8d86d02a4dc5db6988f6343067174caff2e2700834b98d25045013eb24cf00c03dd506d562625d0fe5c576910d176705cd4ba1fcd10f5a98f466a0a2200a629f9f62f6053b554bf09b2a547b844f3e040c2b92c548babbc73cf05e77f23d0cffa9e5dfd0a57f9be64bf453ec48cbd00f2e47349572fead07dc73658256331");
   
	hpke::authencap(config_id,None,&mut skE,&mut z[0..2*EFS],&mut pkE,&mut pkR,&mut skI);
	print!("pkE= "); printbinary(&pkE);
	print!("Encapsulated z= "); printbinary(&z[0..2*EFS]);

	hpke::authdecap(config_id,&mut z[0..2*EFS],&mut pkE,&mut skR,&mut pkI);
	print!("Decapsulated z= "); printbinary(&z[0..EFS]);

	hpke::keyschedule(config_id,&mut key[0..EAS],&mut nonce,mode,&mut pkR,&mut z[0..2*EFS],&mut pkE,&info,Some(&psk),Some(&pskID),Some(&mut pkI));
	print!("key= "); printbinary(&key[0..EAS]);
	print!("nonce= "); printbinary(&nonce);

	gcm::encrypt(&mut cipher[0..plain.len()],&mut tag,&key[0..EAS],&nonce,&aad,&plain);
	print!("Cipher= ");  printbinary(&cipher[0..plain.len()]);
	print!("Tag= ");  printbinary(&tag);

}

fn main() {

    hpke_c25519();
    hpke_nist521();
}