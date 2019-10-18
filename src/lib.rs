use core::fmt::Debug;
use core::hash::Hash;
use libcommon_rs::peer::PeerId;
use libhash::Hash as LibHash;
use serde::de::DeserializeOwned;
use serde::Serialize;
extern crate failure;

// Any PublicKey implementation must implement serde::Deserialise
pub trait PublicKey: PeerId + DeserializeOwned {}

// Any SecretKey implementation must implement serde::Deserialise
pub trait SecretKey: PeerId + DeserializeOwned {}

// Any Signature implementation must implement serde::Deserialise
pub trait Signature: Hash + Serialize + DeserializeOwned + Debug + Clone + Send {
    type Hash: LibHash;
    type PublicKey: PublicKey;
    type SecretKey: SecretKey;
    type Error: failure::Fail;

    fn sign(hash: Self::Hash, key: Self::SecretKey) -> Result<Self, Self::Error>
    where
        Self: Sized;

    fn verify(&self, hash: Self::Hash, key: Self::PublicKey) -> Result<bool, Self::Error>;

    fn generate_key_pair() -> Result<(Self::PublicKey, Self::SecretKey), Self::Error>;

    fn generate_public_key(secret_key: Self::SecretKey) -> Result<Self::PublicKey, Self::Error>;

    fn generate_secret_key() -> Result<Self::SecretKey, Self::Error>;

    // TODO:
    // store secret keys on file and read from file
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
