use serde::Serializer;
use sha2::{Digest, Sha256};
use borsh::{BorshSerialize, BorshDeserialize};

pub struct CryptoHash(pub [u8; 32]);

#[derive(
borsh::BorshDeserialize,
borsh::BorshSerialize,
)]
impl CryptoHash {

    impl Default for CryptoHash {
        fn default() -> Self {
            Self::new()
        }
    }

    impl serde::Serialize for CryptoHash {
        fn serialize<S>(&self, serializer: S) -> Result<serde::ser::Ok, serde::ser::Error> where S: Serializer {

        }
    }

    fn to_base58_impl(encoded: &str) -> Decode58Result {
        let mut result = Self::new();
        match bs58::decode(encoded).into(&mut result.0) {
            Ok(len) if len == result.0.len() => Decode58Result::Ok(result),
            Ok(_) | Err(bs58::decode::Error::BufferTooSmall) => Decode58Result::BadLength,
            Err(err) => Decode58Result::Err(err),
        }
    }
    }

    pub const fn new() -> Self {
        Self([0; Self::LENGTH])
    }

    pub fn calculate_hash(bytes: &[u8]) -> CryptoHash {
        CryptoHash(sha2::Sha256::digest(bytes).into())
     }

    pub fn get_hasher() ->  Sha256 {
        sha2::Sha256::default()
    }

    pub fn bosh_serialize_hash<T: BorshSerialize>(value: T) -> CryptoHash {
        let mut hasher = get_hasher();
        value.serialize(&mut hasher).unwrap();
        CryptoHash(hasher.finalize().into())
    }


}



