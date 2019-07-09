extern crate crypto;
extern crate rand;

use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
use crypto::{aes, blockmodes, buffer, symmetriccipher};
use rand::RngCore;
use rand::rngs::OsRng;
use std::str;
use std::io::{stdin, Write, Read};
use std::fs::File;

fn encrypt(data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut encryptor = aes::cbc_encryptor(aes::KeySize::KeySize256, key, iv, blockmodes::PkcsPadding);
    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);
    loop {
        let result = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true)?;
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }
    Ok(final_result)
}

fn decrypt(encrypted_data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut decryptor = aes::cbc_decryptor(aes::KeySize::KeySize256, key, iv, blockmodes::PkcsPadding);
    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);
    loop {
        let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true)?;
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }
    Ok(final_result)
}

fn en() {
    let mut message = String::new();
    stdin().read_line(&mut message).unwrap();
    let mut key: [u8; 32] = [0; 32];
    let mut iv: [u8; 16] = [0; 16];
    let mut rng = OsRng::new().ok().unwrap();
    rng.fill_bytes(&mut key);
    rng.fill_bytes(&mut iv);
    let encrypted_data = encrypt(message.as_bytes(), &key, &iv).ok().unwrap();

    File::create("key").unwrap().write(&key).unwrap();
    File::create("iv").unwrap().write(&iv).unwrap();
    File::create("encrypt").unwrap().write(&encrypted_data).unwrap();
}

fn de() {
    let mut key: [u8; 32] = [0; 32];
    File::open("key").unwrap().read(&mut key).unwrap();

    let mut iv: [u8; 16] = [0; 16];
    File::open("iv").unwrap().read(&mut iv).unwrap();

    let mut encrypt = Vec::new();
    File::open("encrypt").unwrap().read_to_end(&mut encrypt).unwrap();

    let decrypted_data = decrypt(&encrypt, &key, &iv).ok().unwrap();
    let the_string = str::from_utf8(&decrypted_data).expect("not UTF-8");
    println!("decrypt: {}", the_string);
}

fn main() {
    loop {
        println!("type 1 for encrypt and 2 decrypt");
        let mut action = String::new();
        stdin().read_line(&mut action).unwrap();
        match action.trim().as_ref() {
            "1" => en(),
            "2" => de(),
            &_ => ()
        }
    }
}