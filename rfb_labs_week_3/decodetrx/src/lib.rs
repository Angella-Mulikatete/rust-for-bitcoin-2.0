use std::io::{Read, Error};
// use clap::{Parser, Subcommand};
use clap::{Arg, Command};
use std::fmt;
use sha2::{Sha256, Sha512, Digest}; // https://docs.rs/sha2/latest/sha2/
use transaction::{Amount, Input, Output, Transaction, Txid};
mod transaction;

// #[derive(Parser)]
// #[command(name= " Transaction decoder")]
// #[command(version= "1.0")]
// #[command(about= "Bitcoin Transaction decoder", long_about=None)]
// struct CLI {
//       #[arg(
//             required = true,
//             help="(string, required) Row Transaction hex"
//         )]
//     transaction_hex: String
// }


fn take_bytes<'a>(bytes: &mut &'a [u8], n: usize) -> Result<&'a [u8], Error> {
    if bytes.len() < n {
        return Err(Error::new(
            std::io::ErrorKind::UnexpectedEof,
            "not enough bytes remaining in transaction",
        ));
    }
    let (front, rest) = bytes.split_at(n);
    *bytes = rest;
    Ok(front)
}


#[allow(unused_variables)]
fn read_version(transaction_hex: &str) -> u32 {
 
}

fn read_u64(transaction_bytes: &mut &[u8]) -> Result<u64, Error> {
    let bytes = take_bytes(transaction_bytes, 8)?;
    Ok(u64::from_le_bytes(bytes.try_into().unwrap()))
}

fn read_amount(transaction_bytes: &mut &[u8]) -> Result<Amount, Error> {
    let satoshis = read_u64(transaction_bytes)?;
    Ok(Amount::from_sat(satoshis))
}




fn read_u32(bytes_slice: &mut &[u8]) -> Result<u32, Error> {
    let bytes = take_bytes(bytes_slice, 4)?;
    Ok(u32::from_le_bytes(bytes.try_into().unwrap()))
}
  


fn read_compact_size(transaction_bytes: &mut &[u8]) -> Result<u64, Error> {
    let prefix = take_bytes(transaction_bytes, 1)?[0];

    match prefix {
        0x00..=0xfc => Ok(prefix as u64),
        0xfd => {
            let bytes = take_bytes(transaction_bytes, 2)?;
            Ok(u16::from_le_bytes(bytes.try_into().unwrap()) as u64)
        }
        0xfe => {
            let bytes = take_bytes(transaction_bytes, 4)?;
            Ok(u32::from_le_bytes(bytes.try_into().unwrap()) as u64)
        }
        _ => {
            let bytes = take_bytes(transaction_bytes, 8)?;
            Ok(u64::from_le_bytes(bytes.try_into().unwrap()))
        }
    }
}


fn read_txid(transaction_bytes: &mut &[u8]) -> Result<Txid, Error> {
    let bytes = take_bytes(transaction_bytes, 32)?;
    let array: [u8; 32] = bytes.try_into().unwrap();
    Ok(Txid::from_bytes(array))
}



fn read_script_size(transaction_bytes: &mut &[u8]) -> Result<String, Error> {
    let length = read_compact_size(transaction_bytes)? as usize;
    let script_bytes = take_bytes(transaction_bytes, length)?;
    Ok(hex::encode(script_bytes))
}


fn read_version_byte(transaction_bytes: &mut &[u8]) -> Result<u32, Error> {

}
// Bitcoin uses little-endian encoding for most of its numeric fields, meaning the least significant byte comes first.

fn hash_row_transaction(row_transaction_bytes: &[u8]) -> Result<Txid, Error> {
    let first_hash = Sha256::digest(row_transaction_bytes);
    let second_hash = Sha256::digest(first_hash);
    let bytes: [u8; 32] = second_hash.into();
    Ok(Txid::from_bytes(bytes))
}



pub fn decode_transaction(transaction_hex: String) -> Result<String, Box<dyn std::error::Error>> {
    

}