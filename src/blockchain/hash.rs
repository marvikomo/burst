use sha2::{Digest, Sha256};
use borsh::{BorshSerialize, BorshDeserialize};

pub struct CryptoHash(pub [u8; 32]);

#[derive(
borsh::BorshDeserialize,
borsh::BorshSerialize,
)]
impl CryptoHash {

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



