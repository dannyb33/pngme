use std::fs;

use clap::{Subcommand};

use crate::{args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs}, chunk::Chunk, png::Png, Result};

#[derive(Subcommand)]
pub enum Commands {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs)
}

pub fn encode(args: &EncodeArgs) -> Result<()> {
    let bytes = fs::read(&args.file_path)?;

    let mut png = Png::try_from(bytes.as_ref())?;

    let chunk = Chunk::chunk_from_strings(&args.chunk_type, &args.message)?;

    png.append_chunk(chunk);

    let _ = fs::write(&args.file_path, png.as_bytes())?;
    Ok(())
}

pub fn decode(args: &DecodeArgs) -> Result<()> {
    let bytes = fs::read(&args.file_path)?;

    let png = Png::try_from(bytes.as_ref())?;

    let chunk = png.chunk_by_type(&args.chunk_type);

    let out_string: String = match chunk {
        Some(c) => c.data_as_string()?,
        None => String::from("not found")
    };

    println!("Your message is {}", out_string);
    Ok(())
}

pub fn remove(args: &RemoveArgs) -> Result<()> {
    let bytes = fs::read(&args.file_path)?;

    let mut png = Png::try_from(bytes.as_ref())?;

    let chunk = png.remove_first_chunk(&args.chunk_type)?;

    println!("Deleted {}", chunk);

    let _ = fs::write(&args.file_path, png.as_bytes())?;
    Ok(())
}

pub fn print(args: &PrintArgs) -> Result<()> {
    let bytes = fs::read(&args.file_path)?;
    
    let png = Png::try_from(bytes.as_ref())?;

    println!("{}", png);
    Ok(())
}