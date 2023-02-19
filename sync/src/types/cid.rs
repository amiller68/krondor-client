use anyhow::{Error, Result};
use cid::{
    multihash::{Code, MultihashDigest, Hasher, Sha2_256},
    Cid as _Cid,
};
use ethers::{
    abi::{Token, Tokenizable, InvalidOutputType},
};
use std::{convert::TryFrom, fs::File, io::{Read, Write}};
use serde::{Deserialize, Serialize, Serializer, Deserializer};

#[derive(Debug, Clone)]
pub struct Cid {
    cid: _Cid,
}

impl Serialize for Cid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.cid.to_string())
    }
}

impl<'de> Deserialize<'de> for Cid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let cid = String::deserialize(deserializer)?;
        let _cid = _Cid::try_from(cid).unwrap();
        Ok(Self { cid: _cid })
    }
}

/// Impl TryFrom for Cid for Files
impl TryFrom<File> for Cid {
    type Error = Error;
    // Read the file and create a CID
    fn try_from(file: File) -> Result<Self, Self::Error> {
        let mut hasher = Sha2_256::default();
        let mut buffer = [0; 1024];
        let mut file = file;
        loop {
            let count = file.read(&mut buffer)?;
            if count == 0 {
                break;
            }
            hasher.update(&buffer[..count]);
        }
        let hash = Code::Sha3_256.wrap(&hasher.finalize()).unwrap();
        let cid = _Cid::new_v1(1, hash);
        Ok(Self { cid })
    }
}

impl Tokenizable for Cid {
    fn from_token(token: Token) -> Result<Self, InvalidOutputType> {
        // If the token is a string, convert it to a Cid
        if let Token::String(cid) = token {
            let _cid = _Cid::try_from(cid).unwrap();
            Ok(Self { cid: _cid })
        } else {
            return Err(InvalidOutputType("Token is not a string".to_string()));
        }
    }

    fn into_token(self) -> Token {
        Token::String(self.cid.to_string())
    }
}

impl Cid {
    pub fn to_string(&self) -> String {
        self.cid.to_string()
    }
    pub fn from_str(cid: String) -> Result<Self, Error> {
        let _cid = _Cid::try_from(cid).unwrap();
        Ok(Self { cid: _cid })
    }
}