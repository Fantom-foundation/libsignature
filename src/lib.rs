extern crate rand;

use libcommon_rs::peer::PeerId;
use minisign_verify::{PublicKey, Signature};
use rand::prelude::*;

pub fn sign(&self, hash: String) -> Result<Signature> {
    Signature::decode(hash).expect("Unable to decode the signature");
}

pub fn verify(&self, hash: String, signature: Signature, public_key: PublicKey) -> Result<Bool> {
    public_key.verify(hash, &signature).expect("Signature didn't verify");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
