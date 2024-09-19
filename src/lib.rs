
// - Main application startup
pub mod startup;

/// - Configuration module
/// - Server related configuration gose here
pub mod configuration;

/// - Layer module
/// - All layers related functionality gose here
pub mod layers;

/// - Log module
/// - Server logging functionality gose here
pub mod logs;

/// - Domain
pub mod domain;

/// - infrastructure
pub mod infrastructure;

/// - features
/// This is the module where we write every actual business logic
pub mod features;

/// - interface
pub mod interface;

/// - shared
pub mod shared;

