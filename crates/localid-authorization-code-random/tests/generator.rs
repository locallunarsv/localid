use localid_authorization_code_random::RandomAuthorizationCodeGenerator;

#[test]
fn should_generate_unique_authorization_codes() {
    let generator = RandomAuthorizationCodeGenerator::new();

    let first = generator.generate();
    let second = generator.generate();

    assert_ne!(first, second);
    assert_eq!(first.len(), 64);
    assert_eq!(second.len(), 64);
}

#[test]
fn should_hash_authorization_code_consistently() {
    let generator = RandomAuthorizationCodeGenerator::new();

    let value = "authorization-code";

    let first = generator.hash(value);
    let second = generator.hash(value);

    assert_eq!(first, second);
    assert_ne!(first, value);
}
