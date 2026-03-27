use std::fmt::{Display, Formatter};
use std::string::FromUtf8Error;
use std::u32;
use crate::chunk_type::ChunkType;
use crc::Crc;

pub struct Chunk {
    chunk_length: [u8; 4],
    chunk_type: crate::chunk_type::ChunkType,
    chunk_data: Vec<u8>,
    crc: [u8; 4]
}

impl TryFrom<&[u8]> for Chunk {
    type Error = &'static str;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let chunk_length: [u8; 4] = match value[0..4].try_into() {
            Ok(arr) => arr,
            Err(_) => return Err("Invalid chunk length")
        };

        let type_bytes: [u8; 4] = match value[4..8].try_into() {
            Ok(c) => c,
            Err(_) => return Err("Invalid chunk type bytes")
        };

        let chunk_type: ChunkType = match ChunkType::try_from(type_bytes) {
            Ok(c) => c,
            Err(_) => return Err("Invalid chunk type")
        };

        let length_int: u32 = u32::from_be_bytes(chunk_length);
        let length = length_int as usize;

        let mut data_vector: Vec<u8> = vec![];

        let mut cursor = 8;

        while cursor < length + 8 {
            data_vector.push(value[cursor]);

            cursor += 1;
        }

        let crc = Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);

        let checksum = crc.checksum(&value[4..cursor]);

        let crc_bytes: [u8; 4] = checksum.to_be_bytes();

        if crc_bytes != value[cursor..cursor+4] {
            return Err("Invalid crc");
        }

        Ok(Chunk {
            chunk_length,
            chunk_type,
            chunk_data: data_vector,
            crc: crc_bytes,
        })
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {:?}, {})",
            u32::from_be_bytes(self.chunk_length),
            self.chunk_type,
            self.chunk_data,
            u32::from_be_bytes(self.crc)
        )
    }
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let data_len:[u8; 4] = (data.len() as u32).to_be_bytes();

        let crc = Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);

        let mut digest = crc.digest();

        digest.update(&chunk_type.bytes());
        digest.update(&data);

        Chunk {
            chunk_length: data_len,
            chunk_type,
            chunk_data: data,
            crc: digest.finalize().to_be_bytes()
        }
    }

    pub fn chunk_from_strings(chunk_type: &str, data: &str) -> Result<Chunk, &'static str> {
        let type_bytes: [u8; 4] = match chunk_type.as_bytes().try_into() {
            Ok(d) => d,
            Err(_) => return Err("Invalid type")
        };

        let ct = ChunkType::try_from(type_bytes).unwrap();

        let v = data.as_bytes().to_vec();

        Ok(Chunk::new(ct, v))
    }

    pub fn length(&self) -> u32 {
        u32::from_be_bytes(self.chunk_length)
    }

    pub fn crc(&self) -> u32 {
        u32::from_be_bytes(self.crc)
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut out_vec: Vec<u8> = vec![];

        out_vec.extend_from_slice(&self.length().to_be_bytes());
        out_vec.extend_from_slice(&self.chunk_type().bytes());
        out_vec.extend_from_slice(&self.chunk_data.as_slice());
        out_vec.extend_from_slice(&self.crc().to_be_bytes());

        out_vec
    }

    pub fn data_as_string(&self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.chunk_data.clone())
    }
}

#[cfg(test)]
mod chunk_tests {
    use super::*;
    use std::str::FromStr;
    use crate::chunk_type::ChunkType;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!".as_bytes().to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        println!("{}", chunk_string);
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }
    
    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;
    
        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
    
        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();
    
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
    
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }
    
    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;
    
        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
    
        let chunk = Chunk::try_from(chunk_data.as_ref());
    
        assert!(chunk.is_err());
    }
    
    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;
    
        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
    
        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
    
        let _chunk_string = format!("{}", chunk);

        print!("{}", _chunk_string)
    }
}