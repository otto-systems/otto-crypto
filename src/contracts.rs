use crate::{CryptoError, DecryptCommand, DeriveKeyCommand, EncryptCommand, SignCommand, VerifyCommand};

pub trait EncryptionContract {
    fn encrypt(&self, cmd: EncryptCommand) -> Result<Vec<u8>, CryptoError>;
    fn decrypt(&self, cmd: DecryptCommand) -> Result<Vec<u8>, CryptoError>;
}

pub trait SigningContract {
    fn sign(&self, cmd: SignCommand) -> Result<Vec<u8>, CryptoError>;
    fn verify(&self, cmd: VerifyCommand) -> Result<bool, CryptoError>;
}

pub trait KeyDerivationContract {
    fn derive_key(&self, cmd: DeriveKeyCommand) -> Result<String, CryptoError>;
    fn zeroize_key(&self, key_handle: &str) -> Result<(), CryptoError>;
}
