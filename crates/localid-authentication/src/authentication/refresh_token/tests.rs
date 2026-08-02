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
