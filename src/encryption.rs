use std::error;
use std::fmt;

type OpenSslRsa = openssl::rsa::Rsa<openssl::pkey::Private>;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

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

fn get_hex_digest(bytes: &[u8; 20]) -> String {
    num_bigint::BigInt::from_signed_bytes_be(bytes).to_str_radix(16)
}

pub fn authenticate(
    shared_secret: &Vec<u8>,
    pkey_in_der: &Vec<u8>,
    username: &String,
) -> Result<()> {
    let mut hasher = openssl::sha::Sha1::new();
    hasher.update(shared_secret);
    hasher.update(pkey_in_der);
    let hex_digest = get_hex_digest(&hasher.finish());

    let url = format!("https://sessionserver.mojang.com/session/minecraft/hasJoined?username={username}&serverId={hex_digest}");
    if reqwest::blocking::get(url)?.status().as_str() != "200" {
        return Err(EncryptionError::new("Failed to authenticate player".to_string()).into());
    }

    Ok(())
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

#[cfg(test)]
mod tests {
    use crate::encryption::get_hex_digest;

    #[test]
    fn test_get_hex_digest() {
        let mut hasher = openssl::sha::Sha1::new();

        // Notch
        hasher.update("Notch".as_bytes());
        assert_eq!(
            get_hex_digest(&hasher.finish()),
            "4ed1f46bbe04bc756bcb17c0c7ce3e4632f06a48"
        );

        // jeb_
        hasher = openssl::sha::Sha1::new();
        hasher.update("jeb_".as_bytes());
        assert_eq!(
            get_hex_digest(&hasher.finish()),
            "-7c9d5b0044c130109a5d7b5fb5c317c02b4e28c1"
        );

        // simon
        hasher = openssl::sha::Sha1::new();
        hasher.update("simon".as_bytes());
        assert_eq!(
            get_hex_digest(&hasher.finish()),
            "88e16a1019277b15d58faf0541e11910eb756f6"
        );
    }
}
