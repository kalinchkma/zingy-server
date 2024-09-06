/// - Web module
/// - All web related functionality gose here
/// - Handle web related work load
pub mod web;

/// - Orm module
/// - All database related functionaity gose here
/// - This modules works as a models utils
pub mod orm;

/// - Models module
/// - All business related models related functionality gose here
/// - This module depended on orm module
pub mod models;

/// - Template module
/// - All templates/view related functionality gose here
/// - all front-end functionality gose here
pub mod templates;

/// - Configuration module
/// - Server related configuration gose here
pub mod configuration;

/// - Layer module
/// - All layers related functionality gose here
pub mod layers;

/// - Log module
/// - Server logging functionality gose here
pub mod logs;

