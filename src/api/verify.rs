use base64::Engine;
use rsa::pkcs8::DecodePublicKey;
use rsa::signature::Verifier;
use rsa::{pkcs1v15::VerifyingKey, RsaPublicKey};
use sha2::Sha256;

use crate::error::ChipError;

/// Verify a webhook signature from CHIP.
pub fn verify_signature(
    content: &[u8],
    signature_b64: &str,
    public_key_pem: &str,
) -> Result<bool, ChipError> {
    let signature_bytes = base64::engine::general_purpose::STANDARD
        .decode(signature_b64)
        .map_err(|_| ChipError::VerificationFailed)?;

    let public_key = RsaPublicKey::from_public_key_pem(public_key_pem)
        .map_err(|_| ChipError::VerificationFailed)?;

    let verifying_key = VerifyingKey::<Sha256>::new(public_key);
    let signature = rsa::pkcs1v15::Signature::try_from(signature_bytes.as_slice())
        .map_err(|_| ChipError::VerificationFailed)?;

    match verifying_key.verify(content, &signature) {
        Ok(()) => Ok(true),
        Err(_) => Ok(false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::Engine;
    use rsa::pkcs1v15::SigningKey;
    use rsa::pkcs8::EncodePublicKey;
    use rsa::signature::{SignatureEncoding, Signer};
    use rsa::RsaPrivateKey;
    use sha2::Sha256;

    fn generate_test_keypair() -> (RsaPrivateKey, String) {
        let mut rng = rand::thread_rng();
        let private_key = RsaPrivateKey::new(&mut rng, 2048).unwrap();
        let public_key = private_key.to_public_key();
        let pem = public_key
            .to_public_key_pem(rsa::pkcs8::LineEnding::LF)
            .unwrap();
        (private_key, pem)
    }

    fn sign_content(private_key: &RsaPrivateKey, content: &[u8]) -> String {
        let signing_key = SigningKey::<Sha256>::new(private_key.clone());
        let signature = signing_key.sign(content);
        base64::engine::general_purpose::STANDARD.encode(signature.to_bytes().as_ref())
    }

    #[test]
    fn verify_valid_signature() {
        let (private_key, public_pem) = generate_test_keypair();
        let content = b"test webhook body";
        let sig_b64 = sign_content(&private_key, content);
        let result = verify_signature(content, &sig_b64, &public_pem);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn verify_invalid_signature() {
        let (_, public_pem) = generate_test_keypair();
        let (other_key, _) = generate_test_keypair();
        let content = b"test webhook body";
        let sig_b64 = sign_content(&other_key, content);
        let result = verify_signature(content, &sig_b64, &public_pem);
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn verify_tampered_content() {
        let (private_key, public_pem) = generate_test_keypair();
        let content = b"original content";
        let sig_b64 = sign_content(&private_key, content);
        let result = verify_signature(b"tampered content", &sig_b64, &public_pem);
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn verify_invalid_base64() {
        let (_, public_pem) = generate_test_keypair();
        let result = verify_signature(b"content", "not-valid-base64!!!", &public_pem);
        assert!(result.is_err());
    }

    #[test]
    fn verify_invalid_pem() {
        let result = verify_signature(b"content", "dGVzdA==", "not a pem key");
        assert!(result.is_err());
    }
}
