use core::fmt::Debug;
use core::hash::Hash;
use libcommon_rs::peer::PeerId;
use libhash::Hash as LibHash;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait SecretKey: PeerId {}

pub trait PublicKey: PeerId {}

pub trait Signature: Hash + Serialize + DeserializeOwned + Debug + Clone + Send {
    type Hash: LibHash;
    type PublicKey: PublicKey;
    type SecretKey: SecretKey;
    type Error;

    fn sign(hash: Self::Hash, key: Self::SecretKey) -> Result<Self, Self::Error>
    where
        Self: Sized;

    fn verify(&self, hash: Self::Hash, key: Self::PublicKey) -> Result<bool, Self::Error>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
