use clap::{Args};

#[derive(Args)]
pub struct EncodeArgs {
    pub file_path: String,
    pub chunk_type: String,
    pub message: String
}

#[derive(Args)]
pub struct DecodeArgs {
    pub file_path: String,
    pub chunk_type: String
}

#[derive(Args)]
pub struct RemoveArgs {
    pub file_path: String,
    pub chunk_type: String
}

#[derive(Args)]
pub struct PrintArgs {
    pub file_path: String,
}