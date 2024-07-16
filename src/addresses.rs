#![allow(unused)]

use ripemd160::{Ripemd160, Digest as RipDigest};
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use secp256k1::rand::rngs::OsRng;
use sha2::{Sha256, Digest};
use bs58;

#[derive(Debug, Clone)]
pub struct Address(String);

impl Address {
    pub fn address(&self) -> String {
        self.0.clone()
    }
    pub fn new() -> Self {
        Address("".to_string())
    }

    pub fn generate_private_key(&mut self) {
        let secp = Secp256k1::new();
        // generate a private key
        let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);

        println!("Private key: {:?} and \n Equivalent Public key, {:?}", secret_key, public_key);
        let serialized_public_key = public_key.serialize_uncompressed();
        let address = Address::perform_public_key_hash(public_key.serialize_uncompressed());
        self.0 = bs58::encode(address).into_string();
    }

    fn perform_public_key_hash(value: [u8; 65]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(&value[1..]);
        let sha256_result = hasher.finalize();

        // perform the ripemd-160 hashing on SHA-256 result
        let mut ripemd160_hasher = Ripemd160::new();
        ripemd160_hasher.update(&sha256_result);
        let ripemd160_result = ripemd160_hasher.finalize();

        let mut address_bytes = vec![0x00];
        address_bytes.extend(&ripemd160_result);
        println!("Ripemd extended address byte: {:?}", address_bytes);

        let mut hasher = Sha256::new();
        hasher.update(&address_bytes);
        let sha256_result = hasher.finalize();

        let mut hasher = Sha256::new();
        hasher.update(&sha256_result);
        let sha256_result = hasher.finalize();

        // take the first 4 bytes as checksum
        let checksum = &sha256_result[0..4];
        address_bytes.extend(checksum);
        println!("Address bytes: {:?}", address_bytes);
        address_bytes
    }

    fn to_script_pubkey(address: Address) -> Vec<u8> {
        let decoded = bs58::decode(address.0).into_vec().unwrap();
        let mut script = vec![0x76, 0xa9, 0x14];
        script.extend(&decoded[1..21]);
        script.extend(vec![0x88, 0xac]);

        script
    }
}

