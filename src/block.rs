#![allow(unused)]
use crate::linked_list::LinkedList as List;
use rand::Rng;
use secp256k1::Secp256k1;

use crate::addresses::Address;

type Amount = u64;

struct BlockChain {
    blocks: List<Block>
}

struct Block {
    hash: String,
    id: u128,
    transactions: List<Transaction>,
}


struct TxIn {
    prev_txid: u64,
    out: usize,
    signature: String, // to spend the output
}

struct TxOut {
    public_address: Vec<Address>,
    satoshis: Vec<Amount>, 
    // 1 btc = 10^8 satoshis, in total 10^8 * 21 * 10^6 = 2.1 * 10^15
    // maximum value of u64 is greater than 10^19
    // so u64 is enough to store all valid satoshis
}

#[derive(Clone)]
struct TxId(u64, u32);

impl TxId {
    fn new() -> Self {
        let random_nonce = rand::thread_rng().gen::<u32>();
        TxId(0, random_nonce)
    }

    pub fn get_id(&self) -> u64 {
        self.0.wrapping_add(self.1.into())
    }

    fn use_id(&mut self) -> TxId {
        // uses the current ID and updates the value of the txId
        let prev_id = self.clone();
        self.0 += 1;
        self.1 = rand::thread_rng().gen::<u32>();
        prev_id
    }
}

// Try to include bitcoin related functionalities like serialization, computing addresses etc.,
// You can add your own methods for different types and associated unit tests
struct Transaction {
    inputs: List<TxIn>,
    outputs: List<TxOut>,
    txid: u64,
}

impl Transaction {
    fn new(outputs: Vec<(Address, Amount)>, inputs: List<TxIn>) -> Self {
        let mut output_list = List::new();
        output_list.push(TxOut {
            public_address: outputs.iter().map(|(address, _)| address.clone() ).collect(),
            satoshis: outputs.iter().map(|&(_, values)| values).collect(),
        });

        Transaction {
            inputs,
            outputs: output_list,
            txid: (TxId::new()).get_id(),
        }
    }

    fn create_transaction(&mut self, txin: TxIn) {
        let secp = Secp256k1::new();
        let mut tx: Vec<u8> = Vec::new();
        // get the transaction version and append to transaction
        tx = self.get_tx_version();
        tx.push(self.inputs.len() as u8); // length of txIn

        // Add each input utxo
        for data in self.inputs.iter() {
            tx.extend(data.prev_txid.to_ne_bytes());   // Push the previous txHash
            tx.extend(data.out.to_le_bytes());  // Push the output index

            tx.push(0x00);   /// Placeholder for signature script
            tx.extend(&0xFFFFFFFFu32.to_le_bytes());
        }

        tx.push(self.outputs.len() as u8);
        let output_value = self.outputs.iter().fold(0, |acc, output| acc + output.satoshis.iter().sum::<Amount>());
        tx.extend(&output_value.to_le_bytes());

        // Add the  recipient address(es)
        let mut output_address: Vec<Address> = Vec::new();
        for address in self.outputs.iter().map(|data| data.public_address) {
            output_address = address.iter().map(|ad| ad).collect();
        }
        // for address in output_address.collect() {
        //     let recipient_script = Address::address_to_scriptpubkey(address);
        // }
    }

    fn get_tx_version(&self) -> Vec<u8> {
        vec![0x02, 0x00, 0x00, 0x00]
    }
}

fn address_to_scriptpubkey(address: &Address) -> Vec<u8> {
    let decoded = base58::decode(address)
}