use core::num;
use std::{fs::read, io::Read};

use serde::Serialize;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default, Serialize)]
struct Tx{
    version: u32,
    inputs: Vec<TxIn>,
    outputs: Vec<TxOut>,
    locktime: u32,
    segwit: bool,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default, Serialize)]
struct TxIn{
    prev_tx_id: String,
    prev_tx_index: u32,
    script_sig: String,
    sequence: u32
}


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default, Serialize)]
struct TxOut {
    amount: u64,
    script_pub_key: String
}

fn read_u32(bytes: &mut &[u8]) -> u32 {
    let mut buffer = [0; 4];
    bytes.read(&mut buffer).unwrap();
    u32::from_le_bytes(buffer)
}

fn read_txid(bytes: &mut &[u8]) -> String {
    let mut buffer = [0; 32];
    bytes.read(&mut buffer).unwrap();
    buffer.reverse();
    hex::encode(buffer)
}

fn read_script_sig(bytes: &mut &[u8]) -> String {
    let length = read_var_int(bytes) as usize;
    let mut buffer = vec![0_u8; length];
    bytes.read(&mut buffer).unwrap();
    hex::encode(buffer)

}

fn read_amount(bytes: &mut &[u8]) -> u64 {
    let mut buffer = [0; 8];
    bytes.read(&mut buffer).unwrap();
    u64::from_le_bytes(buffer)
}

fn read_script_pub_key(bytes: &mut &[u8]) -> String{
    let length = read_var_int(bytes) as usize;
    let mut buffer = vec![0_u8; length];
    bytes.read(&mut buffer).unwrap();
    hex::encode(buffer)

}



fn read_var_int(bytes: &mut &[u8]) -> u64{ 
    let mut var_int = [0; 1]; 
    bytes.read(&mut var_int).unwrap();

    match var_int[0]{
        1..=252 =>{
            u8::from_le_bytes(var_int) as u64
        }
        253 => {
            let mut buffer = [0; 2];
            bytes.read(&mut buffer).unwrap();
            u16::from_le_bytes(buffer) as u64
        }
        254 => {
            let mut buffer = [0; 4];
            bytes.read(&mut buffer).unwrap();
            u32::from_le_bytes(buffer) as u64
        }
        255 => {
            let mut buffer = [0; 8];
            bytes.read(&mut buffer).unwrap();
            u64::from_le_bytes(buffer)
        }   
        _ => {
            panic!("invalid varint")
        }

        
    }
}


fn main() {
    let transaction_hex="0200000002a796e34c7d4a53fa3e29e78208b66042f27c66b1bb8851d736e45af956fa00b6010000006a473044022057fe8be21a82339f9be87df543f684fc885017e260f6db764effad7dbe9951b00220678fd5a19620ed29da875dd478918e7669c32509c8bd25b3f3ff1e02e0999c82012103786af4b32017ec640dba2d2a7e1fd5aa4a231a658e4cbc114d51c031576e19bcfdffffff7dbf68b813839f6cd6550b3e7847d43440ed8dca97f7d27fb8ed9af6a03ae3ee010000006a473044022066174c023bdcdc7b9e397ee0cfe1c3b1ac4c61634818ab7d20b09884eb573c2302204bb0a720d33cfd805286c93a27e5d62e7605e721338d3e1946c4fc294a2512be012103572486d3dfd3b1157df1e4ef7bbd516cd5c8beea262f49d636c7dad85ef2af63fdffffff02014f1727000000001976a9145351ad35bfd3e97c037df39f5fe07cb071892d2e88acb65e01741c0000001976a914cebb2851a9c7cfe2582c12ecaf7f3ff4383d1dc088ac00000000";
    let transaction_bytes = hex::decode(transaction_hex).unwrap();
    let mut bytes_slice = transaction_bytes.as_slice();
    let version = read_u32(&mut bytes_slice);
    let num_inputs = read_var_int(&mut bytes_slice);

    let mut inputs = vec![];
    for _ in 0..num_inputs {
        let tx_id = read_txid(&mut bytes_slice);
        let index = read_u32(&mut bytes_slice);
        let script_sig = read_script_sig(&mut bytes_slice);
        let sequence = read_u32(&mut bytes_slice);
       
        let tx_in = TxIn{
            prev_tx_id: tx_id,
            prev_tx_index: index,
            script_sig: script_sig,
            sequence: sequence,

        };
        inputs.push(tx_in);

        }

    let num_outputs = read_var_int(&mut bytes_slice);
    
    let mut outputs = vec![];
    for _ in 0..num_outputs {
        let amount = read_amount(&mut bytes_slice);
        let script_pub_key = read_script_pub_key(&mut bytes_slice);

        let tx_out = TxOut {
            amount,
            script_pub_key
        };
        outputs.push(tx_out)
;    }    

    let locktime = read_u32(&mut bytes_slice);

    let transaction = Tx{
        version,
        inputs,
        outputs,
        locktime, 
        segwit: false
    };

    let json_transaction = serde_json::to_string_pretty(&transaction).unwrap();
    println!("{}", json_transaction)

    
}


#[cfg(test)]
mod unit_tests {

    use crate::read_var_int;


    #[test]
    fn test_reading_compact_size() {
        let mut bytes = [1_u8].as_slice();
        let length = read_var_int(&mut bytes);
        assert_eq!(length, 1_u64);

        let mut bytes = [253_u8, 0, 1].as_slice();
        let length = read_var_int(&mut bytes);
        assert_eq!(length, 256_u64);

        let mut bytes = [253_u8, 253, 0].as_slice();
        let length = read_var_int(&mut bytes);
        assert_eq!(length, 253);

        let mut bytes = [253, 255, 255].as_slice();
        let length = read_var_int(&mut bytes);
        assert_eq!(length, 0xffff);

        let mut bytes = [254, 255, 255, 255, 255].as_slice();
        let length = read_var_int(&mut bytes);
        assert_eq!(length, 0xffffffff);

        let mut bytes = [255_u8, 0, 0, 0, 0, 0, 0, 0, 1].as_slice();
        let length = read_var_int(&mut bytes);
        assert_eq!(length, 256_u64.pow(7));

        let mut bytes = [0xfd, 0x20, 0x4e].as_slice();
        let length = read_var_int(&mut bytes);
        assert_eq!(length, 20_000);



    }
}