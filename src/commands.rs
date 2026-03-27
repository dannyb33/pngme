use std::fs;

use clap::{Subcommand};

use crate::{args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs}, chunk::Chunk, png::Png};

#[derive(Subcommand)]
pub enum Commands {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs)
}

pub fn encode(args: &EncodeArgs) {
    let bytes = fs::read(&args.file_path).unwrap();

    let mut png = Png::try_from(bytes.as_ref()).unwrap();

    let chunk = Chunk::chunk_from_strings(&args.chunk_type, &args.message).unwrap();

    png.append_chunk(chunk);

    let _ = fs::write(&args.file_path, png.as_bytes());
}

pub fn decode(args: &DecodeArgs) {
    let bytes = fs::read(&args.file_path).unwrap();

    let png = Png::try_from(bytes.as_ref()).unwrap();

    let chunk = png.chunk_by_type(&args.chunk_type);

    let out_string: String = match chunk {
        Some(c) => c.data_as_string().unwrap(),
        None => String::from("not found")
    };

    println!("Your message is {}", out_string);
}

pub fn remove(args: &RemoveArgs) {
    let bytes = fs::read(&args.file_path).unwrap();

    let mut png = Png::try_from(bytes.as_ref()).unwrap();

    let chunk = png.remove_first_chunk(&args.chunk_type);

    println!("Deleted {}", chunk.unwrap());

    let _ = fs::write(&args.file_path, png.as_bytes());

}

pub fn print(args: &PrintArgs) {
    let bytes = fs::read(&args.file_path).unwrap();
    
    let png = Png::try_from(bytes.as_ref()).unwrap();

    println!("{}", png);
}