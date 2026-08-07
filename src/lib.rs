pub mod commands;
pub mod contracts;

use thiserror::Error;

pub type KeyHandle = String;
pub type Bytes = Vec<u8>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncryptCommand {
    pub plaintext: Bytes,
    pub key_handle: KeyHandle,
    pub aad: Option<Bytes>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecryptCommand {
    pub ciphertext: Bytes,
    pub key_handle: KeyHandle,
    pub aad: Option<Bytes>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignCommand {
    pub message: Bytes,
    pub signing_key_handle: KeyHandle,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifyCommand {
    pub message: Bytes,
    pub signature: Bytes,
    pub verify_key_handle: KeyHandle,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeriveKeyCommand {
    pub master_key_handle: KeyHandle,
    pub salt: Bytes,
    pub info: Bytes,
    pub algorithm: KdfAlgorithm,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KdfAlgorithm {
    HkdfSha256,
    Pbkdf2Sha256,
    Argon2id,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CryptoError {
    #[error("key handle not found")]
    KeyNotFound,
    #[error("authentication tag mismatch")]
    AuthTagMismatch,
    #[error("malformed ciphertext")]
    MalformedCiphertext,
    #[error("invalid input")]
    InvalidInput,
    #[error("operation failed")]
    OperationFailed,
}
