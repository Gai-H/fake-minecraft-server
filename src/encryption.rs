use std::error;
use std::fmt;

type OpenSslRsa = openssl::rsa::Rsa<openssl::pkey::Private>;
type Result<T> = std::result::Result<T, Box<EncryptionError>>;

#[derive(Debug, Clone)]
pub struct Rsa {
    rsa: OpenSslRsa,
}

impl Rsa {
    pub fn new() -> Result<Rsa> {
        let rsa = Self::generate_key_pair()?;

        Ok(Rsa { rsa })
    }

    fn generate_key_pair() -> Result<OpenSslRsa> {
        let rsa = match OpenSslRsa::generate(1024) {
            Ok(r) => r,
            Err(e) => {
                return Err(
                    EncryptionError::new(format!("Failed to generate RSA key pair: {}", e)).into(),
                )
            }
        };
        Ok(rsa)
    }

    pub fn get_public_key_in_der(&self) -> Result<Vec<u8>> {
        let pkey_in_der = match self.rsa.public_key_to_der() {
            Ok(p) => p,
            Err(e) => {
                return Err(EncryptionError::new(format!(
                    "Failed to encode RSA public key to DER: {}",
                    e
                ))
                .into())
            }
        };
        Ok(pkey_in_der)
    }

    pub fn decrypt_bytes(&self, from: &[u8]) -> Result<Vec<u8>> {
        let mut to: Vec<u8> = vec![0; 128];
        if let Err(e) = self
            .rsa
            .private_decrypt(from, &mut to, openssl::rsa::Padding::PKCS1)
        {
            return Err(
                EncryptionError::new(format!("Could not decrypt byte array: {}", e)).into(),
            );
        }
        Ok(to)
    }
}
pub fn generate_verify_token() -> Result<[u8; 4]> {
    let mut verify_token_array: [u8; 4] = [0; 4];
    match openssl::rand::rand_bytes(&mut verify_token_array) {
        Ok(_) => {}
        Err(e) => {
            return Err(
                EncryptionError::new(format!("Failed to generate verify token: {}", e)).into(),
            )
        }
    };
    Ok(verify_token_array)
}

#[derive(Debug)]
pub struct EncryptionError {
    reason: String,
}

impl EncryptionError {
    fn new(reason: String) -> EncryptionError {
        EncryptionError { reason }
    }
}

impl fmt::Display for EncryptionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Encryption error: {}", self.reason)
    }
}

impl error::Error for EncryptionError {}
