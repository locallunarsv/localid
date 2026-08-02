use super::default_service::DefaultRefreshTokenService;
use localid_refresh_token_random::RandomRefreshTokenIssuer;
use localid_storage_memory::MemoryStorage;
use localid_token_random::RandomTokenIssuer;

#[test]
fn creates_refresh_token_service() {
    let storage = MemoryStorage::new();

    let _service = DefaultRefreshTokenService::new(
        storage.clone(),
        storage.clone(),
        storage,
        RandomRefreshTokenIssuer::new(),
        RandomTokenIssuer::new(),
    );
}

#[test]
fn refresh_rotates_refresh_token() {
    // TODO:
    // 1. create identity
    // 2. create session
    // 3. issue refresh token
    // 4. save refresh token
    // 5. call refresh()
    // 6. assert old token revoked
    // 7. assert new token exists
}
