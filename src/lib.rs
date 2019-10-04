use core::fmt::Debug;
use core::hash::Hash;
use libcommon_rs::peer::PeerId;
use libhash::Hash as LibHash;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait PublicKey: PeerId {}

pub trait SecretKey: PeerId {}

pub trait Signature: Hash + Serialize + DeserializeOwned + Debug + Clone + Send {
    type Hash: LibHash;
    type PublicKey: PublicKey;
    type SecretKey: SecretKey;
    type Error;

    fn sign(hash: Self::Hash, key: Self::SecretKey) -> Result<Self, Self::Error>
    where
        Self: Sized;

    fn verify(&self, hash: Self::Hash, key: Self::PublicKey) -> Result<bool, Self::Error>;

    fn generate_key_pair(&self) -> Result<(Self::PublicKey, Self::SecretKey), Self::Error>;

    fn generate_public_key(
        &self,
        secret_key: Self::SecretKey,
    ) -> Result<Self::PublicKey, Self::Error>;

    fn generate_secret_key(&self) -> Result<Self::SecretKey, Self::Error>;

    // TODO:
    // serde for public key and PeerId
    // store secret keys on file and read from file
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
