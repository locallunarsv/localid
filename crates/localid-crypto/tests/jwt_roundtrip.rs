use std::sync::Arc;

use localid_crypto::{CryptoTokenSigner, CryptoTokenVerifier, KeyId, KeyPair};

use localid_token::{IdTokenClaims, JwtEncoder, JwtVerifier};

#[test]
fn should_create_and_verify_real_rs256_jwt() {
    let key_pair =
        KeyPair::generate(KeyId::new("localid-key-1")).expect("key generation should succeed");

    let key_pair = Arc::new(key_pair);

    let signer = CryptoTokenSigner::new(key_pair.clone());

    let verifier = CryptoTokenVerifier::new(key_pair);

    let encoder = JwtEncoder::new(signer, "localid-key-1");

    let claims = IdTokenClaims {
        iss: "http://auth.home.arpa".to_string(),
        sub: "identity-1".to_string(),
        aud: "client-1".to_string(),
        iat: 100,
        exp: 200,
        nonce: Some("nonce-1".to_string()),
    };

    let token = encoder.encode(&claims).expect("encoding should succeed");

    let jwt_verifier = JwtVerifier::new(verifier);

    let decoded: IdTokenClaims = jwt_verifier
        .verify(&token)
        .expect("verification should succeed");

    assert_eq!(decoded.sub, "identity-1");
}
