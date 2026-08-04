use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};

use axum::http::HeaderName;

/// Header name for request correlation id.
pub const REQUEST_ID_HEADER: HeaderName = HeaderName::from_static("x-request-id");

/// Creates request id middleware layers.
pub fn request_id_layers() -> (SetRequestIdLayer<MakeRequestUuid>, PropagateRequestIdLayer) {
    (
        SetRequestIdLayer::new(REQUEST_ID_HEADER.clone(), MakeRequestUuid),
        PropagateRequestIdLayer::new(REQUEST_ID_HEADER.clone()),
    )
}
