#![deny(missing_docs)]

//! Authorization domain for LocalID.
//!
//! This crate provides authorization decisions based on
//! identities, sessions, roles, and permissions.

mod authorization;

pub use authorization::{
    AllAuthorizationPolicy, AnyAuthorizationPolicy, AuthorizationAudit, AuthorizationContext,
    AuthorizationDecision, AuthorizationDeniedReason, AuthorizationPolicy, AuthorizationPolicyExt,
    AuthorizationRequest, AuthorizationService, DefaultAuthorizationService,
    ExactPermissionMatcher, OwnedResource, OwnershipAuthorizationPolicy, Permission,
    PermissionMatcher, Resource, Role, RoleBasedAuthorizationPolicy, WildcardPermissionMatcher,
};
