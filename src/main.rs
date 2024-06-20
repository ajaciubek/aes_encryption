use aes::cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit};
use aes::Aes128;
use aes_encryption::{self, BLOCK_LEN};
use clap::{ArgAction, Parser};
use std::io::BufReader;
use std::io::Read;
use std::{fs, io::Write};

#[derive(Debug, Parser)]
#[command(author, about, long_about = None)]
struct Args {
    #[arg(short, long, action=ArgAction::SetTrue)]
    decrypt: bool,
    #[arg(short, long)]
    key: String,
    #[arg(short, long)]
    input: String,
    #[arg(short, long)]
    output: String,
}

fn encrypt(path: String, aes: &Aes128, output: String) {
    let message = fs::read_to_string(path).unwrap();
    let data = aes_encryption::read_string(message);
    let mut file = fs::File::create(output).unwrap();
    for chunk in data {
        let mut block = GenericArray::from(chunk);
        aes.encrypt_block(&mut block);
        let _ = file.write_all(block.as_slice());
    }
}

fn decrypt(path: String, aes: &Aes128, output: String) {
    let my_buf = BufReader::new(fs::File::open(path).unwrap());
    let mut buffer = [0u8; aes_encryption::BLOCK_LEN];
    let mut str_buffer = String::new();
    for (i, byte_or_error) in my_buf.bytes().enumerate() {
        buffer[i % aes_encryption::BLOCK_LEN] = byte_or_error.unwrap();
        if (i != 0) & (i % aes_encryption::BLOCK_LEN == 15) {
            let mut block = GenericArray::from(buffer);
            aes.decrypt_block(&mut block);
            let slice = block.as_slice();
            let mut end: usize = BLOCK_LEN;
            if let Some(pos) = slice.iter().position(|&x| x == 0) {
                end = pos;
            }
            let s = match std::str::from_utf8(&slice[0..end]) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };
            str_buffer.push_str(s);
        }
    }
    let _ = fs::write(output, str_buffer);
}

fn main() {
    let args = Args::parse();
    if args.key.len() != aes_encryption::BLOCK_LEN {
        panic!("Wrong key size")
    }
    let key = GenericArray::from(aes_encryption::read_string(args.key)[0]);
    let cipher = Aes128::new(&key);

    if !args.decrypt {
        encrypt(args.input, &cipher, args.output);
    } else {
        decrypt(args.input, &cipher, args.output);
    }
}
