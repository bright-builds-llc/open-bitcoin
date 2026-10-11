// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_permissions.h
// - packages/bitcoin-knots/src/net_permissions.cpp
// - packages/bitcoin-knots/src/init.cpp

//! Pure BASIC peer capability and request-serving policies.

mod capability;
pub mod policy;
pub mod resource;

pub use capability::{BasicPeerServingCapability, BasicPeerServingPrerequisite};
